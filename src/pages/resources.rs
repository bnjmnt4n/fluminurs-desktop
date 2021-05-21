use iced::{scrollable, Column, Command, Container, Element, Length, Scrollable, Text};

use crate::message::Message;
use crate::resource::{ResourceMessage, ResourceState, ResourceType};

#[derive(Debug, Clone)]
pub struct ResourcesPage {
    resource_type: ResourceType,
    scroll: scrollable::State,
}

#[derive(Debug, Clone)]
pub enum ResourcesMessage {
    ResourceMessage(ResourceType, String, ResourceMessage),
}

impl ResourcesPage {
    pub fn default(resource_type: ResourceType) -> ResourcesPage {
        ResourcesPage {
            resource_type,
            scroll: scrollable::State::new(),
        }
    }

    pub fn update(message: ResourcesMessage) -> Command<Message> {
        match message {
            ResourcesMessage::ResourceMessage(resource_type, path, message) => Command::perform(
                async move { (resource_type, path, message) },
                Message::ResourceMessage,
            ),
        }
    }

    pub fn view<'a>(
        &'a mut self,
        files: &'a mut Option<Vec<ResourceState>>,
    ) -> Element<'a, ResourcesMessage> {
        let files: Element<_> = if let Some(ref mut files) = files {
            files
                .iter_mut()
                .fold(Column::new().spacing(20), |column, file| {
                    // TODO: figure out Rust move semantics here
                    let resource_type = self.resource_type.clone();
                    let resource_path = file.resource_path().clone();
                    column.push(file.view().map(move |message| {
                        ResourcesMessage::ResourceMessage(
                            resource_type.clone(),
                            resource_path.clone(),
                            message,
                        )
                    }))
                })
                .into()
        } else {
            let type_text = match self.resource_type {
                ResourceType::File => "No files found",
                ResourceType::Multimedia => "No multimedia found",
                ResourceType::Weblecture => "No weblectures found",
                ResourceType::Conference => "No conferences found",
            };

            Text::new(type_text).into()
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
