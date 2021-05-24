use std::collections::HashMap;
use std::time::SystemTime;

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

    modules.items.retain(|module| module.last_updated != SystemTime::UNIX_EPOCH);
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
}
