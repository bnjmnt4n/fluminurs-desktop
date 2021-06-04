use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::storage::{get_app_strategy_args, Storage};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    username: Option<String>,
    password: Option<String>,
    download_location: Option<String>,

    #[serde(skip)]
    dirty: bool,
    #[serde(skip)]
    saving: bool,
}

impl Settings {
    pub fn default() -> Self {
        Settings {
            username: None,
            password: None,
            download_location: None,
            dirty: false,
            saving: false,
        }
    }

    pub fn set_login_details(&mut self, username: String, password: String) {
        self.username = Some(username);
        self.password = Some(password);
        self.dirty = true;
    }

    pub fn get_username(&self) -> &Option<String> {
        &self.username
    }

    pub fn get_password(&self) -> &Option<String> {
        &self.password
    }
}

impl Storage for Settings {
    fn path() -> PathBuf {
        use etcetera::app_strategy;
        use etcetera::app_strategy::AppStrategy;

        let app_strategy = app_strategy::choose_app_strategy(get_app_strategy_args()).unwrap();

        app_strategy.in_config_dir("settings.json")
    }

    fn get_dirty(&mut self) -> &mut bool {
        &mut self.dirty
    }

    fn get_saving(&mut self) -> &mut bool {
        &mut self.saving
    }
}
