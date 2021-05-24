use std::fmt::Debug;
use std::marker::Send;
use std::path::PathBuf;
use std::pin::Pin;

use async_trait::async_trait;
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
    fn is_dirty(&self) -> bool;
    fn is_saving(&self) -> bool;
    fn mark_saving(&mut self);

    async fn load() -> Result<Self, Error> {
        let contents = tokio::fs::read_to_string(Self::path())
            .await
            .map_err(|_| Error {})?;

        if let Ok(settings) = serde_json::from_str(&contents) {
            println!("Read file");
            Ok(settings)
        } else {
            println!("Corrupt settings, deleting file...");
            tokio::fs::remove_file(Self::path())
                .await
                .map_err(|_| Error {})?;
            Err(Error {})
        }
    }

    fn save(&mut self) -> Pin<Box<dyn Future<Output = Result<StorageWrite, Error>> + Send>>
    where
        Self: 'static,
    {
        let dirty = self.is_dirty();
        if dirty && !self.is_saving() {
            self.mark_saving();
            let to_save = self.clone();

            to_save.save_internal()
        } else if dirty {
            Self::wait_internal()
        } else {
            Box::pin(future::ready(Ok(StorageWrite::Unnecessary)))
        }
    }

    async fn save_internal(self) -> Result<StorageWrite, Error> {
        let json = serde_json::to_string(&self).map_err(|_| Error {})?;

        tokio::fs::write(Self::path(), json)
            .await
            .map_err(|_| Error {})?;

        Ok(StorageWrite::Successful)
    }

    async fn wait_internal() -> Result<StorageWrite, Error> {
        // This is a simple way to save at most once every couple seconds
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;

        Ok(StorageWrite::Retry)
    }
}
