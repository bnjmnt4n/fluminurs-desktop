use iced::{
    button, scrollable, Button, Column, Command, Container, Element, Length, Scrollable, Text,
};

use crate::data::{DataItems, FetchStatus};
use crate::message::Message;
use crate::module::{Module, ModuleMessage};

#[derive(Debug, Clone)]
pub struct ModulesPage {
    refresh_button: button::State,
    scroll: scrollable::State,
}

impl ModulesPage {
    pub fn default() -> Self {
        Self {
            refresh_button: button::State::new(),
            scroll: scrollable::State::new(),
        }
    }

    pub fn update(&mut self, message: ModuleMessage) -> Command<Message> {
        match message {
            ModuleMessage::RefreshModules => Command::perform(async {}, Message::LoadModules),
        }
    }

    pub fn view<'a>(&'a mut self, data: &'a DataItems<Module>) -> Element<'a, ModuleMessage> {
        let modules: Element<_> = if data.items.len() > 0 {
            let col = Column::new().spacing(20);
            data.items
                .iter()
                .filter(|m| m.is_taking)
                .fold(col, |column, module| column.push(module.view()))
                .into()
        } else {
            Text::new("No modules found").into()
        };

        let refresh_button: Button<_> = match data.fetch_status {
            FetchStatus::Fetching => Button::new(&mut self.refresh_button, Text::new("Loading…")),
            _ => Button::new(&mut self.refresh_button, Text::new("Refresh"))
                .on_press(ModuleMessage::RefreshModules),
        };

        let content = Column::new().spacing(20).push(refresh_button).push(modules);

        let scrollable =
            Scrollable::new(&mut self.scroll).push(Container::new(content).width(Length::Fill));

        Container::new(scrollable).height(Length::Fill).into()
    }
}
