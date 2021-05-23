use iced::{button, Button, Command, Element, Row, Text, Rule, Column, Space, Length, VerticalAlignment};

use crate::message::Message;
use crate::pages::Page;

#[derive(Debug, Clone)]
pub struct Header {
    modules_button: button::State,
    files_button: button::State,
    multimedia_button: button::State,
    weblectures_button: button::State,
    conferences_button: button::State,
    active_module: Page,
}

#[derive(Debug, Clone)]
pub enum HeaderMessage {
    SwitchPage(Page),
}

impl Header {
    pub fn default() -> Self {
        Header {
            modules_button: button::State::new(),
            files_button: button::State::new(),
            multimedia_button: button::State::new(),
            weblectures_button: button::State::new(),
            conferences_button: button::State::new(),
            active_module: Page::Modules,
        }
    }

    pub fn update(&mut self, message: HeaderMessage) -> Command<Message> {
        match message {
            HeaderMessage::SwitchPage(page) => {
                self.active_module = page.clone();
                Command::perform(async { page }, Message::SwitchPage)
            }
        }
    }

    pub fn view(&mut self, name: &Option<String>) -> Element<HeaderMessage> {
        // TODO: different styles for active module
        let content = Row::new()
            .max_width(800)
            .push(
                Button::new(&mut self.modules_button, Text::new("Modules"))
                    .on_press(HeaderMessage::SwitchPage(Page::Modules)).style(match self.active_module { Page::Modules => style::Button::ActiveButton, _ => style::Button::InactiveButton }),
            )
            .push(
                Button::new(&mut self.files_button, Text::new("Files"))
                    .on_press(HeaderMessage::SwitchPage(Page::Files)).style(match self.active_module { Page::Files => style::Button::ActiveButton, _ => style::Button::InactiveButton }))
            .push(
                Button::new(&mut self.multimedia_button, Text::new("Multimedia"))
                    .on_press(HeaderMessage::SwitchPage(Page::Multimedia)).style(match self.active_module{ Page::Multimedia => style::Button::ActiveButton, _ => style::Button::InactiveButton }),
            )
            .push(
                Button::new(&mut self.weblectures_button, Text::new("Weblectures"))
                    .on_press(HeaderMessage::SwitchPage(Page::Weblectures)).style(match self.active_module{ Page::Weblectures => style::Button::ActiveButton, _ => style::Button::InactiveButton }),
            )
            .push(
                Button::new(&mut self.conferences_button, Text::new("Conferences"))
                    .on_press(HeaderMessage::SwitchPage(Page::Conferences)).style(match self.active_module {Page::Conferences => style::Button::ActiveButton, _ => style::Button::InactiveButton}),
            );

        let content = if let Some(name) = name {
            content.push(Space::with_width(Length::FillPortion(7))).push(Text::new(name).vertical_alignment(VerticalAlignment::Bottom).height(Length::Units(25)))
        } else {
            content
        };

        let content2 = Column::new().push(content).push(Rule::horizontal(0).style(style::Divider::Header));

        content2.into()
    }
}

mod style {
    use iced::{ button, rule, Background, Color };

    pub enum Button {
        ActiveButton,
        InactiveButton,
    }

    pub enum Divider {
        Header,
    }

    impl button::StyleSheet for Button {
        fn active(&self) -> button::Style {
            match self {
                Button::ActiveButton =>  button::Style {
                    background: Option::Some(Background::Color( Color {r: 0.0, g: 0.0, b: 0.545, a: 1.0})),
                    border_radius: 0.0,
                    border_width: 0.0,
                    text_color: Color::WHITE,
                    ..button::Style::default()
                },
                Button::InactiveButton => button::Style::default(),
            }

        }
    }

    impl rule::StyleSheet for Divider {
        fn style(&self) -> rule::Style{
            rule::Style {
                fill_mode: rule::FillMode::Full,
                color: Color::BLACK,
                width: 1,
                radius: 1.0,
            }
        }
    }
}
