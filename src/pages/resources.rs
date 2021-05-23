use std::collections::HashMap;
use std::path::PathBuf;

use iced::{
    button, scrollable, Button, Column, Command, Container, Element, Length, Scrollable, Text,
};

use crate::message::Message;
use crate::module::Module;
use crate::resource::{ResourceMessage, ResourceState, ResourceType};

#[derive(Debug, Clone)]
pub struct ResourcesPage {
    resource_type: ResourceType,
    loading_state: ResourcesLoadingState,
    refresh_button: button::State,
    scroll: scrollable::State,
}

#[derive(Debug, Clone)]
pub enum ResourcesMessage {
    Refresh,
    RefreshInProgress,
    RefreshSuccessful,
    RefreshFailed,
    ResourceMessage(ResourceType, String, PathBuf, ResourceMessage),
}

#[derive(Debug, Clone)]
pub enum ResourcesLoadingState {
    Loading,
    Idle,
    Error,
}

impl ResourcesPage {
    pub fn default(resource_type: ResourceType) -> Self {
        ResourcesPage {
            resource_type,
            loading_state: ResourcesLoadingState::Idle,
            refresh_button: button::State::new(),
            scroll: scrollable::State::new(),
        }
    }

    pub fn update(&mut self, message: ResourcesMessage) -> Command<Message> {
        match message {
            ResourcesMessage::Refresh => {
                self.loading_state = ResourcesLoadingState::Loading;
                let resource_type = self.resource_type;
                Command::perform(async move { resource_type }, Message::LoadResources)
            }
            ResourcesMessage::RefreshInProgress => {
                self.loading_state = ResourcesLoadingState::Loading;
                Command::none()
            }
            ResourcesMessage::RefreshSuccessful => {
                self.loading_state = ResourcesLoadingState::Idle;
                Command::none()
            }
            ResourcesMessage::RefreshFailed => {
                self.loading_state = ResourcesLoadingState::Error;
                Command::none()
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
        files: &'a mut Option<Vec<ResourceState>>,
        modules_map: &'a HashMap<String, Module>,
    ) -> Element<'a, ResourcesMessage> {
        let files: Element<_> = if let Some(ref mut files) = files {
            files
                .iter_mut()
                .fold(Column::new().spacing(20), |column, file| {
                    // TODO: figure out Rust move semantics here
                    let resource_type = self.resource_type;
                    let resource_module_id = file.module_id.clone();
                    let resource_path = file.path.clone();
                    column.push(file.view(modules_map).map(move |message| {
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
            let type_text = match self.loading_state {
                ResourcesLoadingState::Idle => match self.resource_type {
                    ResourceType::File => "No files found",
                    ResourceType::Multimedia => "No multimedia found",
                    ResourceType::Weblecture => "No weblectures found",
                    ResourceType::Conference => "No conferences found",
                },
                ResourcesLoadingState::Loading => "Loading…",
                ResourcesLoadingState::Error => "Failed to fetch resources",
            };

            Text::new(type_text).into()
        };

        let refresh_button: Button<_> = match self.loading_state {
            ResourcesLoadingState::Loading => {
                Button::new(&mut self.refresh_button, Text::new("Loading…"))
            }
            _ => Button::new(&mut self.refresh_button, Text::new("Refresh"))
                .on_press(ResourcesMessage::Refresh),
        };

        let content = Column::new()
            .max_width(800)
            .spacing(20)
            .push(refresh_button)
            .push(files);

        let scrollable =
            Scrollable::new(&mut self.scroll).push(Container::new(content).width(Length::Fill));

        Container::new(scrollable)
            .height(Length::Fill)
            .center_y()
            .into()
    }
}
