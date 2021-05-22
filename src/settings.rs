use std::path::PathBuf;
use tokio;

use serde::{Deserialize, Serialize};
use serde_json;

use crate::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    username: Option<String>,
    password: Option<String>,
    download_location: Option<String>,
}

impl Settings {
    pub fn default() -> Self {
        Settings {
            username: None,
            password: None,
            download_location: None,
        }
    }

    pub fn set_login_details(&mut self, username: String, password: String) {
        self.username = Some(username);
        self.password = Some(password);
    }

    pub fn get_username(&self) -> &Option<String> {
        &self.username
    }

    pub fn get_password(&self) -> &Option<String> {
        &self.password
    }

    fn path() -> PathBuf {
        // TODO: change to default OS storage directory?
        let mut path = std::env::current_dir().unwrap_or(PathBuf::new());
        path.push("settings.json");

        path
    }

    pub async fn load() -> Result<Settings, Error> {
        let contents = tokio::fs::read_to_string(Self::path()).await.map_err(|_| Error {})?;

        if let Ok(settings) = serde_json::from_str::<Settings>(&contents) {
            println!("Read settings");
            Ok(settings)
        } else {
            println!("Corrupt settings, deleting file...");
            tokio::fs::remove_file(Self::path())
                .await
                .map_err(|_| Error {})?;
            Err(Error {})
        }
    }

    pub async fn save(self) -> Result<(), Error> {
        let json = serde_json::to_string_pretty(&self)
            .map_err(|_| Error {})?;

        let path = Self::path();

        tokio::fs::write(path, json).await.map_err(|_| Error {})?;

        // This is a simple way to save at most once every couple seconds
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;

        Ok(())
    }
}
