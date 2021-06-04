use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::storage::{get_project_dirs, Storage};

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
        let mut path: PathBuf = get_project_dirs().config_dir().into();
        path.push("settings.json");

        path
    }

    fn get_dirty(&mut self) -> &mut bool {
        &mut self.dirty
    }

    fn get_saving(&mut self) -> &mut bool {
        &mut self.saving
    }
}
