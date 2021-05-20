use fluminurs::{module::Module, Api};
use iced::{executor, Application, Clipboard, Column, Command, Element, Settings};

mod api;
mod header;
mod message;
mod pages;
mod resource;
mod utils;

use crate::header::Header;
use crate::message::Message;
use crate::pages::{Page, Pages};
use crate::resource::DownloadStatus;
use crate::resource::Resource;
use crate::resource::ResourceMessage;
use crate::resource::ResourceState;

pub fn main() -> iced::Result {
    FluminursDesktop::run(Settings::default())
}

pub struct FluminursDesktop {
    api: Option<Api>,
    modules: Option<Vec<Module>>,
    files: Option<Vec<ResourceState>>,
    name: Option<String>,
    current_page: Page,
    pages: Pages,
    header: Header,
}

#[derive(Debug, Clone)]
pub struct Error;

impl FluminursDesktop {
    fn default() -> Self {
        Self {
            api: None,
            name: None,
            modules: None,
            files: None,
            current_page: Page::Login,
            pages: Pages::default(),
            header: Header::default(),
        }
    }
}

impl Application for FluminursDesktop {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        (Self::default(), Command::none())
    }

    fn title(&self) -> String {
        match self.current_page {
            Page::Login => String::from("Login"),
            Page::Modules => String::from("Modules"),
            Page::Files => String::from("Files"),
        }
    }

    fn update(
        &mut self,
        message: Self::Message,
        _clipboard: &mut Clipboard,
    ) -> Command<Self::Message> {
        match message {
            Message::LoginPage(message) => self.pages.login.update(message),
            Message::ModulesPage(message) => self.pages.modules.update(message),
            Message::FilesPage(message) => self.pages.files.update(message),
            Message::Header(message) => self.header.update(message),
            Message::SwitchPage(page) => {
                self.current_page = page;
                Command::none()
            }
            Message::LoadedAPI(result) => match result {
                Ok((api, name, modules)) => {
                    self.name = Some(name);
                    self.api = Some(api.clone());
                    self.modules = Some(modules.clone());
                    self.current_page = Page::Modules;

                    // TODO: once we've logged in, fetch the other content types as well.
                    Command::perform(api::fetch_files(api, modules), Message::LoadedFiles)
                }
                Err(_) => Command::none(),
            },
            Message::LoadedFiles(result) => match result {
                Ok(files) => {
                    self.files = Some(files);
                    Command::none()
                }
                Err(_) => Command::none(),
            },
            Message::ResourceMessage((path, message)) => match message {
                ResourceMessage::DownloadResource => {
                    let api = self.api.as_ref().unwrap().clone();
                    self.files
                        .as_mut()
                        .and_then(|files| {
                            files
                                .iter_mut()
                                .find(|file| file.resource_path().eq(&path))
                                .and_then(|file| {
                                    file.download_status = DownloadStatus::Downloading;

                                    match &file.resource {
                                        Resource::File(resource) => Some(Command::perform(
                                            api::download_resource(api, resource.clone()),
                                            Message::ResourceDownloaded,
                                        )),
                                    }
                                })
                        })
                        .unwrap_or_else(|| Command::none())
                }
                ResourceMessage::OpenResource => Command::none(),
            },
            Message::ResourceDownloaded(message) => match message {
                Ok(path) => self
                    .files
                    .as_mut()
                    .and_then(|files| {
                        files
                            .iter_mut()
                            .find(|file| file.resource_path().eq(&path))
                            .and_then(|file| {
                                file.download_status = DownloadStatus::Downloaded;

                                Some(Command::none())
                            })
                    })
                    .unwrap_or_else(|| Command::none()),
                Err(_) => Command::none(),
            },
        }
    }

    fn view(&mut self) -> Element<Self::Message> {
        let display_header = match self.current_page {
            Page::Login => false,
            _ => true,
        };

        let page = match self.current_page {
            Page::Login => self.pages.login.view().map(Message::LoginPage),
            Page::Modules => self
                .pages
                .modules
                .view(&self.modules)
                .map(Message::ModulesPage),
            Page::Files => self
                .pages
                .files
                .view(&mut self.files)
                .map(Message::FilesPage),
        };

        if display_header {
            let header = self.header.view(&self.name).map(Message::Header);

            Column::new()
                .max_width(800)
                .spacing(20)
                .push(header)
                .push(page)
                .into()
        } else {
            page
        }
    }
}
