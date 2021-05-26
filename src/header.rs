use iced::{button, Button, Column, Command, Element, Row, Rule, Text};

use crate::message::Message;
use crate::pages::Page;

#[derive(Debug, Clone)]
pub struct Header {
    modules_button: button::State,
    files_button: button::State,
    multimedia_button: button::State,
    weblectures_button: button::State,
    conferences_button: button::State,
    settings_button: button::State,
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
            settings_button: button::State::new(),
        }
    }

    pub fn update(&mut self, message: HeaderMessage) -> Command<Message> {
        match message {
            HeaderMessage::SwitchPage(page) => {
                Command::perform(async { page }, Message::SwitchPage)
            }
        }
    }

    pub fn view(&mut self, active_page: &Page, logged_in: bool) -> Element<HeaderMessage> {
        let content = Row::new()
            .push(create_button(
                &mut self.modules_button,
                Page::Modules,
                "Modules",
                active_page,
            ))
            .push(create_button(
                &mut self.files_button,
                Page::Files,
                "Files",
                active_page,
            ))
            .push(create_button(
                &mut self.multimedia_button,
                Page::Multimedia,
                "Multimedia",
                active_page,
            ))
            .push(create_button(
                &mut self.weblectures_button,
                Page::Weblectures,
                "Weblectures",
                active_page,
            ))
            .push(create_button(
                &mut self.conferences_button,
                Page::Conferences,
                "Conferences",
                active_page,
            ))
            .push(create_button(
                &mut self.settings_button,
                Page::Settings,
                // TODO: different color?
                if logged_in {"Settings"} else {"Settings *"},
                active_page,
            ));

        Column::new()
            .push(content)
            .push(Rule::horizontal(0).style(style::Divider::Header))
            .into()
    }
}

fn create_button<'a, 'b>(
    button_state: &'a mut button::State,
    page: Page,
    page_name: &'static str,
    active_page: &'b Page,
) -> Button<'a, HeaderMessage> {
    Button::new(button_state, Text::new(page_name))
        .style(get_button_style(active_page, &page))
        .on_press(HeaderMessage::SwitchPage(page))
}

fn get_button_style(active_page: &Page, current_module: &Page) -> style::Button {
    if active_page == current_module {
        style::Button::ActiveButton
    } else {
        style::Button::InactiveButton
    }
}

mod style {
    use iced::{button, rule, Background, Color};

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
                Button::ActiveButton => button::Style {
                    background: Option::Some(Background::Color(Color {
                        r: 0.0,
                        g: 0.0,
                        b: 0.545,
                        a: 1.0,
                    })),
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
        fn style(&self) -> rule::Style {
            rule::Style {
                fill_mode: rule::FillMode::Full,
                color: Color::BLACK,
                width: 1,
                radius: 1.0,
            }
        }
    }
}
