use std::collections::HashMap;
use std::path::PathBuf;

use iced::{
    button, scrollable, Button, Column, Command, Container, Element, Length, Scrollable, Text,
};

use chrono::offset::Utc;
use chrono::DateTime;

use crate::data::{DataItems, FetchStatus};
use crate::message::Message;
use crate::module::Module;
use crate::resource::{ResourceMessage, ResourceState, ResourceType};

#[derive(Debug, Clone)]
pub struct ResourcesPage {
    resource_type: ResourceType,
    refresh_button: button::State,
    scroll: scrollable::State,
}

#[derive(Debug, Clone)]
pub enum ResourcesMessage {
    Refresh,
    ResourceMessage(ResourceType, String, PathBuf, ResourceMessage),
}

impl ResourcesPage {
    pub fn default(resource_type: ResourceType) -> Self {
        ResourcesPage {
            resource_type,
            refresh_button: button::State::new(),
            scroll: scrollable::State::new(),
        }
    }

    pub fn update(&mut self, message: ResourcesMessage) -> Command<Message> {
        match message {
            ResourcesMessage::Refresh => {
                let resource_type = self.resource_type;
                Command::perform(async move { resource_type }, Message::LoadResources)
            }
            ResourcesMessage::ResourceMessage(resource_type, module_id, path, message) => {
                Command::perform(
                    async move { (resource_type, module_id, path, message) },
                    Message::ResourceMessage,
                )
            }
        }
    }

    pub fn view<'a>(
        &'a mut self,
        data: &'a mut DataItems<ResourceState>,
        modules_map: &'a HashMap<String, Module>,
    ) -> Element<'a, ResourcesMessage> {
        let files: Element<_> = if data.items.len() > 0 {
            data.items
                .iter_mut()
                .fold(Column::new().spacing(20), |column, file| {
                    // TODO: figure out Rust move semantics here
                    let resource_type = self.resource_type;
                    let resource_module_id = file.module_id.clone();
                    let resource_path = file.path.clone();
                    column.push(file.view(modules_map, resource_type).map(move |message| {
                        ResourcesMessage::ResourceMessage(
                            resource_type,
                            resource_module_id.clone(),
                            resource_path.clone(),
                            message,
                        )
                    }))
                })
                .into()
        } else {
            let type_text = match data.fetch_status {
                FetchStatus::Idle => match self.resource_type {
                    ResourceType::File => "No files found",
                    ResourceType::Multimedia => "No multimedia found",
                    ResourceType::Weblecture => "No weblectures found",
                    ResourceType::Conference => "No conferences found",
                },
                FetchStatus::Fetching => "Loading…",
                FetchStatus::Error => "Failed to fetch resources",
            };

            Text::new(type_text).into()
        };

        let refresh_button: Button<_> = match data.fetch_status {
            FetchStatus::Fetching => Button::new(&mut self.refresh_button, Text::new("Loading…")),
            _ => Button::new(&mut self.refresh_button, Text::new("Refresh"))
                .on_press(ResourcesMessage::Refresh),
        };

        let last_updated: DateTime<Utc> = data.last_updated.into();
        let last_updated = last_updated.format("%d/%m/%Y %T");
        let last_updated = Text::new(format!("Last updated at {}", last_updated));

        let content = Column::new()
            .spacing(20)
            .push(refresh_button)
            .push(last_updated)
            .push(files);

        let scrollable =
            Scrollable::new(&mut self.scroll).push(Container::new(content).width(Length::Fill));

        Container::new(scrollable).height(Length::Fill).into()
    }
}
