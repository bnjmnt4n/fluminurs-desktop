use std::collections::HashMap;

use crate::data::DataItems;
use crate::module::Module;
use crate::resource::ResourceState;

pub fn clean_username(username: &str) -> String {
    if username.to_lowercase().starts_with("nusstu\\") {
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
    // TODO: use latest
    modules.last_updated = new.last_updated;
    modules.fetch_status = new.fetch_status;
    modules.items.append(&mut new.items);
}

pub fn merge_resources(
    resources: &mut DataItems<ResourceState>,
    mut new: DataItems<ResourceState>,
) {
    // TODO: use latest
    resources.last_updated = new.last_updated;
    resources.fetch_status = new.fetch_status;
    resources.items.append(&mut new.items);
}
