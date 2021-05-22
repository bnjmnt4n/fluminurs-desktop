use futures_util::future;
use std::ffi::OsStr;
use std::ffi::OsString;
use std::path::{Path, PathBuf};

use fluminurs::module::Module;
use fluminurs::resource::{sort_and_make_all_paths_unique, OverwriteMode, OverwriteResult, Resource as FluminursResource};
use fluminurs::Api;

use crate::resource::{Resource, ResourceState};
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
        // TODO: no hardcode!
        .modules(Some("2020".to_owned()))
        .await
        .map_err(|_| Error {})?;

    Ok((api, name, modules))
}

// TODO: reduce code duplication with fluminurs

pub async fn load_modules_files(
    api: Api,
    modules: Vec<Module>,
) -> Result<Vec<ResourceState>, Error> {
    let include_uploadable_folders = true;

    let root_dirs = modules
        .iter()
        .filter(|module| module.has_access())
        .map(|module| {
            (
                module.id.clone(),
                module.workbin_root(|_| PathBuf::new()),
                module.is_teaching(),
            )
        })
        .collect::<Vec<_>>();

    let (files, errors) =
        future::join_all(root_dirs.into_iter().map(|(module_id, root_dir, _)| async {
            let files = root_dir
                .load(&api, include_uploadable_folders)
                .await
                .map(|mut files| {
                    // to avoid duplicate files from being corrupted,
                    // we append the id to duplicate resources
                    sort_and_make_all_paths_unique(&mut files);
                    files
                });

            (module_id, files)
        }))
        .await
        .into_iter()
        .fold(
            (vec![], vec![]),
            move |(mut ok, mut err), (module_id, res)| {
                match res {
                    Ok(dir) => {
                        let mut resources = dir
                            .into_iter()
                            .map(|video| {
                                ResourceState::new(Resource::File(video), module_id.clone())
                            })
                            .collect::<Vec<_>>();
                        ok.append(&mut resources);
                    }
                    Err(e) => {
                        err.push((module_id, e));
                    }
                }
                (ok, err)
            },
        );
    for (module_id, e) in errors {
        println!("Failed loading module files: {} {}", module_id, e);
    }
    Ok(files)
}

pub async fn load_modules_multimedia(
    api: Api,
    modules: Vec<Module>,
) -> Result<Vec<ResourceState>, Error> {
    let multimedias = modules
        .iter()
        .filter(|module| module.has_access())
        .map(|module| {
            (
                module.id.clone(),
                module.multimedia_root(|_| PathBuf::new()),
            )
        })
        .collect::<Vec<_>>();

    let (videos, errors) = future::join_all(multimedias.into_iter().map(
        |(module_id, multimedia)| async {
            let videos = multimedia.load(&api).await
                .map(|(mut ivs, mut evs)| {
                    // to avoid duplicate files from being corrupted,
                    // we append the id to duplicate resources
                    sort_and_make_all_paths_unique(&mut ivs);
                    sort_and_make_all_paths_unique(&mut evs);
                    (ivs, evs)
                });
            (module_id, videos)
        },
    ))
    .await
    .into_iter()
    .fold(
        (vec![], vec![]),
        move |(mut ok, mut err), (module_id, res)| {
            match res {
                Ok((iv, ev)) => {
                    let mut iv = iv
                        .into_iter()
                        .map(|video| {
                            ResourceState::new(Resource::InternalVideo(video), module_id.clone())
                        })
                        .collect::<Vec<_>>();
                    let mut ev = ev
                        .into_iter()
                        .map(|video| {
                            ResourceState::new(Resource::ExternalVideo(video), module_id.clone())
                        })
                        .collect::<Vec<_>>();
                    ok.append(&mut iv);
                    ok.append(&mut ev);
                }
                Err(e) => {
                    err.push((module_id, e));
                }
            }
            (ok, err)
        },
    );

    for (module_id, e) in errors {
        println!("Failed loading module multimedia: {} {}", module_id, e);
    }

    Ok(videos)
}

pub async fn load_modules_weblectures(
    api: Api,
    modules: Vec<Module>,
) -> Result<Vec<ResourceState>, Error> {
    let weblectures = modules
        .iter()
        .filter(|module| module.has_access())
        .map(|module| {
            (
                module.id.clone(),
                module.weblecture_root(|_| PathBuf::new()),
            )
        })
        .collect::<Vec<_>>();

    let (files, errors) = future::join_all(weblectures.into_iter().map(
        |(module_id, weblecture)| async {
            let weblectures = weblecture.load(&api).await
                .map(|mut weblectures| {
                    // to avoid duplicate files from being corrupted,
                    // we append the id to duplicate resources
                    sort_and_make_all_paths_unique(&mut weblectures);
                    weblectures
                });
            (module_id, weblectures)
        },
    ))
    .await
    .into_iter()
    .fold(
        (vec![], vec![]),
        move |(mut ok, mut err), (module_id, res)| {
            match res {
                Ok(dir) => {
                    let mut resources = dir
                        .into_iter()
                        .map(|weblecture| {
                            ResourceState::new(
                                Resource::WebLectureVideo(weblecture),
                                module_id.clone(),
                            )
                        })
                        .collect::<Vec<_>>();
                    ok.append(&mut resources);
                }
                Err(e) => {
                    err.push((module_id, e));
                }
            }
            (ok, err)
        },
    );

    for (module_id, e) in errors {
        println!("Failed loading module web lecture: {} {}", module_id, e);
    }

    Ok(files)
}

pub async fn load_modules_conferences(
    api: Api,
    modules: Vec<Module>,
) -> Result<Vec<ResourceState>, Error> {
    let conferences = modules
        .iter()
        .filter(|module| module.has_access())
        .map(|module| {
            (
                module.id.clone(),
                module.conferencing_root(|_| PathBuf::new()),
            )
        })
        .collect::<Vec<_>>();

    let (zoom_recordings, errors) = future::join_all(conferences.into_iter().map(
        |(module_id, conference)| async {
            let recordings = conference.load(&api).await
                .map(|mut recordings| {
                    // to avoid duplicate files from being corrupted,
                    // we append the id to duplicate resources
                    sort_and_make_all_paths_unique(&mut recordings);
                    recordings
                });
            (module_id, recordings)
        },
    ))
    .await
    .into_iter()
    .fold(
        (vec![], vec![]),
        move |(mut ok, mut err), (module_id, res)| {
            match res {
                Ok(dir) => {
                    let mut resources = dir
                        .into_iter()
                        .map(|recording| {
                            ResourceState::new(
                                Resource::ZoomRecording(recording),
                                module_id.clone(),
                            )
                        })
                        .collect::<Vec<_>>();
                    ok.append(&mut resources);
                }
                Err(e) => {
                    err.push((module_id, e));
                }
            }
            (ok, err)
        },
    );

    for (module_id, e) in errors {
        println!("Failed loading module conferences: {} {}", module_id, e);
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
    download_path: PathBuf,
    path: PathBuf,
    // overwrite_mode: OverwriteMode,
) -> Result<PathBuf, Error> {
    match resource {
        Resource::File(resource) => {
            download_fluminurs_resource(api, resource, download_path, path).await
        }
        Resource::InternalVideo(resource) => {
            download_fluminurs_resource(api, resource, download_path, path).await
        }
        Resource::ExternalVideo(resource) => {
            download_fluminurs_resource(api, resource, download_path, path).await
        }
        Resource::WebLectureVideo(resource) => {
            download_fluminurs_resource(api, resource, download_path, path).await
        }
        Resource::ZoomRecording(resource) => {
            download_zoom_recording(api, resource, download_path, path).await
        }
    }
}

pub async fn download_fluminurs_resource<T: FluminursResource>(
    api: Api,
    file: T,
    path: PathBuf,
    return_path: PathBuf,
    // overwrite_mode: OverwriteMode,
) -> Result<PathBuf, Error> {
    // TODO: customize destination path
    let dest_path = Path::new(".");
    let temp_path = dest_path
        .join(path.parent().unwrap())
        .join(make_temp_file_name(path.file_name().unwrap()));
    let filepath = dest_path.join(path.clone());

    match file
        .download(&api, &filepath, &temp_path, OverwriteMode::Skip)
        .await
    {
        Ok(OverwriteResult::NewFile) => {
            println!("Downloaded to {}", path.to_string_lossy());
            Ok(return_path)
        }
        Ok(OverwriteResult::AlreadyHave) => {
            println!("File already exists: {}", path.to_string_lossy());
            Ok(return_path)
        }
        Ok(OverwriteResult::Skipped) => {
            println!("Skipped {}", path.to_string_lossy());
            Ok(return_path)
        }
        Ok(OverwriteResult::Overwritten) => {
            println!("Updated {}", path.to_string_lossy());
            Ok(return_path)
        }
        Ok(OverwriteResult::Renamed { renamed_path }) => {
            println!(
                "Renamed {} to {}",
                path.to_string_lossy(),
                renamed_path.to_string_lossy()
            );
            Ok(return_path)
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
    download_path: PathBuf,
    path: PathBuf,
    // overwrite_mode: OverwriteMode,
) -> Result<PathBuf, Error> {
    match api.login_zoom().await {
        Err(e) => {
            println!("Failed to log in to Zoom: {}", e);
            // TODO
            download_fluminurs_resource(api, file, download_path, path).await
        }
        Ok(_) => {
            println!("Logged in to Zoom");
            download_fluminurs_resource(api, file, download_path, path).await
        }
    }
}
