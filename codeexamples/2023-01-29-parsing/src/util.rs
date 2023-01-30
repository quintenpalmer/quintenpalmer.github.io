use std::path;

pub fn get_maybe_extension_string(p: &path::PathBuf) -> Option<String> {
    match p.extension() {
        Some(v) => Some(v.to_str().unwrap().to_lowercase()),
        None => None,
    }
}
