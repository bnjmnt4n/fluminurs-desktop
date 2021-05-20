use iced::{button, Button, Command, Element, Row, Text};

use crate::message::Message;
use crate::pages::Page;

#[derive(Debug, Clone)]
pub struct Header {
    modules_button: button::State,
    files_button: button::State,
}

#[derive(Debug, Clone)]
pub enum HeaderMessage {
    SwitchPage(Page),
}

impl Header {
    pub fn default() -> Self {
        Self {
            modules_button: button::State::new(),
            files_button: button::State::new(),
        }
    }

    pub fn update(&mut self, message: HeaderMessage) -> Command<Message> {
        match message {
            HeaderMessage::SwitchPage(page) => {
                Command::perform(async { page }, Message::SwitchPage)
            }
        }
    }

    pub fn view(&mut self, name: &Option<String>) -> Element<HeaderMessage> {
        // TODO: different styles for active module
        let content = Row::new()
            .max_width(800)
            .spacing(20)
            .push(
                Button::new(&mut self.modules_button, Text::new("Modules"))
                    .on_press(HeaderMessage::SwitchPage(Page::Modules)),
            )
            .push(
                Button::new(&mut self.files_button, Text::new("Files"))
                    .on_press(HeaderMessage::SwitchPage(Page::Files)),
            );

        let content = if let Some(name) = name {
            content.push(Text::new(name))
        } else {
            content
        };

        content.into()
    }
}
