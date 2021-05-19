use iced::{button, text_input, Button, Column, Command, Element, Text, TextInput};

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
            .max_width(800)
            .spacing(20)
            .push(Text::new("fluminurs-desktop").size(40))
            .push(Text::new("Username"))
            .push(username_input)
            .push(Text::new("Password"))
            .push(password_input)
            .push(Button::new(login_button, Text::new("Login")).on_press(LoginMessage::Submit));

        // if loading {
        //     content = content.push(Text::new("Loading..."));
        // }

        content.into()
    }
}
