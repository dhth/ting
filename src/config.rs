use anyhow::Context;
use etcetera::{BaseStrategy, choose_base_strategy};
use serde::Deserialize;
use std::collections::BTreeMap;
use std::path::PathBuf;

#[allow(unused)]
#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(serde::Serialize))]
pub struct Config {
    exit_codes: Option<ExitCodeSounds>,
    cues: Option<BTreeMap<String, String>>,
}

#[allow(unused)]
#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(serde::Serialize))]
pub struct ExitCodeSounds {
    success: Option<String>,
    error: Option<String>,
}

#[allow(unused)]
pub fn get_config(user_provided_path: Option<PathBuf>) -> anyhow::Result<Option<Config>> {
    let config_path = match user_provided_path {
        Some(path) => {
            let metadata = match std::fs::metadata(&path) {
                Ok(m) => m,
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                    anyhow::bail!("no file exists at path {}", path.to_string_lossy())
                }
                Err(e) => return Err(e).context("couldn't determine if config file exists"),
            };
            if !metadata.is_file() {
                anyhow::bail!("provided path is not a file");
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
                anyhow::bail!("default config path is not a file");
            }
            default_config_path
        }
    };

    let config_contents = std::fs::read_to_string(&config_path).with_context(|| {
        format!(
            "couldn't read config file at {}",
            &config_path.to_string_lossy()
        )
    })?;

    let config: Config = toml::from_str(&config_contents).with_context(|| {
        format!(
            "couldn't parse config file at {}",
            &config_path.to_string_lossy()
        )
    })?;

    Ok(Some(config))
}

fn get_default_config_path() -> anyhow::Result<PathBuf> {
    let strategy = choose_base_strategy()?;

    Ok(strategy.config_dir().join("ting").join("ting.toml"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::{assert_snapshot, assert_yaml_snapshot};

    //-------------//
    //  SUCCESSES  //
    //-------------//

    #[test]
    fn parsing_a_valid_config_works() {
        // GIVEN
        let toml_content = r#"
[exit_codes]
success = "~/ting/sounds/success.wav"
error = "~/ting/sounds/error.wav"

[cues]
build-success = "/Users/user/ting/sounds/victory.wav"
build-fail = "/Users/user/ting/sounds/buzzer.wav"
"#;

        // WHEN
        let config: Config = toml::from_str(toml_content).expect("config should've been parsed");

        // THEN
        assert_yaml_snapshot!(config, @r"
        exit_codes:
          success: ~/ting/sounds/success.wav
          error: ~/ting/sounds/error.wav
        cues:
          build-fail: /Users/user/ting/sounds/buzzer.wav
          build-success: /Users/user/ting/sounds/victory.wav
        ");
    }

    #[test]
    fn parsing_config_with_no_cues_works() {
        // GIVEN
        let toml_content = r#"
[exit_codes]
success = "~/ting/sounds/success.wav"
error = "~/ting/sounds/error.wav"
"#;

        // WHEN
        let config: Config = toml::from_str(toml_content).expect("config should've been parsed");

        // THEN
        assert_yaml_snapshot!(config, @r"
        exit_codes:
          success: ~/ting/sounds/success.wav
          error: ~/ting/sounds/error.wav
        cues: ~
        ");
    }

    #[test]
    fn parsing_config_with_empty_cues_section_works() {
        // GIVEN
        let toml_content = r#"
[exit_codes]
success = "~/ting/sounds/success.wav"
error = "~/ting/sounds/error.wav"

[cues]
"#;

        // WHEN
        let config: Config = toml::from_str(toml_content).expect("config should've been parsed");

        // THEN
        assert_yaml_snapshot!(config, @r"
        exit_codes:
          success: ~/ting/sounds/success.wav
          error: ~/ting/sounds/error.wav
        cues: {}
        ");
    }

    #[test]
    fn parsing_config_with_no_exit_codes_works() {
        // GIVEN
        let toml_content = r#"
[cues]
build-success = "~/ting/sounds/victory.wav"
build-fail = "~/ting/sounds/buzzer.wav"
"#;

        // WHEN
        let config: Config = toml::from_str(toml_content).expect("config should've been parsed");

        // THEN
        assert_yaml_snapshot!(config, @r"
        exit_codes: ~
        cues:
          build-fail: ~/ting/sounds/buzzer.wav
          build-success: ~/ting/sounds/victory.wav
        ");
    }

    #[test]
    fn parsing_config_with_empty_exit_codes_section_works() {
        // GIVEN
        let toml_content = r#"
[exit_codes]

[cues]
build-success = "~/ting/sounds/victory.wav"
build-fail = "~/ting/sounds/buzzer.wav"
"#;

        // WHEN
        let config: Config = toml::from_str(toml_content).expect("config should've been parsed");

        // THEN
        assert_yaml_snapshot!(config, @r"
        exit_codes:
          success: ~
          error: ~
        cues:
          build-fail: ~/ting/sounds/buzzer.wav
          build-success: ~/ting/sounds/victory.wav
        ");
    }

    #[test]
    fn parsing_config_without_success_code_works() {
        // GIVEN
        let toml_content = r#"
[exit_codes]
error = "error.wav"

[cues]
test = "~/ting/sounds/test.wav"
"#;

        // WHEN
        let config: Config = toml::from_str(toml_content).expect("config should've been parsed");

        // THEN
        assert_yaml_snapshot!(config, @r"
        exit_codes:
          success: ~
          error: error.wav
        cues:
          test: ~/ting/sounds/test.wav
        ");
    }

    #[test]
    fn parsing_config_without_error_code_works() {
        // GIVEN
        let toml_content = r#"
[exit_codes]
success = "success.wav"

[cues]
test = "~/ting/sounds/test.wav"
"#;

        // WHEN
        let config: Config = toml::from_str(toml_content).expect("config should've been parsed");

        // THEN
        assert_yaml_snapshot!(config, @r"
        exit_codes:
          success: success.wav
          error: ~
        cues:
          test: ~/ting/sounds/test.wav
        ");
    }

    #[test]
    fn parsing_empty_config_works() {
        // GIVEN
        let toml_content = "";

        // WHEN
        let config: Config = toml::from_str(toml_content).expect("config should've been parsed");

        // THEN
        assert_yaml_snapshot!(config, @r"
        exit_codes: ~
        cues: ~
        ");
    }

    //------------//
    //  FAILURES  //
    //------------//

    #[test]
    fn parsing_config_with_invalid_toml_fails() {
        // GIVEN
        let toml_content = r#"
[exit_codes
success = "~/ting/sounds/success.wav"
error = "~/ting/sounds/error.wav"

[cues]
test = "~/ting/sounds/test.wav"
"#;

        // WHEN
        let error = toml::from_str::<Config>(toml_content).expect_err("parsing should've failed");

        // THEN
        assert_snapshot!(error, @r"
        TOML parse error at line 2, column 12
          |
        2 | [exit_codes
          |            ^
        unclosed table, expected `]`
        ");
    }

    #[test]
    fn parsing_config_with_invalid_data_fails() {
        // GIVEN
        let toml_content = r#"
[exit_codes]
success = 1
error = "~/ting/sounds/error.wav"

[cues]
test = "~/ting/sounds/test.wav"
"#;

        // WHEN
        let error = toml::from_str::<Config>(toml_content).expect_err("parsing should've failed");

        // THEN
        assert_snapshot!(error, @r"
        TOML parse error at line 3, column 11
          |
        3 | success = 1
          |           ^
        invalid type: integer `1`, expected a string
        ");
    }
}
