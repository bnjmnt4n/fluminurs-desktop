use std::path::PathBuf;
use std::time::SystemTime;
use std::{cmp::Ordering, collections::HashMap};

use crate::data::DataItems;
use crate::module::Module;
use crate::resource::ResourceState;

pub fn clean_username(username: &str) -> String {
    let username = username.to_lowercase();
    if username.starts_with("nusstu\\") {
        username.to_owned()
    } else {
        format!("nusstu\\{}", username)
    }
}

pub fn construct_modules_map(modules: &[Module]) -> HashMap<String, Module> {
    // TODO: avoid cloning everything
    modules
        .iter()
        .map(|item| (item.id.to_string(), item.clone()))
        .collect()
}

pub fn merge_modules(modules: &mut DataItems<Module>, mut new: DataItems<Module>) {
    // We assume that the more recently fetched data is always going to be fresher
    // than the local data, and merge without checking.
    modules.last_updated = new.last_updated;
    modules.fetch_status = new.fetch_status;
    modules.items.append(&mut new.items);

    // Sort by term (later semesters first), followed by module code and last updated time.
    modules.items.sort_unstable_by(|m1, m2| {
        m1.term
            .cmp(&m2.term)
            .reverse()
            .then_with(|| m1.code.cmp(&m2.code))
            .then_with(|| m1.last_updated.cmp(&m2.last_updated))
    });

    // Mark all older duplicates with last updated time of the Unix epoch to indicate removal.
    modules
        .items
        .iter_mut()
        .fold(&mut Module::empty(), |mut prev, curr| {
            if prev.term == curr.term && prev.code == curr.code {
                prev.last_updated = SystemTime::UNIX_EPOCH;

                prev
            } else {
                curr
            }
        });

    modules
        .items
        .retain(|module| module.last_updated != SystemTime::UNIX_EPOCH);
}

pub fn merge_resources(
    resources: &mut DataItems<ResourceState>,
    mut new: DataItems<ResourceState>,
) {
    // We assume that the more recently fetched data is always going to be fresher
    // than the local data, and merge without checking.
    resources.last_updated = new.last_updated;
    resources.fetch_status = new.fetch_status;
    resources.items.append(&mut new.items);

    // Sort by module ID, followed by path, last updated time and whether there is a
    // local download.
    resources.items.sort_unstable_by(|m1, m2| {
        m1.module_id
            .cmp(&m2.module_id)
            .then_with(|| m1.path.cmp(&m2.path))
            .then_with(|| m1.last_updated.cmp(&m2.last_updated))
            .then_with(|| {
                if let Some(_) = m1.download_path {
                    Ordering::Less
                } else if let Some(_) = m2.download_path {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            })
    });

    resources
        .items
        .iter_mut()
        .fold(&mut ResourceState::empty(), |prev, curr| {
            // We keep the older resource, since it would contain our persisted information
            // about download path and time, and update its last updated timing from the new
            // resource.
            if prev.module_id == curr.module_id && prev.path == curr.path {
                curr.path = PathBuf::new();
                std::mem::swap(&mut prev.last_updated, &mut curr.last_updated);

                if let Some(_) = curr.resource {
                    std::mem::swap(&mut prev.resource, &mut curr.resource);
                }

                prev
            } else {
                curr
            }
        });

    resources
        .items
        .retain(|resource| !resource.path.as_os_str().is_empty());
}
