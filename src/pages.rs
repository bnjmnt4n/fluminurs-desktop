pub mod loading;
pub mod login;
pub mod modules;
pub mod resources;
pub mod settings;

use crate::pages::loading::LoadingPage;
use crate::pages::login::LoginPage;
use crate::pages::modules::ModulesPage;
use crate::pages::resources::ResourcesPage;
use crate::pages::settings::SettingsPage;
use crate::resource::ResourceType;

#[derive(Debug, Clone, PartialEq)]
pub enum Page {
    Loading,
    Login,
    Settings,
    Modules,
    Files,
    Multimedia,
    Weblectures,
    Conferences,
}

pub struct Pages {
    pub loading: LoadingPage,
    pub login: LoginPage,
    pub settings: SettingsPage,
    pub modules: ModulesPage,
    pub files: ResourcesPage,
    pub multimedia: ResourcesPage,
    pub weblectures: ResourcesPage,
    pub conferences: ResourcesPage,
}

impl Pages {
    pub fn default() -> Self {
        Pages {
            loading: LoadingPage::default(),
            login: LoginPage::default(),
            settings: SettingsPage::default(),
            modules: ModulesPage::default(),
            files: ResourcesPage::default(ResourceType::File),
            multimedia: ResourcesPage::default(ResourceType::Multimedia),
            weblectures: ResourcesPage::default(ResourceType::Weblecture),
            conferences: ResourcesPage::default(ResourceType::Conference),
        }
    }
}
