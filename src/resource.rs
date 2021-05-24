use std::collections::HashMap;
use std::default::Default;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use iced::{button, Align, Button, Element, Length, Row, Text};

use serde::{Deserialize, Serialize};

use fluminurs::resource::Resource as FluminursResource;
use fluminurs::{
    conferencing::ZoomRecording,
    file::File,
    multimedia::{ExternalVideo, InternalVideo},
    weblecture::WebLectureVideo,
};

use crate::module::Module;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceState {
    pub module_id: String,
    pub path: PathBuf,
    pub last_updated: SystemTime,
    pub download_path: Option<PathBuf>,
    pub download_time: Option<SystemTime>,

    #[serde(skip)]
    pub resource: Option<Resource>,
    #[serde(skip)]
    pub download_status: DownloadStatus,
    #[serde(skip)]
    open_button: button::State,
    #[serde(skip)]
    download_button: button::State,
}

#[derive(Debug, Copy, Clone)]
pub enum ResourceType {
    File,
    Multimedia,
    Weblecture,
    Conference,
}

#[derive(Debug, Clone)]
pub enum Resource {
    File(File),
    InternalVideo(InternalVideo),
    ExternalVideo(ExternalVideo),
    WebLectureVideo(WebLectureVideo),
    ZoomRecording(ZoomRecording),
}

#[derive(Debug, Clone)]
pub enum DownloadStatus {
    Downloaded,
    Downloading,
    NotDownloaded,
}

impl Default for DownloadStatus {
    fn default() -> Self {
        DownloadStatus::NotDownloaded
    }
}

#[derive(Debug, Clone)]
pub enum ResourceMessage {
    OpenResource,
    DownloadResource,
}

impl ResourceState {
    pub fn empty() -> Self {
        ResourceState {
            module_id: "".to_string(),
            path: PathBuf::new(),
            last_updated: SystemTime::UNIX_EPOCH,
            download_path: None,
            download_time: None,
            resource: None,
            download_status: DownloadStatus::NotDownloaded,
            open_button: button::State::new(),
            download_button: button::State::new(),
        }
    }

    pub fn new(resource: Resource, module_id: String) -> Self {
        ResourceState {
            module_id,
            path: get_resource_path(&resource),
            last_updated: get_resource_last_updated(&resource),
            download_path: None,
            download_time: None,

            resource: Some(resource),
            download_status: DownloadStatus::NotDownloaded,
            open_button: button::State::new(),
            download_button: button::State::new(),
        }
    }

    pub fn local_resource_path(&self, modules_map: &HashMap<String, Module>) -> PathBuf {
        Path::new(match modules_map.get(&self.module_id) {
            Some(module) => module.code.as_ref(),
            None => "Unknown",
        })
        .join(Path::new(match &self.resource {
            Some(Resource::File(_)) => "Files",
            Some(Resource::InternalVideo(_)) => "Multimedia",
            Some(Resource::ExternalVideo(_)) => "Multimedia",
            Some(Resource::WebLectureVideo(_)) => "Weblectures",
            Some(Resource::ZoomRecording(_)) => "Conferences",
            None => "None",
        }))
        .join(self.path.clone())
    }

    pub fn view(&mut self, modules_map: &HashMap<String, Module>) -> Element<ResourceMessage> {
        let content = Row::new()
            .height(Length::Units(30))
            .align_items(Align::Center)
            .max_width(800)
            .spacing(20)
            .push(Text::new(
                self.local_resource_path(modules_map).display().to_string(),
            ));

        let download_content: Element<_> = match self.download_status {
            DownloadStatus::Downloading => Text::new("Downloading…").into(),
            DownloadStatus::NotDownloaded => {
                Button::new(&mut self.download_button, Text::new("Download"))
                    .on_press(ResourceMessage::DownloadResource)
                    .into()
            }
            DownloadStatus::Downloaded => Button::new(&mut self.open_button, Text::new("Open"))
                .on_press(ResourceMessage::OpenResource)
                .into(),
        };

        content.push(download_content).into()
    }
}

fn get_resource_path(resource: &Resource) -> PathBuf {
    match &resource {
        Resource::File(resource) => resource.path().to_path_buf(),
        Resource::ZoomRecording(resource) => resource.path().to_path_buf(),
        Resource::InternalVideo(resource) => resource.path().to_path_buf(),
        Resource::ExternalVideo(resource) => resource.path().to_path_buf(),
        Resource::WebLectureVideo(resource) => resource.path().to_path_buf(),
    }
}

fn get_resource_last_updated(resource: &Resource) -> SystemTime {
    match &resource {
        Resource::File(resource) => resource.last_updated(),
        Resource::ZoomRecording(resource) => resource.last_updated(),
        Resource::InternalVideo(resource) => resource.last_updated(),
        Resource::ExternalVideo(resource) => resource.last_updated(),
        Resource::WebLectureVideo(resource) => resource.last_updated(),
    }
}
