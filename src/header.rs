use iced::{button, Button, Element, Row, Text};

use crate::pages::Page;

#[derive(Debug, Clone)]
pub struct HeaderState {
    modules_button: button::State,
    files_button: button::State,
}

impl HeaderState {
    pub fn default() -> Self {
        HeaderState {
            modules_button: button::State::new(),
            files_button: button::State::new(),
        }
    }

    pub fn view(&mut self, name: &Option<String>) -> Element<Page> {
        let content = Row::new()
            .max_width(800)
            .spacing(20)
            .push(
                Button::new(&mut self.modules_button, Text::new("Modules")).on_press(Page::Modules),
            )
            .push(Button::new(&mut self.files_button, Text::new("Files")).on_press(Page::Files));

        let content = if let Some(name) = name {
            content.push(Text::new(name))
        } else {
            content
        };

        content.into()
    }
}
