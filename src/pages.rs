pub mod login;
pub mod modules;
pub mod resources;

use crate::pages::login::LoginPage;
use crate::pages::modules::ModulesPage;
use crate::pages::resources::ResourcesPage;
use crate::resource::ResourceType;

#[derive(Debug, Clone)]
pub enum Page {
    Login,
    Modules,
    Files,
    Multimedia,
    Weblectures,
    Conferences,
}

pub struct Pages {
    pub login: LoginPage,
    pub modules: ModulesPage,
    pub files: ResourcesPage,
    pub multimedia: ResourcesPage,
    pub weblectures: ResourcesPage,
    pub conferences: ResourcesPage,
}

impl Pages {
    pub fn default() -> Pages {
        Pages {
            login: LoginPage::default(),
            modules: ModulesPage::default(),
            files: ResourcesPage::default(ResourceType::File),
            multimedia: ResourcesPage::default(ResourceType::Multimedia),
            weblectures: ResourcesPage::default(ResourceType::Weblecture),
            conferences: ResourcesPage::default(ResourceType::Conference),
        }
    }
}
