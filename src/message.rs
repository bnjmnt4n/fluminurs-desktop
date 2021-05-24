use std::path::PathBuf;
use std::time::SystemTime;

use iced::Command;

use fluminurs::Api;

use crate::api;
use crate::data::{DataItems, FetchStatus};
use crate::header::HeaderMessage;
use crate::module::{Module, ModuleMessage};
use crate::pages::loading::LoadingMessage;
use crate::pages::login::LoginMessage;
use crate::pages::resources::{ResourcesMessage, ResourcesPage};
use crate::pages::Page;
use crate::resource::{DownloadStatus, ResourceMessage, ResourceState, ResourceType};
use crate::settings::Settings;
use crate::storage::{Storage, StorageWrite};
use crate::utils::{construct_modules_map, merge_modules, merge_resources};
use crate::Error;
use crate::FluminursDesktop;

#[derive(Debug)]
pub enum Message {
    LoadingPage(LoadingMessage),
    LoginPage(LoginMessage),
    ModulesPage(ModuleMessage),
    ResourcesPage((ResourceType, ResourcesMessage)),
    Header(HeaderMessage),
    SwitchPage(Page),

    SettingsLoaded(Result<Settings, Error>),
    SettingsSaved(Result<StorageWrite, Error>),
    DataSaved(Result<StorageWrite, Error>),
    LoadedAPI(Result<(Api, String, String, String, DataItems<Module>), Error>),
    LoadModules(()),
    LoadedModules(Result<DataItems<Module>, Error>),
    LoadResources(ResourceType),
    LoadedResources((ResourceType, Result<DataItems<ResourceState>, Error>)),
    ResourceMessage((ResourceType, String, PathBuf, ResourceMessage)),
    ResourceDownloaded((ResourceType, String, PathBuf, Result<PathBuf, Error>)),
    OpenFileResult(Result<std::process::ExitStatus, std::io::Error>),
}

pub fn handle_message(state: &mut FluminursDesktop, message: Message) -> Command<Message> {
    match message {
        // For messages that have to deal with local state, pass them back to
        // be handled by each individual page/component.
        Message::LoadingPage(message) => state.pages.loading.update(message),
        Message::LoginPage(message) => state.pages.login.update(message),
        Message::ModulesPage(message) => state.pages.modules.update(message),
        Message::ResourcesPage((resource_type, message)) => {
            get_resources_page(state, resource_type).update(message)
        }
        Message::Header(message) => state.header.update(message),

        // Switch the current active page.
        Message::SwitchPage(page) => {
            state.current_page = page;
            Command::none()
        }

        Message::SettingsLoaded(message) => match message {
            Ok(settings) => {
                state.settings = settings;
                state.current_page = Page::Login;

                if let Some(username) = state.settings.get_username() {
                    state
                        .pages
                        .login
                        .update(LoginMessage::UsernameEdited(username.to_string()));
                }
                if let Some(password) = state.settings.get_password() {
                    state
                        .pages
                        .login
                        .update(LoginMessage::PasswordEdited(password.to_string()));
                }

                Command::none()
            }
            // TODO
            Err(_) => Command::none(),
        },
        Message::SettingsSaved(message) => match message {
            Ok(StorageWrite::Successful) => {
                println!("Saved settings");
                state.settings.mark_saving(false);
                Command::none()
            }
            Ok(StorageWrite::Retry) => {
                println!("Retrying settings save");
                Command::perform(state.settings.save(), Message::SettingsSaved)
            }
            Ok(StorageWrite::Unnecessary) => {
                println!("No need to write settings");
                Command::none()
            }
            // TODO
            Err(_) => {
                println!("Failed to save settings");
                Command::none()
            }
        },

        Message::DataSaved(message) => match message {
            Ok(StorageWrite::Successful) => {
                println!("Saved data");
                state.data.mark_saving(false);
                Command::none()
            }
            Ok(StorageWrite::Retry) => {
                println!("Retrying data save");
                Command::perform(state.data.save(), Message::DataSaved)
            }
            Ok(StorageWrite::Unnecessary) => {
                println!("No need to write data");
                Command::none()
            }
            // TODO
            Err(_) => {
                println!("Failed to save data");
                Command::none()
            }
        },
        // After we've successfully logged in, fetch all resources.
        Message::LoadedAPI(result) => match result {
            Ok((api, username, password, name, modules)) => {
                state.name = Some(name);
                state.api = Some(api);
                merge_modules(&mut state.data.modules, modules);
                state.data.mark_dirty();
                state.modules_map = construct_modules_map(&state.data.modules.items);
                state.current_page = Page::Modules;

                state.settings.set_login_details(username, password);

                Command::batch(vec![
                    Command::perform(state.data.save(), Message::DataSaved),
                    Command::perform(state.settings.save(), Message::SettingsSaved),
                    Command::perform(async { ResourceType::File }, Message::LoadResources),
                    Command::perform(async { ResourceType::Multimedia }, Message::LoadResources),
                    Command::perform(async { ResourceType::Weblecture }, Message::LoadResources),
                    Command::perform(async { ResourceType::Conference }, Message::LoadResources),
                ])
            }
            Err(_) => state.pages.login.update(LoginMessage::Failed),
        },

        // Load modules.
        Message::LoadModules(()) => {
            match state.api.as_ref().cloned() {
                Some(api) => {
                    state.data.modules.fetch_status = FetchStatus::Fetching;
                    let last_updated = SystemTime::now();

                    Command::perform(
                        async move {
                            // TODO: don't hardcode
                            api::load_modules(&api, Some("2020".to_owned()), last_updated).await
                        },
                        Message::LoadedModules,
                    )
                }
                // TODO: refresh API?
                None => Command::none(),
            }
        }

        // Update loaded modules.
        Message::LoadedModules(result) => match result {
            Ok(modules) => {
                merge_modules(&mut state.data.modules, modules);
                state.data.mark_dirty();

                Command::perform(state.data.save(), Message::DataSaved)
            }
            // TODO
            Err(_) => {
                state.data.modules.fetch_status = FetchStatus::Error;

                Command::none()
            }
        },

        // Load resources.
        Message::LoadResources(resource_type) => {
            match state.api.as_ref().cloned() {
                Some(api) => {
                    if state.data.modules.items.len() > 0 {
                        let modules = state
                            .data
                            .modules
                            .items
                            .clone()
                            .into_iter()
                            .filter_map(|module| module.internal_module)
                            .collect();
                        let last_updated = SystemTime::now();
                        let fetch_status = get_fetch_status(state, resource_type);
                        *fetch_status = FetchStatus::Fetching;

                        Command::perform(
                            async move {
                                let result = match resource_type {
                                    ResourceType::File => {
                                        api::load_modules_files(api, modules, last_updated).await
                                    }
                                    ResourceType::Multimedia => {
                                        api::load_modules_multimedia(api, modules, last_updated)
                                            .await
                                    }
                                    ResourceType::Weblecture => {
                                        api::load_modules_weblectures(api, modules, last_updated)
                                            .await
                                    }
                                    ResourceType::Conference => {
                                        api::load_modules_conferences(api, modules, last_updated)
                                            .await
                                    }
                                };

                                (resource_type, result)
                            },
                            Message::LoadedResources,
                        )
                    } else {
                        // No modules
                        Command::none()
                    }
                }
                // TODO: refresh API?
                None => Command::none(),
            }
        }

        // Update loaded resources.
        Message::LoadedResources((resource_type, result)) => match result {
            Ok(resources) => {
                let curr_resources = match resource_type {
                    ResourceType::File => &mut state.data.files,
                    ResourceType::Multimedia => &mut state.data.multimedia,
                    ResourceType::Weblecture => &mut state.data.weblectures,
                    ResourceType::Conference => &mut state.data.conferences,
                };

                merge_resources(curr_resources, resources);
                state.data.mark_dirty();

                Command::perform(state.data.save(), Message::DataSaved)
            }
            // TODO
            Err(_) => {
                let fetch_status = get_fetch_status(state, resource_type);
                *fetch_status = FetchStatus::Error;

                Command::none()
            }
        },

        // Perform a specific action for a resource.
        Message::ResourceMessage((resource_type, module_id, path, message)) => match message {
            ResourceMessage::DownloadResource => {
                // Note: we clone the API here so it can be passed across threads to be used
                // in an iced Command.
                let api = state.api.as_ref().cloned();
                match api {
                    Some(api) => {
                        let modules_map = state.modules_map.clone();
                        let resources = get_resources_items(state, resource_type);
                        resources
                            .iter_mut()
                            .find(|file| file.path.eq(&path) && file.module_id.eq(&module_id))
                            .map(|file| {
                                file.download_status = DownloadStatus::Downloading;
                                match &file.resource {
                                    Some(resource) => {
                                        let resource = resource.clone();
                                        let path = file.path.clone();
                                        let download_path = file.local_resource_path(&modules_map);

                                        Command::perform(
                                            async move {
                                                let result = api::download_resource(
                                                    api,
                                                    resource,
                                                    download_path,
                                                    path.clone(),
                                                )
                                                .await;
                                                (resource_type, module_id, path, result)
                                            },
                                            Message::ResourceDownloaded,
                                        )
                                    }
                                    None => Command::none(),
                                }
                            })
                            .unwrap_or_else(|| Command::none())
                    }
                    // TODO: if there's no API, try to re-authenticate?
                    None => Command::none(),
                }
            }

            // Open downloaded file.
            // TODO: doesn't work well on Linux.
            ResourceMessage::OpenResource => {
                Command::perform(async move { open::that(path) }, Message::OpenFileResult)
            }
        },

        Message::OpenFileResult(result) => {
            match result {
                Ok(result) => println!("Opened file successfully: {}", result),
                Err(err) => println!("Error opening file: {}", err),
            }
            Command::none()
        }

        // Update resource download status, either marking as complete or error.
        Message::ResourceDownloaded((resource_type, module_id, path, message)) => {
            let resources = get_resources_items(state, resource_type);
            resources
                .iter_mut()
                .find(|file| file.path.eq(&path) && file.module_id.eq(&module_id))
                .map(|file| {
                    match message {
                        Ok(_path) => {
                            // TODO: handle renames based on the new path returned.
                            file.download_status = DownloadStatus::Downloaded;
                        }
                        // TODO: handle error
                        Err(_) => {}
                    };

                    Command::none()
                })
                .unwrap_or_else(|| Command::none())
        }
    }
}

fn get_resources_items<'a>(
    state: &'a mut FluminursDesktop,
    resource_type: ResourceType,
) -> &'a mut Vec<ResourceState> {
    match resource_type {
        ResourceType::File => state.data.files.items.as_mut(),
        ResourceType::Multimedia => state.data.multimedia.items.as_mut(),
        ResourceType::Weblecture => state.data.weblectures.items.as_mut(),
        ResourceType::Conference => state.data.conferences.items.as_mut(),
    }
}

fn get_resources_page(
    state: &mut FluminursDesktop,
    resource_type: ResourceType,
) -> &mut ResourcesPage {
    match resource_type {
        ResourceType::File => &mut state.pages.files,
        ResourceType::Multimedia => &mut state.pages.multimedia,
        ResourceType::Weblecture => &mut state.pages.weblectures,
        ResourceType::Conference => &mut state.pages.conferences,
    }
}

fn get_fetch_status(state: &mut FluminursDesktop, resource_type: ResourceType) -> &mut FetchStatus {
    match resource_type {
        ResourceType::File => &mut state.data.files.fetch_status,
        ResourceType::Multimedia => &mut state.data.multimedia.fetch_status,
        ResourceType::Weblecture => &mut state.data.weblectures.fetch_status,
        ResourceType::Conference => &mut state.data.conferences.fetch_status,
    }
}
