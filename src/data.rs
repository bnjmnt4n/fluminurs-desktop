use std::path::PathBuf;
use std::time::SystemTime;

use serde::{Deserialize, Serialize};

use crate::module::Module;
use crate::resource::ResourceState;
use crate::storage::{get_project_dirs, Storage};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Data {
    pub modules: DataItems<Module>,
    pub files: DataItems<ResourceState>,
    pub multimedia: DataItems<ResourceState>,
    pub weblectures: DataItems<ResourceState>,
    pub conferences: DataItems<ResourceState>,

    #[serde(skip)]
    dirty: bool,
    #[serde(skip)]
    saving: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataItems<T> {
    pub last_updated: SystemTime,
    pub items: Vec<T>,

    #[serde(skip)]
    pub fetch_status: FetchStatus,
}

impl<T> Default for DataItems<T> {
    fn default() -> Self {
        DataItems {
            last_updated: SystemTime::UNIX_EPOCH,
            items: vec![],
            fetch_status: FetchStatus::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FetchStatus {
    Idle,
    Fetching,
    Error,
}

impl Default for FetchStatus {
    fn default() -> Self {
        FetchStatus::Idle
    }
}

impl Data {
    pub fn default() -> Self {
        Data {
            modules: DataItems::default(),
            files: DataItems::default(),
            multimedia: DataItems::default(),
            weblectures: DataItems::default(),
            conferences: DataItems::default(),
            dirty: false,
            saving: false,
        }
    }

    pub fn mark_dirty(&mut self) {
        self.dirty = true;
    }
}

impl Storage for Data {
    fn path() -> PathBuf {
        let mut path: PathBuf = get_project_dirs().data_dir().into();
        path.push("data.json");

        path
    }

    fn get_dirty(&mut self) -> &mut bool {
        &mut self.dirty
    }

    fn get_saving(&mut self) -> &mut bool {
        &mut self.saving
    }
}
