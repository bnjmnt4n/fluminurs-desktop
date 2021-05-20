pub mod files;
pub mod login;
pub mod modules;

use crate::pages::files::FilesPage;
use crate::pages::login::LoginPage;
use crate::pages::modules::ModulesPage;

#[derive(Debug, Clone)]
pub enum Page {
    Login,
    Modules,
    Files,
}

pub struct Pages {
    pub login: LoginPage,
    pub modules: ModulesPage,
    pub files: FilesPage,
}

impl Pages {
    pub fn default() -> Self {
        Self {
            login: LoginPage::default(),
            modules: ModulesPage::default(),
            files: FilesPage::default(),
        }
    }
}
