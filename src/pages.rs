pub mod files;
pub mod login;
pub mod modules;

use crate::pages::files::FilesState;
use crate::pages::login::LoginState;
use crate::pages::modules::ModulesState;

#[derive(Debug, Clone)]
pub enum Page {
    Login,
    Modules,
    Files,
}

pub struct PageStates {
    pub login: LoginState,
    pub modules: ModulesState,
    pub files: FilesState,
}

impl PageStates {
    pub fn default() -> Self {
        PageStates {
            login: LoginState::default(),
            modules: ModulesState::default(),
            files: FilesState::default(),
        }
    }
}
