use iced::{scrollable, Column, Command, Container, Element, Length, Scrollable, Text};

use crate::message::Message;

#[derive(Debug, Clone)]
pub struct FilesState {
    scroll: scrollable::State,
}

#[derive(Debug, Clone)]
pub enum FilesMessage {}

impl FilesState {
    pub fn default() -> Self {
        FilesState {
            scroll: scrollable::State::new(),
        }
    }

    pub fn update(&mut self, _message: FilesMessage) -> Command<Message> {
        Command::none()
    }

    pub fn view(&mut self, files: &Option<Vec<String>>) -> Element<FilesMessage> {
        let files: Element<_> = if let Some(files) = files {
            files
                .iter()
                .fold(Column::new().spacing(20), |column, file| {
                    column.push(Text::new(file))
                })
                .into()
        } else {
            Text::new("No modules found").into()
        };
        let content = Column::new().max_width(800).spacing(20).push(files);

        let scrollable =
            Scrollable::new(&mut self.scroll).push(Container::new(content).width(Length::Fill));

        Container::new(scrollable)
            .height(Length::Fill)
            .center_y()
            .into()
    }
}
