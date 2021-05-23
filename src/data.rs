use std::path::PathBuf;
use tokio;

use serde::{Deserialize, Serialize};
use serde_json;

use crate::module::Module;
use crate::resource::ResourceState;
use crate::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Data {
    pub modules: Option<Vec<Module>>,
    pub files: Option<Vec<ResourceState>>,
    pub multimedia: Option<Vec<ResourceState>>,
    pub weblectures: Option<Vec<ResourceState>>,
    pub conferences: Option<Vec<ResourceState>>,
}

impl Data {
    pub fn default() -> Self {
        Data {
            modules: None,
            files: None,
            multimedia: None,
            weblectures: None,
            conferences: None,
        }
    }

    fn path() -> PathBuf {
        // TODO: change to default OS storage directory?
        let mut path = std::env::current_dir().unwrap_or(PathBuf::new());
        path.push("data.json");

        path
    }

    pub async fn load() -> Result<Data, Error> {
        let contents = tokio::fs::read_to_string(Self::path())
            .await
            .map_err(|_| Error {})?;

        if let Ok(settings) = serde_json::from_str::<Data>(&contents) {
            println!("Read data");
            Ok(settings)
        } else {
            println!("Corrupt data, deleting file...");
            tokio::fs::remove_file(Self::path())
                .await
                .map_err(|_| Error {})?;
            Err(Error {})
        }
    }

    pub async fn save(self) -> Result<(), Error> {
        let json = serde_json::to_string_pretty(&self).map_err(|_| Error {})?;

        let path = Self::path();

        tokio::fs::write(path, json).await.map_err(|_| Error {})?;

        // This is a simple way to save at most once every couple seconds
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;

        Ok(())
    }
}
