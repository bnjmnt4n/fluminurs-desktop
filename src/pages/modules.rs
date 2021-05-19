use iced::{scrollable, Column, Command, Container, Element, Length, Scrollable, Text};

use crate::message::Message;
use fluminurs::module::Module;

#[derive(Debug, Clone)]
pub struct ModulesState {
    scroll: scrollable::State,
}

#[derive(Debug, Clone)]
pub enum ModulesMessage {}

impl ModulesState {
    pub fn default() -> Self {
        ModulesState {
            scroll: scrollable::State::new(),
        }
    }

    pub fn update(&mut self, _message: ModulesMessage) -> Command<Message> {
        Command::none()
    }

    pub fn view(&mut self, modules: &Option<Vec<Module>>) -> Element<ModulesMessage> {
        let modules: Element<_> = if let Some(modules) = modules {
            let col = Column::new().spacing(20);
            let col = col.push(Text::new("You are taking:"));
            let col = modules
                .iter()
                .filter(|m| m.is_taking())
                .fold(col, |column, module| {
                    column.push(Text::new(format!("{} {}\n", module.code, module.name)))
                });
            let col = col.push(Text::new("You are teaching:"));
            let col = modules
                .iter()
                .filter(|m| m.is_teaching())
                .fold(col, |column, module| {
                    column.push(Text::new(format!("{} {}\n", module.code, module.name)))
                });

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
