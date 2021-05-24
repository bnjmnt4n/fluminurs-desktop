use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::storage::Storage;

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
        // TODO: change to default OS storage directory?
        let mut path = std::env::current_dir().unwrap_or(PathBuf::new());
        path.push("settings.json");

        path
    }

    fn is_dirty(&self) -> bool {
        self.dirty
    }

    fn is_saving(&self) -> bool {
        self.saving
    }

    fn mark_saving(&mut self) {
        self.dirty = false;
        self.saving = true;
    }
}
