use std::collections::HashMap;

use iced::{executor, Application, Clipboard, Column, Command, Element, Settings};

use fluminurs::Api;

mod api;
mod data;
mod header;
mod message;
mod module;
mod pages;
mod resource;
mod settings;
mod utils;

use crate::data::Data;
use crate::header::Header;
use crate::message::{handle_message, Message};
use crate::module::Module;
use crate::pages::{Page, Pages};
use crate::resource::ResourceType;
use crate::settings::Settings as FluminursDesktopSettings;

pub fn main() -> iced::Result {
    FluminursDesktop::run(Settings::default())
}

pub struct FluminursDesktop {
    api: Option<Api>,
    settings: FluminursDesktopSettings,
    data: Data,
    modules_map: HashMap<String, Module>,
    name: Option<String>,
    current_page: Page,
    pages: Pages,
    header: Header,
}

#[derive(Debug, Clone)]
pub struct Error;

impl FluminursDesktop {
    fn default() -> Self {
        FluminursDesktop {
            api: None,
            settings: FluminursDesktopSettings::default(),
            name: None,
            data: Data::default(),
            modules_map: HashMap::new(),
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
        (
            Self::default(),
            Command::perform(FluminursDesktopSettings::load(), Message::SettingsLoaded),
        )
    }

    fn title(&self) -> String {
        match self.current_page {
            Page::Loading => String::from("fluminurs-desktop"),
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
            Page::Loading => self.pages.loading.view().map(Message::LoadingPage),
            Page::Login => self.pages.login.view().map(Message::LoginPage),
            Page::Modules => self
                .pages
                .modules
                .view(&self.data.modules)
                .map(Message::ModulesPage),
            Page::Files => self
                .pages
                .files
                .view(&mut self.data.files, &self.modules_map)
                .map(|message| Message::ResourcesPage((ResourceType::File, message))),
            Page::Multimedia => self
                .pages
                .multimedia
                .view(&mut self.data.multimedia, &self.modules_map)
                .map(|message| Message::ResourcesPage((ResourceType::Multimedia, message))),
            Page::Weblectures => self
                .pages
                .weblectures
                .view(&mut self.data.weblectures, &self.modules_map)
                .map(|message| Message::ResourcesPage((ResourceType::Weblecture, message))),
            Page::Conferences => self
                .pages
                .conferences
                .view(&mut self.data.conferences, &self.modules_map)
                .map(|message| Message::ResourcesPage((ResourceType::Conference, message))),
        };

        if display_header {
            let header = self
                .header
                .view(&self.name, &self.current_page)
                .map(Message::Header);

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
