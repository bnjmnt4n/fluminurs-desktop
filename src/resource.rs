use iced::{button, Align, Button, Element, Length, Row, Text};

use fluminurs::resource::Resource as FluminursResource;
use fluminurs::{
    conferencing::ZoomRecording,
    file::File,
    multimedia::{ExternalVideo, InternalVideo},
    weblecture::WebLectureVideo,
};

#[derive(Debug)]
pub struct ResourceState {
    pub resource: Resource,
    pub download_status: DownloadStatus,
    open_button: button::State,
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

#[derive(Debug)]
pub enum DownloadStatus {
    Downloaded,
    Downloading,
    NotDownloaded,
}

#[derive(Debug, Clone)]
pub enum ResourceMessage {
    OpenResource,
    DownloadResource,
}

impl ResourceState {
    pub fn new(resource: Resource) -> Self {
        Self {
            resource,
            download_status: DownloadStatus::NotDownloaded,
            open_button: button::State::new(),
            download_button: button::State::new(),
        }
    }

    pub fn resource_path(&self) -> String {
        match &self.resource {
            Resource::File(resource) => resource.path().display().to_string(),
            Resource::ZoomRecording(resource) => resource.path().display().to_string(),
            Resource::InternalVideo(resource) => resource.path().display().to_string(),
            Resource::ExternalVideo(resource) => resource.path().display().to_string(),
            Resource::WebLectureVideo(resource) => resource.path().display().to_string(),
        }
    }

    pub fn view(&mut self) -> Element<ResourceMessage> {
        let content = Row::new()
            .height(Length::Units(30))
            .align_items(Align::Center)
            .max_width(800)
            .spacing(20)
            .push(Text::new(self.resource_path()));

        let download_content: Element<_> = match self.download_status {
            DownloadStatus::Downloading => Text::new("Downloading...").into(),
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
