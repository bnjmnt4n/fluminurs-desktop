use std::fmt::Debug;
use std::marker::Send;
use std::path::PathBuf;
use std::pin::Pin;

use async_trait::async_trait;
use etcetera::app_strategy::AppStrategyArgs;
use futures_util::{future, Future};
use serde::{de::DeserializeOwned, Serialize};
use serde_json;

use crate::Error;

#[derive(Debug)]
pub enum StorageWrite {
    Successful,
    Retry,
    Unnecessary,
}

#[async_trait]
pub trait Storage: Debug + Clone + Serialize + DeserializeOwned + Send {
    fn path() -> PathBuf;
    fn get_dirty(&mut self) -> &mut bool;
    fn get_saving(&mut self) -> &mut bool;

    fn is_dirty(&mut self) -> bool {
        *self.get_dirty()
    }

    fn is_saving(&mut self) -> bool {
        *self.get_saving()
    }

    fn mark_saving(&mut self, saving: bool) {
        if saving {
            *self.get_dirty() = false;
            *self.get_saving() = true;
        } else {
            *self.get_saving() = false;
        }
    }

    async fn load() -> Result<Self, Error> {
        let contents = tokio::fs::read_to_string(Self::path())
            .await
            .map_err(|_| Error {})?;

        if let Ok(settings) = serde_json::from_str(&contents) {
            Ok(settings)
        } else {
            println!("Corrupt file found");
            Err(Error {})
        }
    }

    fn save(&mut self) -> Pin<Box<dyn Future<Output = Result<StorageWrite, Error>> + Send>>
    where
        Self: 'static,
    {
        let dirty = self.is_dirty();
        if dirty && !self.is_saving() {
            self.mark_saving(true);
            let to_save = self.clone();

            to_save.save_internal()
        } else if dirty {
            Box::pin(wait())
        } else {
            Box::pin(future::ready(Ok(StorageWrite::Unnecessary)))
        }
    }

    async fn save_internal(self) -> Result<StorageWrite, Error> {
        let json = serde_json::to_string(&self).map_err(|_| Error {})?;

        let path = Self::path();

        // Ensure directory to be written to exists.
        tokio::fs::create_dir_all(&path.parent().unwrap())
            .await
            .map_err(|_| Error {})?;
        tokio::fs::write(Self::path(), json)
            .await
            .map_err(|_| Error {})?;

        wait().await?;

        Ok(StorageWrite::Successful)
    }
}

async fn wait() -> Result<StorageWrite, Error> {
    // This is a simple way to save at most once every couple seconds
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;

    Ok(StorageWrite::Retry)
}

pub fn get_app_strategy_args() -> AppStrategyArgs {
    AppStrategyArgs {
        top_level_domain: "se".to_string(),
        author: "ofcr".to_string(),
        app_name: "Fluminurs Desktop".to_string(),
    }
}
