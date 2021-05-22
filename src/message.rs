use std::path::PathBuf;

use iced::Command;
extern crate open;

use fluminurs::module::Module;
use fluminurs::Api;

use crate::api;
use crate::header::HeaderMessage;
use crate::pages::login::LoginMessage;
use crate::pages::modules::ModulesMessage;
use crate::pages::resources::{ResourcesMessage, ResourcesPage};
use crate::pages::Page;
use crate::resource::{DownloadStatus, ResourceMessage, ResourceState, ResourceType};
use crate::settings::Settings;
use crate::Error;
use crate::FluminursDesktop;

#[derive(Debug)]
pub enum Message {
    LoginPage(LoginMessage),
    ModulesPage(ModulesMessage),
    ResourcesPage((ResourceType, ResourcesMessage)),
    Header(HeaderMessage),
    SwitchPage(Page),

    SettingsLoaded(Result<Settings, Error>),
    SettingsSaved(Result<(), Error>),
    LoadedAPI(Result<(Api, String, String, String, Vec<Module>), Error>),
    LoadResources(ResourceType),
    LoadedResources((ResourceType, Result<Vec<ResourceState>, Error>)),
    ResourceMessage((ResourceType, String, PathBuf, ResourceMessage)),
    ResourceDownloaded((ResourceType, String, PathBuf, Result<PathBuf, Error>)),
    OpenFileResult(Result<std::process::ExitStatus, std::io::Error>),
}

pub fn handle_message(state: &mut FluminursDesktop, message: Message) -> Command<Message> {
    match message {
        // For messages that have to deal with local state, pass them back to
        // be handled by each individual page/component.
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

                if let Some(username) = state.settings.get_username() {
                    state.pages.login.update(LoginMessage::UsernameEdited(username.to_string()));
                }
                if let Some(password) = state.settings.get_password() {
                    state.pages.login.update(LoginMessage::PasswordEdited(password.to_string()));
                }

                Command::none()
            }
            // TODO
            Err(_) => Command::none()
        }
        Message::SettingsSaved(message) => match message {
            Ok(()) => {
                println!("Saved settings");
                Command::none()
            }
            // TODO
            Err(_) => {
                println!("Failed to save settings");
                Command::none()
            }
        }

        // After we've successfully logged in, fetch all resources.
        Message::LoadedAPI(result) => match result {
            Ok((api, username, password, name, modules)) => {
                state.name = Some(name);
                state.api = Some(api);
                state.modules = Some(modules.clone());
                // TODO: avoid cloning everything
                state.modules_map = modules
                    .into_iter()
                    .map(|item| (item.id.to_string(), item))
                    .collect();
                state.current_page = Page::Modules;

                state.settings.set_login_details(username, password);

                Command::batch(vec![
                    Command::perform(state.settings.clone().save(), Message::SettingsSaved),
                    Command::perform(async { ResourceType::File }, Message::LoadResources),
                    Command::perform(async { ResourceType::Multimedia }, Message::LoadResources),
                    Command::perform(async { ResourceType::Weblecture }, Message::LoadResources),
                    Command::perform(async { ResourceType::Conference }, Message::LoadResources),
                ])
            }
            Err(_) => state.pages.login.update(LoginMessage::Failed),
        },

        // Load resources.
        Message::LoadResources(resource_type) => {
            match state.api.as_ref().cloned() {
                Some(api) => match state.modules.as_ref().cloned() {
                    Some(modules) => Command::batch(vec![
                        Command::perform(
                            async move { (resource_type, ResourcesMessage::RefreshInProgress) },
                            Message::ResourcesPage,
                        ),
                        Command::perform(
                            async move {
                                let result = match resource_type {
                                    ResourceType::File => {
                                        api::load_modules_files(api, modules).await
                                    }
                                    ResourceType::Multimedia => {
                                        api::load_modules_multimedia(api, modules).await
                                    }
                                    ResourceType::Weblecture => {
                                        api::load_modules_weblectures(api, modules).await
                                    }
                                    ResourceType::Conference => {
                                        api::load_modules_conferences(api, modules).await
                                    }
                                };

                                (resource_type, result)
                            },
                            Message::LoadedResources,
                        ),
                    ]),
                    // TODO: refetch modules?
                    None => Command::none(),
                },
                // TODO: refresh API?
                None => Command::none(),
            }
        }

        // Update loaded resources.
        Message::LoadedResources((resource_type, result)) => match result {
            Ok(resources) => {
                match resource_type {
                    ResourceType::File => state.files = Some(resources),
                    ResourceType::Multimedia => state.multimedia = Some(resources),
                    ResourceType::Weblecture => state.weblectures = Some(resources),
                    ResourceType::Conference => state.conferences = Some(resources),
                };

                Command::perform(
                    async move { (resource_type, ResourcesMessage::RefreshSuccessful) },
                    Message::ResourcesPage,
                )
            }
            // TODO
            Err(_) => Command::perform(
                async move { (resource_type, ResourcesMessage::RefreshFailed) },
                Message::ResourcesPage,
            ),
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
                        let resources = get_resources(state, resource_type);
                        resources
                            .and_then(|files| {
                                files
                                    .iter_mut()
                                    .find(|file| {
                                        file.path.eq(&path) && file.module_id.eq(&module_id)
                                    })
                                    .map(|file| {
                                        file.download_status = DownloadStatus::Downloading;
                                        let resource = file.resource.clone();
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
                                    })
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
            let resources = get_resources(state, resource_type);
            resources
                .and_then(|files| {
                    files
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
                })
                .unwrap_or_else(|| Command::none())
        }
    }
}

fn get_resources<'a>(
    state: &'a mut FluminursDesktop,
    resource_type: ResourceType,
) -> Option<&'a mut Vec<ResourceState>> {
    match resource_type {
        ResourceType::File => state.files.as_mut(),
        ResourceType::Multimedia => state.multimedia.as_mut(),
        ResourceType::Weblecture => state.weblectures.as_mut(),
        ResourceType::Conference => state.conferences.as_mut(),
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
