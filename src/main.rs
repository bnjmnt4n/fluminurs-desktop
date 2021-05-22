use std::collections::HashMap;

use iced::{executor, Application, Clipboard, Column, Command, Element, Settings};

use fluminurs::{module::Module, Api};

mod api;
mod header;
mod message;
mod pages;
mod resource;
mod utils;

use crate::header::Header;
use crate::message::{handle_message, Message};
use crate::pages::{Page, Pages};
use crate::resource::ResourceState;
use crate::resource::ResourceType;

pub fn main() -> iced::Result {
    FluminursDesktop::run(Settings::default())
}

pub struct FluminursDesktop {
    api: Option<Api>,
    modules: Option<Vec<Module>>,
    modules_map: HashMap<String, Module>,
    files: Option<Vec<ResourceState>>,
    multimedia: Option<Vec<ResourceState>>,
    weblectures: Option<Vec<ResourceState>>,
    conferences: Option<Vec<ResourceState>>,
    name: Option<String>,
    current_page: Page,
    pages: Pages,
    header: Header,
}

#[derive(Debug, Clone)]
pub struct Error;

impl FluminursDesktop {
    fn default() -> FluminursDesktop {
        FluminursDesktop {
            api: None,
            name: None,
            modules: None,
            modules_map: HashMap::new(),
            files: None,
            multimedia: None,
            weblectures: None,
            conferences: None,
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
            Page::Multimedia => String::from("Multimedia"),
            Page::Weblectures => String::from("Weblectures"),
            Page::Conferences => String::from("Conferences"),
        }
    }

    fn update(
        &mut self,
        message: Self::Message,
        _clipboard: &mut Clipboard,
    ) -> Command<Self::Message> {
        handle_message(self, message)
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
                .view(&mut self.files, &self.modules_map)
                .map(|message| Message::ResourcesPage((ResourceType::File, message))),
            Page::Multimedia => self
                .pages
                .multimedia
                .view(&mut self.multimedia, &self.modules_map)
                .map(|message| Message::ResourcesPage((ResourceType::Multimedia, message))),
            Page::Weblectures => self
                .pages
                .weblectures
                .view(&mut self.weblectures, &self.modules_map)
                .map(|message| Message::ResourcesPage((ResourceType::Weblecture, message))),
            Page::Conferences => self
                .pages
                .conferences
                .view(&mut self.conferences, &self.modules_map)
                .map(|message| Message::ResourcesPage((ResourceType::Conference, message))),
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
