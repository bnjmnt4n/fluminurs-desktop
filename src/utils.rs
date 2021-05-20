pub fn clean_username(username: &str) -> String {
    if username.to_lowercase().starts_with("nusstu\\") {
        username.to_owned()
    } else {
        format!("nusstu\\{}", username)
    }
}
