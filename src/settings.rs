use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::storage::{get_project_dirs, Storage};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    username: Option<String>,
    password: Option<String>,
    save_username: bool,
    save_password: bool,
    download_location: Option<PathBuf>,

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
            // Default to saving username but not password
            save_username: true,
            save_password: false,
            download_location: Some(default_download_dir()),
            dirty: false,
            saving: false,
        }
    }

    pub fn set_save_username(&mut self, save_username: bool) {
        if self.save_username != save_username {
            self.save_username = save_username;
            if !save_username && self.username.is_some() {
                self.username = None;
            }
            self.dirty = true;
        }
    }

    pub fn set_save_password(&mut self, save_password: bool) {
        if self.save_password != save_password {
            self.save_password = save_password;
            if !save_password && self.password.is_some() {
                self.password = None;
            }
            self.dirty = true;
        }
    }

    pub fn set_login_details(&mut self, username: String, password: String) {
        if self.save_username {
            self.username = Some(username);
        }
        if self.save_password {
            self.password = Some(password);
        }
        self.dirty = true;
    }

    pub fn set_download_location(&mut self, download_location: PathBuf) {
        self.download_location = Some(download_location);
        self.dirty = true;
    }

    pub fn get_username(&self) -> &Option<String> {
        &self.username
    }

    pub fn get_password(&self) -> &Option<String> {
        &self.password
    }

    pub fn get_save_username(&self) -> bool {
        self.save_username
    }

    pub fn get_save_password(&self) -> bool {
        self.save_password
    }

    pub fn get_download_location(&self) -> &Option<PathBuf> {
        &self.download_location
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

pub fn default_download_dir() -> PathBuf {
    let mut download_dir: PathBuf = directories::UserDirs::new()
        .unwrap()
        .download_dir()
        .unwrap()
        .into();
    download_dir.push("LumiNUS");

    download_dir
}
