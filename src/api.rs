use fluminurs::file::File;
use fluminurs::module::Module;
use fluminurs::resource::Resource as FluminursResource;
use fluminurs::resource::{OverwriteMode, OverwriteResult};
use fluminurs::Api;
use futures_util::future;
use std::ffi::OsStr;
use std::ffi::OsString;
use std::path::Path;

use crate::resource::Resource;
use crate::resource::ResourceState;
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

pub async fn fetch_files(api: Api, modules: Vec<Module>) -> Result<Vec<ResourceState>, Error> {
    let files = load_modules_files(&api, &modules)
        .await?
        .into_iter()
        .map(|file| ResourceState::new(Resource::File(file)))
        .collect();

    Ok(files)
}

// TODO: reduce code duplication with fluminurs

async fn load_modules_files(api: &Api, modules: &[Module]) -> Result<Vec<File>, Error> {
    let include_uploadable_folders = true;

    let root_dirs = modules
        .iter()
        .filter(|module| module.has_access())
        .map(|module| {
            (
                module.workbin_root(|code| Path::new(code).to_owned()),
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

fn make_temp_file_name(name: &OsStr) -> OsString {
    let prepend = OsStr::new("~!");
    let mut res = OsString::with_capacity(prepend.len() + name.len());
    res.push(prepend);
    res.push(name);
    res
}

pub async fn download_resource<T: FluminursResource>(
    api: Api,
    file: T,
    // overwrite_mode: OverwriteMode,
) -> Result<String, Error> {
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
            Ok(filepath)
        }
        Ok(OverwriteResult::AlreadyHave) => Ok(filepath),
        Ok(OverwriteResult::Skipped) => {
            println!("Skipped {}", path.to_string_lossy());
            Ok(filepath)
        }
        Ok(OverwriteResult::Overwritten) => {
            println!("Updated {}", path.to_string_lossy());
            Ok(filepath)
        }
        Ok(OverwriteResult::Renamed { renamed_path }) => {
            println!(
                "Renamed {} to {}",
                path.to_string_lossy(),
                renamed_path.to_string_lossy()
            );
            Ok(filepath)
        }
        Err(e) => {
            println!("Failed to download file: {}", e);
            Err(Error {})
        }
    }
}
