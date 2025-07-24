use crate::config::get_config;
use crate::utils::expand_tilde;
use std::path::PathBuf;

const SAMPLE_CONFIG: &str = include_str!("assets/sample-config.toml");

pub fn get_sample_config() -> &'static str {
    SAMPLE_CONFIG
}

pub fn handle_validate_config(
    user_provided_path: Option<PathBuf>,
) -> anyhow::Result<Vec<anyhow::Error>> {
    let config = get_config(user_provided_path.clone())?;

    let config = match config {
        Some(config) => config,
        None => anyhow::bail!("no config found at default location"),
    };

    let mut all_sound_files = Vec::new();

    if let Some(ref exit_codes) = config.exit_codes {
        if let Some(ref success_path) = exit_codes.success {
            all_sound_files.push(("exit_codes.success".to_string(), success_path));
        }
        if let Some(ref error_path) = exit_codes.error {
            all_sound_files.push(("exit_codes.error".to_string(), error_path));
        }
    }

    for (cue_name, cue_path) in &config.cues {
        all_sound_files.push((format!("cues.{cue_name}"), cue_path));
    }

    let mut validation_errors = Vec::new();

    for (name, path) in all_sound_files {
        let expanded_path = expand_tilde(path);

        match std::fs::metadata(&expanded_path) {
            Ok(metadata) => {
                if !metadata.is_file() {
                    validation_errors.push(anyhow::anyhow!(
                        "path associated with {} is not a file: '{}'",
                        &name,
                        expanded_path.to_string_lossy()
                    ));
                }
            }
            Err(e) => {
                if e.kind() == std::io::ErrorKind::NotFound {
                    validation_errors.push(anyhow::anyhow!(
                        "file associated with {} does not exist: '{}'",
                        &name,
                        expanded_path.to_string_lossy()
                    ));
                } else {
                    // this is a program error (in most cases), not a validation error
                    anyhow::bail!(
                        "couldn't access metadata for file associated with {}: '{}' - {}",
                        &name,
                        expanded_path.to_string_lossy(),
                        e
                    );
                }
            }
        }
    }

    Ok(validation_errors)
}
