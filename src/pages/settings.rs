use iced::{
    button, scrollable, Align, Button, Checkbox, Column, Command, Container, Element, Length, Row,
    Scrollable, Text,
};

use crate::message::Message;
use crate::pages::Page;
use crate::settings::Settings;

#[derive(Debug, Clone)]
pub struct SettingsPage {
    login_button: button::State,
    scroll: scrollable::State,
}

#[derive(Debug, Clone)]
pub enum SettingsMessage {
    SwitchPage(Page),
    ToggleSaveUsername(bool),
    ToggleSavePassword(bool),
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
            SettingsMessage::ToggleSaveUsername(save_username) => {
                Command::perform(async move { save_username }, Message::ToggleSaveUsername)
            }
            SettingsMessage::ToggleSavePassword(save_password) => {
                Command::perform(async move { save_password }, Message::ToggleSavePassword)
            }
        }
    }

    pub fn view(&mut self, settings: &Settings, logged_in: bool) -> Element<SettingsMessage> {
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

        let save_username_row: Element<_> = {
            let checkbox = Checkbox::new(
                settings.get_save_username(),
                "Save username",
                SettingsMessage::ToggleSaveUsername,
            )
            .width(Length::Fill);

            Row::new()
                .spacing(20)
                .align_items(Align::Center)
                .push(checkbox)
                .into()
        };

        let save_password_row: Element<_> = {
            let checkbox = Checkbox::new(
                settings.get_save_password(),
                "Save password",
                SettingsMessage::ToggleSavePassword,
            )
            .width(Length::Fill);

            Row::new()
                .spacing(20)
                .align_items(Align::Center)
                .push(checkbox)
                .into()
        };

        let content = Column::new()
            .spacing(20)
            .push(login_element)
            .push(save_username_row)
            .push(save_password_row);

        let scrollable =
            Scrollable::new(&mut self.scroll).push(Container::new(content).width(Length::Fill));

        Container::new(scrollable).height(Length::Fill).into()
    }
}
