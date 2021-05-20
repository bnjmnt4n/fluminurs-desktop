use iced::{
    button, text_input, Align, Button, Column, Command, Container, Element, HorizontalAlignment,
    Length, Text, TextInput,
};

use crate::api;
use crate::message::Message;

#[derive(Debug, Clone)]
pub struct LoginState {
    username: String,
    password: String,
    username_input: text_input::State,
    password_input: text_input::State,
    login_button: button::State,
    loading: bool,
}

#[derive(Debug, Clone)]
pub enum LoginMessage {
    UsernameEdited(String),
    PasswordEdited(String),
    Submit,
}

impl LoginState {
    pub fn default() -> Self {
        LoginState {
            username: "".to_string(),
            password: "".to_string(),
            username_input: text_input::State::new(),
            password_input: text_input::State::new(),
            login_button: button::State::new(),
            loading: false,
        }
    }

    pub fn update(&mut self, message: LoginMessage) -> Command<Message> {
        match message {
            LoginMessage::UsernameEdited(username) => {
                self.username = username;
                Command::none()
            }
            LoginMessage::PasswordEdited(string) => {
                self.password = string;
                Command::none()
            }
            LoginMessage::Submit => Command::perform(
                api::login(self.username.clone(), self.password.clone()),
                Message::LoadedAPI,
            ),
        }
    }

    pub fn view(&mut self) -> Element<LoginMessage> {
        let LoginState {
            username,
            password,
            username_input,
            password_input,
            login_button,
            ..
        } = self;

        // TODO: tab navigation, error handling
        let username_input = TextInput::new(
            username_input,
            "Username",
            &username,
            LoginMessage::UsernameEdited,
        )
        .on_submit(LoginMessage::Submit)
        .padding(10);

        let password_input = TextInput::new(
            password_input,
            "Password",
            &password,
            LoginMessage::PasswordEdited,
        )
        .password()
        .on_submit(LoginMessage::Submit)
        .padding(10);

        let content = Column::new()
            .align_items(Align::Center)
            // .padding(100)
            .max_width(800)
            .spacing(10)
            .push(
                Text::new("Fluminurs-Desktop")
                    .size(40)
                    .width(Length::Fill)
                    .horizontal_alignment(HorizontalAlignment::Center),
            )
            .push(username_input.style(style::TextInput::UsernameInput))
            .push(password_input.style(style::TextInput::UsernameInput))
            .push(Button::new(login_button, Text::new("Login")).on_press(LoginMessage::Submit));

        // if loading {
        //     content = content.push(Text::new("Loading..."));
        // }

        let container = Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y();
        container.into()
        // content.into()
    }
}

mod style {
    use iced::{text_input, Background, Color};

    pub enum TextInput {
        UsernameInput,
    }

    impl text_input::StyleSheet for TextInput {
        fn active(&self) -> text_input::Style {
            text_input::Style {
                background: Background::Color(Color::WHITE),
                border_color: Color::BLACK,
                border_width: 1.0,
                border_radius: 1.0,
            }
        }
        fn focused(&self) -> text_input::Style {
            text_input::Style {
                background: Background::Color(Color {
                    r: 1.0,
                    g: 1.0,
                    b: 0.878,
                    a: 1.0,
                }),
                border_color: Color {
                    r: 1.0,
                    g: 0.0,
                    b: 0.0,
                    a: 1.0,
                },
                border_width: 1.0,
                border_radius: 1.0,
            }
        }
        fn placeholder_color(&self) -> Color {
            Color {
                r: 0.753,
                g: 0.753,
                b: 0.753,
                a: 1.0,
            }
        }
        fn value_color(&self) -> Color {
            Color::BLACK
        }
        fn selection_color(&self) -> Color {
            Color::BLACK
        }
    }
}
