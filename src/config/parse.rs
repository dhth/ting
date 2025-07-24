use super::types::Config;
use crate::utils::expand_tilde;
use anyhow::Context;
use etcetera::{BaseStrategy, choose_base_strategy};
use std::path::PathBuf;

#[allow(unused)]
pub fn get_config(user_provided_path: Option<PathBuf>) -> anyhow::Result<Option<Config>> {
    let config_path = match user_provided_path {
        Some(path) => {
            let path = expand_tilde(&path);
            let metadata = match std::fs::metadata(&path) {
                Ok(m) => m,
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                    anyhow::bail!("no file exists at path '{}'", path.to_string_lossy())
                }
                Err(e) => return Err(e).context("couldn't determine if config file exists"),
            };
            if !metadata.is_file() {
                anyhow::bail!("provided path is not a file: '{}'", &path.to_string_lossy());
            }
            path
        }
        None => {
            let default_config_path =
                get_default_config_path().context("couldn't get ting's default config path")?;
            let metadata = match std::fs::metadata(&default_config_path) {
                Ok(m) => m,
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(None),
                Err(e) => return Err(e).context("couldn't determine if config file exists"),
            };
            if !metadata.is_file() {
                anyhow::bail!(
                    "default config path is not a file: '{}'",
                    default_config_path.to_string_lossy()
                );
            }
            default_config_path
        }
    };

    let config_contents = std::fs::read_to_string(&config_path).with_context(|| {
        format!(
            "couldn't read config file at '{}'",
            &config_path.to_string_lossy()
        )
    })?;

    let config: Config = toml::from_str(&config_contents).with_context(|| {
        format!(
            "couldn't parse config file at '{}'",
            &config_path.to_string_lossy()
        )
    })?;

    Ok(Some(config))
}

pub fn get_default_config_path() -> anyhow::Result<PathBuf> {
    let strategy = choose_base_strategy()?;

    Ok(strategy.config_dir().join("ting").join("ting.toml"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::{assert_snapshot, assert_yaml_snapshot};
    use std::path::Path;

    fn testdata_path(filename: &str) -> PathBuf {
        Path::new("src/config/testdata").join(filename)
    }

    //-------------//
    //  SUCCESSES  //
    //-------------//

    #[test]
    fn getting_config_from_valid_config_file_works() {
        // GIVEN
        let config_path = testdata_path("valid-config.toml");

        // WHEN
        let result = get_config(Some(config_path)).expect("should load config successfully");
        let config = result.expect("should return Some(config)");

        // THEN
        assert_yaml_snapshot!(config, @r"
        exit_codes:
          success: ~/ting/sounds/success.wav
          error: ~/ting/sounds/error.wav
        cues:
          build-fail: /Users/user/ting/sounds/buzzer.wav
          build-success: /Users/user/ting/sounds/victory.wav
          test-pass: ~/sounds/chime.wav
        ");
    }

    //------------//
    //  FAILURES  //
    //------------//

    #[test]
    fn getting_config_from_non_existent_file_fails() {
        // GIVEN
        let config_path = testdata_path("does-not-exist.toml");

        // WHEN
        let error = get_config(Some(config_path)).expect_err("should've failed to get config");

        // THEN
        assert_snapshot!(format!("{:#}", error), @"no file exists at path 'src/config/testdata/does-not-exist.toml'");
    }

    #[test]
    fn getting_config_from_a_non_file_fails() {
        // GIVEN
        let config_path = testdata_path("not-a-file");

        // WHEN
        let error = get_config(Some(config_path)).expect_err("should've failed to get config");

        // THEN
        assert_snapshot!(format!("{:#}", error), @"provided path is not a file: 'src/config/testdata/not-a-file'");
    }

    #[test]
    fn getting_config_from_an_invalid_toml_file_fails() {
        // GIVEN
        let config_path = testdata_path("invalid-toml.toml");

        // WHEN
        let error = get_config(Some(config_path)).expect_err("should've failed to get config");

        // THEN
        assert_snapshot!(format!("{:#}", error), @r"
        couldn't parse config file at 'src/config/testdata/invalid-toml.toml': TOML parse error at line 1, column 12
          |
        1 | [exit_codes
          |            ^
        unclosed table, expected `]`
        ");
    }

    #[test]
    fn getting_config_from_a_toml_file_with_invalid_data_fails() {
        // GIVEN
        let config_path = testdata_path("invalid-data-types.toml");

        // WHEN
        let error = get_config(Some(config_path)).expect_err("should've failed to get config");

        // THEN
        assert_snapshot!(format!("{:#}", error), @r"
        couldn't parse config file at 'src/config/testdata/invalid-data-types.toml': TOML parse error at line 2, column 11
          |
        2 | success = 123
          |           ^^^
        invalid type: integer `123`, expected a string
        ");
    }
}
