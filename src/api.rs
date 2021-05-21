use futures_util::future;
use std::ffi::OsStr;
use std::ffi::OsString;
use std::path::Path;

use fluminurs::module::Module;
use fluminurs::resource::{OverwriteMode, OverwriteResult, Resource as FluminursResource};
use fluminurs::Api;
use fluminurs::{
    conferencing::ZoomRecording,
    file::File,
    multimedia::{ExternalVideo, InternalVideo},
    weblecture::WebLectureVideo,
};

use crate::resource::{Resource, ResourceState, ResourceType};
use crate::Error;

pub async fn login(
    username: String,
    password: String,
) -> Result<(Api, String, Vec<Module>), Error> {
    let api = Api::with_login(&username, &password)
        .await
        .map_err(|_| Error {})?
        // TODO: custom ffmpeg location
        .with_ffmpeg("ffmpeg".to_owned());

    let name = api.name().await.map_err(|_| Error {})?;

    let modules = api
        .modules(Some("2020".to_owned()))
        .await
        .map_err(|_| Error {})?;

    Ok((api, name, modules))
}

pub async fn fetch_files(
    api: Api,
    modules: Vec<Module>,
) -> Result<(ResourceType, Vec<ResourceState>), Error> {
    let files = load_modules_files(&api, &modules)
        .await?
        .into_iter()
        .map(|file| ResourceState::new(Resource::File(file)))
        .collect();

    Ok((ResourceType::File, files))
}

pub async fn fetch_multimedia(
    api: Api,
    modules: Vec<Module>,
) -> Result<(ResourceType, Vec<ResourceState>), Error> {
    let (internal_videos, external_videos) = load_modules_multimedia(&api, &modules).await?;

    let mut videos: Vec<ResourceState> = internal_videos
        .into_iter()
        .map(|file| ResourceState::new(Resource::InternalVideo(file)))
        .collect();

    let mut external_videos: Vec<ResourceState> = external_videos
        .into_iter()
        .map(|file| ResourceState::new(Resource::ExternalVideo(file)))
        .collect();

    videos.append(&mut external_videos);

    Ok((ResourceType::Multimedia, videos))
}

pub async fn fetch_weblectures(
    api: Api,
    modules: Vec<Module>,
) -> Result<(ResourceType, Vec<ResourceState>), Error> {
    let weblectures = load_modules_weblectures(&api, &modules)
        .await?
        .into_iter()
        .map(|file| ResourceState::new(Resource::WebLectureVideo(file)))
        .collect();

    Ok((ResourceType::Weblecture, weblectures))
}

pub async fn fetch_conferences(
    api: Api,
    modules: Vec<Module>,
) -> Result<(ResourceType, Vec<ResourceState>), Error> {
    let conferences = load_modules_conferences(&api, &modules)
        .await?
        .into_iter()
        .map(|file| ResourceState::new(Resource::ZoomRecording(file)))
        .collect();

    Ok((ResourceType::Conference, conferences))
}

// TODO: reduce code duplication with fluminurs

async fn load_modules_files(api: &Api, modules: &[Module]) -> Result<Vec<File>, Error> {
    let include_uploadable_folders = true;

    let root_dirs = modules
        .iter()
        .filter(|module| module.has_access())
        .map(|module| {
            (
                module.workbin_root(|code| Path::new(code).join(Path::new("Files"))),
                module.is_teaching(),
            )
        })
        .collect::<Vec<_>>();

    let (files, errors) = future::join_all(root_dirs.into_iter().map(|(root_dir, _)| async move {
        root_dir
            .load(api, include_uploadable_folders)
            .await
            .map(|mut files| {
                // to avoid duplicate files from being corrupted,
                // we append the id to duplicate files
                fluminurs::file::sort_and_make_all_paths_unique(&mut files);
                files
            })
    }))
    .await
    .into_iter()
    .fold((vec![], vec![]), move |(mut ok, mut err), res| {
        match res {
            Ok(mut dir) => {
                ok.append(&mut dir);
            }
            Err(e) => {
                err.push(e);
            }
        }
        (ok, err)
    });
    for e in errors {
        println!("Failed loading module files: {}", e);
    }
    Ok(files)
}

async fn load_modules_multimedia(
    api: &Api,
    modules: &[Module],
) -> Result<(Vec<InternalVideo>, Vec<ExternalVideo>), Error> {
    let multimedias = modules
        .iter()
        .filter(|module| module.has_access())
        .map(|module| module.multimedia_root(|code| Path::new(code).join(Path::new("Multimedia"))))
        .collect::<Vec<_>>();

    let (internal_videos, external_videos, errors) = future::join_all(
        multimedias
            .into_iter()
            .map(|multimedia| multimedia.load(api)),
    )
    .await
    .into_iter()
    .fold(
        (vec![], vec![], vec![]),
        move |(mut internal_videos, mut external_videos, mut err), res| {
            match res {
                Ok((mut iv, mut ev)) => {
                    internal_videos.append(&mut iv);
                    external_videos.append(&mut ev);
                }
                Err(e) => {
                    err.push(e);
                }
            }
            (internal_videos, external_videos, err)
        },
    );

    for e in errors {
        println!("Failed loading module multimedia: {}", e);
    }
    Ok((internal_videos, external_videos))
}

async fn load_modules_weblectures(
    api: &Api,
    modules: &[Module],
) -> Result<Vec<WebLectureVideo>, Error> {
    let weblectures = modules
        .iter()
        .filter(|module| module.has_access())
        .map(|module| {
            module.weblecture_root(|code| Path::new(code).join(Path::new("Web Lectures")))
        })
        .collect::<Vec<_>>();

    let (files, errors) = future::join_all(
        weblectures
            .into_iter()
            .map(|weblecture| weblecture.load(api)),
    )
    .await
    .into_iter()
    .fold((vec![], vec![]), move |(mut ok, mut err), res| {
        match res {
            Ok(mut dir) => {
                ok.append(&mut dir);
            }
            Err(e) => {
                err.push(e);
            }
        }
        (ok, err)
    });

    for e in errors {
        println!("Failed loading module web lecture: {}", e);
    }
    Ok(files)
}

async fn load_modules_conferences(
    api: &Api,
    modules: &[Module],
) -> Result<Vec<ZoomRecording>, Error> {
    let conferences = modules
        .iter()
        .filter(|module| module.has_access())
        .map(|module| {
            module.conferencing_root(|code| Path::new(code).join(Path::new("Conferences")))
        })
        .collect::<Vec<_>>();

    let (zoom_recordings, errors) = future::join_all(
        conferences
            .into_iter()
            .map(|conference| conference.load(api)),
    )
    .await
    .into_iter()
    .fold((vec![], vec![]), move |(mut ok, mut err), res| {
        match res {
            Ok(mut dir) => {
                ok.append(&mut dir);
            }
            Err(e) => {
                err.push(e);
            }
        }
        (ok, err)
    });

    for e in errors {
        println!("Failed loading module conferences: {}", e);
    }
    Ok(zoom_recordings)
}

fn make_temp_file_name(name: &OsStr) -> OsString {
    let prepend = OsStr::new("~!");
    let mut res = OsString::with_capacity(prepend.len() + name.len());
    res.push(prepend);
    res.push(name);
    res
}

pub async fn download_resource(
    api: Api,
    resource: Resource,
    resource_type: ResourceType,
    // overwrite_mode: OverwriteMode,
) -> Result<(ResourceType, String), Error> {
    match resource {
        Resource::File(resource) => download_fluminurs_resource(api, resource, resource_type).await,
        Resource::InternalVideo(resource) => {
            download_fluminurs_resource(api, resource, resource_type).await
        }
        Resource::ExternalVideo(resource) => {
            download_fluminurs_resource(api, resource, resource_type).await
        }
        Resource::WebLectureVideo(resource) => {
            download_fluminurs_resource(api, resource, resource_type).await
        }
        Resource::ZoomRecording(resource) => {
            download_zoom_recording(api, resource, resource_type).await
        }
    }
}

pub async fn download_fluminurs_resource<T: FluminursResource>(
    api: Api,
    file: T,
    resource_type: ResourceType,
    // overwrite_mode: OverwriteMode,
) -> Result<(ResourceType, String), Error> {
    // TODO: customize destination path
    let dest_path = Path::new(".");
    let temp_path = dest_path
        .join(file.path().parent().unwrap())
        .join(make_temp_file_name(file.path().file_name().unwrap()));
    let path = dest_path.join(file.path());

    let filepath = file.path().display().to_string();
    match file
        .download(&api, &path, &temp_path, OverwriteMode::Skip)
        .await
    {
        Ok(OverwriteResult::NewFile) => {
            println!("Downloaded to {}", path.to_string_lossy());
            Ok((resource_type, filepath))
        }
        Ok(OverwriteResult::AlreadyHave) => Ok((resource_type, filepath)),
        Ok(OverwriteResult::Skipped) => {
            println!("Skipped {}", path.to_string_lossy());
            Ok((resource_type, filepath))
        }
        Ok(OverwriteResult::Overwritten) => {
            println!("Updated {}", path.to_string_lossy());
            Ok((resource_type, filepath))
        }
        Ok(OverwriteResult::Renamed { renamed_path }) => {
            println!(
                "Renamed {} to {}",
                path.to_string_lossy(),
                renamed_path.to_string_lossy()
            );
            Ok((resource_type, filepath))
        }
        Err(e) => {
            println!("Failed to download file: {}", e);
            Err(Error {})
        }
    }
}

pub async fn download_zoom_recording<T: FluminursResource>(
    mut api: Api,
    file: T,
    resource_type: ResourceType,
    // overwrite_mode: OverwriteMode,
) -> Result<(ResourceType, String), Error> {
    match api.login_zoom().await {
        Err(e) => {
            println!("Failed to log in to Zoom: {}", e);
            // TODO
            download_fluminurs_resource(api, file, resource_type).await
        }
        Ok(_) => {
            println!("Logged in to Zoom");
            download_fluminurs_resource(api, file, resource_type).await
        }
    }
}
