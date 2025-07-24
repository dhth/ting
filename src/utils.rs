use std::path::{Path, PathBuf};

pub fn expand_tilde<P>(path: P) -> PathBuf
where
    P: AsRef<Path>,
{
    let path = path.as_ref();

    if let Ok(stripped) = path.strip_prefix("~/") {
        if let Ok(home) = etcetera::home_dir() {
            return home.join(stripped);
        }
    }

    path.to_path_buf()
}
