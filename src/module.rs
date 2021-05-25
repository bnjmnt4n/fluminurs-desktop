use std::time::SystemTime;

use serde::{Deserialize, Serialize};

use iced::{Element, Length, Row, Text};

use fluminurs::module::Module as FluminursModule;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Module {
    pub id: String,
    pub code: String,
    pub name: String,
    pub term: String,
    pub is_taking: bool,
    pub is_teaching: bool,
    pub last_updated: SystemTime,

    #[serde(skip)]
    pub internal_module: Option<FluminursModule>,
}

#[derive(Debug, Clone)]
pub enum ModuleMessage {
    RefreshModules,
}

impl Module {
    pub fn empty() -> Self {
        Module {
            id: "".to_string(),
            code: "".to_string(),
            name: "".to_string(),
            term: "".to_string(),
            is_taking: false,
            is_teaching: false,
            last_updated: SystemTime::UNIX_EPOCH,
            internal_module: None,
        }
    }

    pub fn new(module: FluminursModule, last_updated: SystemTime) -> Self {
        Module {
            id: module.id.to_string(),
            code: module.code.to_string(),
            name: module.name.to_string(),
            term: module.term.to_string(),
            is_taking: module.is_taking(),
            is_teaching: module.is_teaching(),
            last_updated,
            internal_module: Some(module),
        }
    }

    pub fn view(&self) -> Element<ModuleMessage> {
        Row::new()
            .height(Length::Units(30))
            .max_width(800)
            .spacing(20)
            .push(Text::new(format!("{} {}", self.code, self.name)))
            .into()
    }
}
