use iced::{scrollable, Column, Command, Container, Element, Length, Scrollable, Text};

use crate::message::Message;
use crate::resource::{ResourceMessage, ResourceState};

#[derive(Debug, Clone)]
pub struct FilesPage {
    scroll: scrollable::State,
}

#[derive(Debug, Clone)]
pub enum FilesMessage {
    FileMessage(String, ResourceMessage),
}

impl FilesPage {
    pub fn default() -> Self {
        Self {
            scroll: scrollable::State::new(),
        }
    }

    pub fn update(&mut self, message: FilesMessage) -> Command<Message> {
        match message {
            FilesMessage::FileMessage(path, message) => {
                Command::perform(async { (path, message) }, Message::ResourceMessage)
            }
        }
    }

    pub fn view<'a>(
        &'a mut self,
        files: &'a mut Option<Vec<ResourceState>>,
    ) -> Element<'a, FilesMessage> {
        let files: Element<_> = if let Some(ref mut files) = files {
            files
                .iter_mut()
                .fold(Column::new().spacing(20), |column, file| {
                    let resource_path = file.resource_path().clone();
                    column.push(file.view().map(move |message| {
                        FilesMessage::FileMessage(resource_path.clone(), message)
                    }))
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
