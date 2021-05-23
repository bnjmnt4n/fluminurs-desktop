use iced::{Command, Container, Element, Length, Text};

use crate::message::Message;

#[derive(Debug, Clone)]
pub struct LoadingPage {}

#[derive(Debug, Clone)]
pub enum LoadingMessage {}

#[derive(Debug, Clone)]
pub enum LoadingState {}

impl LoadingPage {
    pub fn default() -> Self {
        LoadingPage {}
    }

    pub fn update(&mut self, _message: LoadingMessage) -> Command<Message> {
        Command::none()
    }

    pub fn view(&mut self) -> Element<LoadingMessage> {
        let loading_message = Text::new("Loading…");

        Container::new(loading_message)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
