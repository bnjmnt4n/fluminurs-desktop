use fluminurs::file::File;
use fluminurs::module::Module;
use fluminurs::Api;
use futures_util::future;
use std::path::Path;

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

pub async fn fetch_files(api: Api, modules: Vec<Module>) -> Result<Vec<File>, Error> {
    let files = load_modules_files(&api, &modules)
        .await?;

    Ok(files)
}

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
