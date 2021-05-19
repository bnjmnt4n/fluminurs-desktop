use fluminurs::{module::Module, Api};
use iced::{executor, Application, Clipboard, Column, Command, Element, Settings};

mod api;
mod header;
mod message;
mod pages;

use crate::header::HeaderState;
use crate::message::Message;
use crate::pages::{Page, PageStates};

pub fn main() -> iced::Result {
    FluminursDesktop::run(Settings::default())
}

pub struct FluminursDesktop {
    api: Option<Api>,
    modules: Option<Vec<Module>>,
    files: Option<Vec<String>>,
    name: Option<String>,
    page: Page,
    page_states: PageStates,
    header_state: HeaderState,
}

#[derive(Debug, Clone)]
pub struct Error;

impl FluminursDesktop {
    fn default() -> Self {
        FluminursDesktop {
            api: None,
            name: None,
            modules: None,
            files: None,
            page: Page::Login,
            page_states: PageStates::default(),
            header_state: HeaderState::default(),
        }
    }
}

impl Application for FluminursDesktop {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        (FluminursDesktop::default(), Command::none())
    }

    fn title(&self) -> String {
        match self.page {
            Page::Login => String::from("Login - fluminurs-desktop"),
            Page::Modules => String::from("Modules - fluminurs-desktop"),
            Page::Files => String::from("Files - fluminurs-desktop"),
        }
    }

    fn update(
        &mut self,
        message: Self::Message,
        _clipboard: &mut Clipboard,
    ) -> Command<Self::Message> {
        match message {
            Message::LoginMessage(message) => self.page_states.login.update(message),
            Message::ModulesMessage(message) => self.page_states.modules.update(message),
            Message::FilesMessage(message) => self.page_states.files.update(message),
            Message::SwitchPage(page) => {
                self.page = page;
                Command::none()
            }
            Message::LoadedAPI(result) => match result {
                Ok((api, name, modules, files)) => {
                    self.api = Some(api);
                    self.modules = Some(modules);
                    self.files = Some(files);
                    self.name = Some(name);
                    self.page = Page::Modules;
                    Command::none()
                }
                Err(_) => Command::none(),
            },
        }
    }

    fn view(&mut self) -> Element<Self::Message> {
        let page = match self.page {
            Page::Login => self.page_states.login.view().map(Message::LoginMessage),
            Page::Modules => self
                .page_states
                .modules
                .view(&self.modules)
                .map(Message::ModulesMessage),
            Page::Files => self
                .page_states
                .files
                .view(&self.files)
                .map(Message::FilesMessage),
        };

        match self.page {
            Page::Login => page,
            _ => {
                let header = self.header_state.view(&self.name).map(Message::SwitchPage);

                Column::new()
                    .max_width(800)
                    .spacing(20)
                    .push(header)
                    .push(page)
                    .into()
            }
        }
    }
}
