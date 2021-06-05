use iced::{
    button, scrollable, Align, Button, Column, Command, Container, Element, Length, Row,
    Scrollable, Text,
};

use crate::message::Message;
use crate::pages::Page;

#[derive(Debug, Clone)]
pub struct SettingsPage {
    login_button: button::State,
    scroll: scrollable::State,
}

#[derive(Debug, Clone)]
pub enum SettingsMessage {
    SwitchPage(Page),
}

impl SettingsPage {
    pub fn default() -> Self {
        SettingsPage {
            login_button: button::State::new(),
            scroll: scrollable::State::new(),
        }
    }

    pub fn update(&mut self, message: SettingsMessage) -> Command<Message> {
        match message {
            SettingsMessage::SwitchPage(page) => {
                Command::perform(async { page }, Message::SwitchPage)
            }
        }
    }

    pub fn view(&mut self, logged_in: bool) -> Element<SettingsMessage> {
        let login_element: Element<_> = if logged_in {
            Text::new("Logged in").into()
        } else {
            Row::new()
                .height(Length::Units(30))
                .align_items(Align::Center)
                .spacing(20)
                .push(Text::new("Not logged in"))
                .push(
                    Button::new(&mut self.login_button, Text::new("Login"))
                        .on_press(SettingsMessage::SwitchPage(Page::Login)),
                )
                .into()
        };

        let content = Column::new().spacing(20).push(login_element);

        let scrollable =
            Scrollable::new(&mut self.scroll).push(Container::new(content).width(Length::Fill));

        Container::new(scrollable).height(Length::Fill).into()
    }
}
