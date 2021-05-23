use iced::{scrollable, Column, Command, Container, Element, Length, Scrollable, Text};

use crate::message::Message;
use crate::module::{Module, ModuleMessage};

#[derive(Debug, Clone)]
pub struct ModulesPage {
    scroll: scrollable::State,
}

impl ModulesPage {
    pub fn default() -> Self {
        Self {
            scroll: scrollable::State::new(),
        }
    }

    pub fn update(&mut self, _message: ModuleMessage) -> Command<Message> {
        Command::none()
    }

    pub fn view<'a>(&'a mut self, modules: &'a Option<Vec<Module>>) -> Element<'a, ModuleMessage> {
        let modules: Element<_> = if let Some(modules) = modules {
            let col = Column::new().spacing(20);
            let col = col.push(Text::new("You are taking:"));
            let col = modules
                .iter()
                .filter(|m| m.is_taking)
                .fold(col, |column, module| column.push(module.view()));
            let col = col.push(Text::new("You are teaching:"));
            let col = modules
                .iter()
                .filter(|m| m.is_teaching)
                .fold(col, |column, module| column.push(module.view()));

            col.into()
        } else {
            Text::new("No modules found").into()
        };

        let content = Column::new().max_width(800).spacing(20).push(modules);

        let scrollable =
            Scrollable::new(&mut self.scroll).push(Container::new(content).width(Length::Fill));

        Container::new(scrollable)
            .height(Length::Fill)
            .center_y()
            .into()
    }
}
