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
use crate::Error;
use crate::FluminursDesktop;

#[derive(Debug)]
pub enum Message {
    LoginPage(LoginMessage),
    ModulesPage(ModulesMessage),
    ResourcesPage(ResourcesMessage),
    Header(HeaderMessage),
    SwitchPage(Page),

    LoadedAPI(Result<(Api, String, Vec<Module>), Error>),
    LoadedResources(Result<(ResourceType, Vec<ResourceState>), Error>),
    ResourceMessage((ResourceType, String, ResourceMessage)),
    ResourceDownloaded(Result<(ResourceType, String), Error>),
}

pub fn handle_message(state: &mut FluminursDesktop, message: Message) -> Command<Message> {
    match message {
        // For messages that have to deal with local state, pass them back to
        // be handled by each individual page/component.
        Message::LoginPage(message) => state.pages.login.update(message),
        Message::ModulesPage(message) => state.pages.modules.update(message),
        Message::ResourcesPage(message) => ResourcesPage::update(message),
        Message::Header(message) => state.header.update(message),

        // Switch the current active page.
        Message::SwitchPage(page) => {
            state.current_page = page;
            Command::none()
        }

        // After we've successfully logged in, fetch all resources.
        Message::LoadedAPI(result) => match result {
            Ok((api, name, modules)) => {
                let commands = Command::batch(vec![
                    Command::perform(
                        api::fetch_files(api.clone(), modules.clone()),
                        Message::LoadedResources,
                    ),
                    Command::perform(
                        api::fetch_multimedia(api.clone(), modules.clone()),
                        Message::LoadedResources,
                    ),
                    Command::perform(
                        api::fetch_weblectures(api.clone(), modules.clone()),
                        Message::LoadedResources,
                    ),
                    Command::perform(
                        api::fetch_conferences(api.clone(), modules.clone()),
                        Message::LoadedResources,
                    ),
                ]);

                state.name = Some(name);
                state.api = Some(api);
                state.modules = Some(modules);
                state.current_page = Page::Modules;

                commands
            }
            Err(_) => Command::none(),
        },

        // Update loaded resources.
        Message::LoadedResources(result) => match result {
            Ok((resource_type, resources)) => {
                match resource_type {
                    ResourceType::File => state.files = Some(resources),
                    ResourceType::Multimedia => state.multimedia = Some(resources),
                    ResourceType::Weblecture => state.weblectures = Some(resources),
                    ResourceType::Conference => state.conferences = Some(resources),
                };

                Command::none()
            }
            Err(_) => Command::none(),
        },

        // Perform a specific action for a resource.
        Message::ResourceMessage((resource_type, path, message)) => match message {
            ResourceMessage::DownloadResource => {
                // Note: we clone the API here so it can be passed accross threads to be used
                // in an iced Command.
                let api = state.api.as_ref().cloned();
                match api {
                    Some(api) => {
                        let resources = get_resources(state, resource_type);
                        resources
                            .and_then(|files| {
                                files
                                    .iter_mut()
                                    .find(|file| file.resource_path().eq(&path))
                                    .map(|file| {
                                        file.download_status = DownloadStatus::Downloading;

                                        Command::perform(
                                            api::download_resource(
                                                api,
                                                file.resource.clone(),
                                                resource_type,
                                            ),
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
            // TODO: open downloaded file
            ResourceMessage::OpenResource => {
                let opened = open::that(path);
                match opened {
                    Ok(..) => println!("Opened Successfully"),
                    Err(..) => println!("Failed to open"),
                }
                Command::none()
            }
        },

        // Update resource download status, either marking as complete or error.
        Message::ResourceDownloaded(message) => match message {
            Ok((resource_type, path)) => {
                let resources = get_resources(state, resource_type);
                resources
                    .and_then(|files| {
                        files
                            .iter_mut()
                            .find(|file| file.resource_path().eq(&path))
                            .map(|file| {
                                file.download_status = DownloadStatus::Downloaded;
                                Command::none()
                            })
                    })
                    .unwrap_or_else(|| Command::none())
            }
            // TODO: handle error
            Err(_) => Command::none(),
        },
    }
}

fn get_resources(
    state: &mut FluminursDesktop,
    resource_type: ResourceType,
) -> Option<&mut Vec<ResourceState>> {
    match resource_type {
        ResourceType::File => state.files.as_mut(),
        ResourceType::Multimedia => state.multimedia.as_mut(),
        ResourceType::Weblecture => state.weblectures.as_mut(),
        ResourceType::Conference => state.conferences.as_mut(),
    }
}
