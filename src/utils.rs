use std::path::{Path, PathBuf};

pub fn expand_tilde<P: AsRef<Path>>(path: P) -> PathBuf {
    let path_str = path.as_ref().to_string_lossy();

    if let Some(stripped) = path_str.strip_prefix("~/") {
        if let Some(home) = std::env::var_os("HOME") {
            let mut home_path = PathBuf::from(home);
            home_path.push(stripped);
            return home_path;
        }
    }

    path.as_ref().to_path_buf()
}
