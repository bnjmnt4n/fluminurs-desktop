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
    download_location_button: button::State,
    is_changing_download_location: bool,
    scroll: scrollable::State,
}

#[derive(Debug, Clone)]
pub enum SettingsMessage {
    SwitchPage(Page),
    ToggleSaveUsername(bool),
    ToggleSavePassword(bool),
    ChangeDownloadLocation,
    DownloadLocationChanged,
}

impl SettingsPage {
    pub fn default() -> Self {
        SettingsPage {
            login_button: button::State::new(),
            download_location_button: button::State::new(),
            is_changing_download_location: false,
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
            SettingsMessage::ChangeDownloadLocation => {
                self.is_changing_download_location = true;
                Command::perform(async {}, Message::ChangeDownloadLocation)
            }
            SettingsMessage::DownloadLocationChanged => {
                self.is_changing_download_location = false;
                Command::none()
            }
        }
    }

    pub fn view(&mut self, settings: &mut Settings, logged_in: bool) -> Element<SettingsMessage> {
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

        let download_location_details: Element<SettingsMessage> = {
            let download_location: Element<_> =
                if let Some(download_location) = settings.get_download_location() {
                    Text::new(download_location.to_string_lossy()).into()
                } else {
                    Text::new("No download location specified").into()
                };

            let download_location_button =
                Button::new(&mut self.download_location_button, Text::new("Change…"));

            // Disable change button if dialog is currently open/opening.
            // TODO: opening the dialog seems to take a bit of time on my Linux system.
            let download_location_button = if self.is_changing_download_location {
                download_location_button
            } else {
                download_location_button.on_press(SettingsMessage::ChangeDownloadLocation)
            };

            Row::new()
                .height(Length::Units(30))
                .align_items(Align::Center)
                .spacing(20)
                .push(Text::new("Download location"))
                .push(download_location)
                .push(download_location_button)
                .into()
        };

        let content = Column::new()
            .spacing(20)
            .push(login_element)
            .push(save_username_row)
            .push(save_password_row)
            .push(download_location_details)
            .push(Text::new("Note: changing the download location will not shift files from the old location to the new one."));

        let scrollable =
            Scrollable::new(&mut self.scroll).push(Container::new(content).width(Length::Fill));

        Container::new(scrollable).height(Length::Fill).into()
    }
}
