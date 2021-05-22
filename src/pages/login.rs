use iced::{
    button, text_input, Align, Button, Color, Column, Command, Container, Element,
    HorizontalAlignment, Length, Text, TextInput,
};

use crate::api;
use crate::message::Message;
use crate::utils::clean_username;

#[derive(Debug, Clone)]
pub struct LoginPage {
    username: String,
    password: String,
    username_input: text_input::State,
    password_input: text_input::State,
    login_button: button::State,
    login_state: LoginState,
}

#[derive(Debug, Clone)]
pub enum LoginMessage {
    UsernameEdited(String),
    PasswordEdited(String),
    Submit,
    Failed,
}

#[derive(Debug, Clone)]
pub enum LoginState {
    Initial,
    SigningIn,
    Error,
}

impl LoginPage {
    pub fn default() -> Self {
        Self {
            username: "".to_string(),
            password: "".to_string(),
            username_input: text_input::State::new(),
            password_input: text_input::State::new(),
            login_button: button::State::new(),
            login_state: LoginState::Initial,
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
            LoginMessage::Submit => {
                self.login_state = LoginState::SigningIn;
                Command::perform(
                    api::login(clean_username(&self.username), self.password.clone()),
                    Message::LoadedAPI,
                )
            }
            LoginMessage::Failed => {
                self.login_state = LoginState::Error;
                Command::none()
            }
        }
    }

    pub fn view(&mut self) -> Element<LoginMessage> {
        let LoginPage {
            username,
            password,
            username_input,
            password_input,
            login_button,
            login_state,
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

        let error_message = match *login_state {
            LoginState::Error => "Username or password is incorrect",
            _ => "",
        };

        let button_text = match *login_state {
            LoginState::SigningIn => "Signing in…",
            _ => "Sign in",
        };
        let login_button = Button::new(login_button, Text::new(button_text));

        // Disable login button if signing in is in progress
        let login_button = match *login_state {
            LoginState::SigningIn => login_button,
            _ => login_button.on_press(LoginMessage::Submit),
        };

        let content = Column::new()
            .align_items(Align::Center)
            .max_width(400)
            .spacing(10)
            .push(
                Text::new("fluminurs-desktop")
                    .size(40)
                    .width(Length::Fill)
                    .horizontal_alignment(HorizontalAlignment::Center),
            )
            .push(Text::new(error_message).color(Color {
                r: 1.0,
                g: 0.0,
                b: 0.0,
                a: 1.0,
            }))
            .push(username_input.style(style::TextInput::UsernameInput))
            .push(password_input.style(style::TextInput::UsernameInput))
            .push(login_button);

        let container = Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y();

        container.into()
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
            Color::WHITE
        }
    }
}
