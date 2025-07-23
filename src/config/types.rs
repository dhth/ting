use serde::Deserialize;
use std::collections::BTreeMap;

#[allow(unused)]
#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(serde::Serialize))]
pub struct Config {
    exit_codes: Option<ExitCodeSounds>,
    #[serde(default)]
    cues: BTreeMap<String, String>,
}

#[allow(unused)]
#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(serde::Serialize))]
pub struct ExitCodeSounds {
    success: Option<String>,
    error: Option<String>,
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
        let config = toml::from_str::<Config>(toml_content).expect("config should've been parsed");

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
        let config = toml::from_str::<Config>(toml_content).expect("config should've been parsed");

        // THEN
        assert_yaml_snapshot!(config, @r"
        exit_codes:
          success: ~/ting/sounds/success.wav
          error: ~/ting/sounds/error.wav
        cues: {}
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
        let config = toml::from_str::<Config>(toml_content).expect("config should've been parsed");

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
        let config = toml::from_str::<Config>(toml_content).expect("config should've been parsed");

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
        let config = toml::from_str::<Config>(toml_content).expect("config should've been parsed");

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
        let config = toml::from_str::<Config>(toml_content).expect("config should've been parsed");

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
        let config = toml::from_str::<Config>(toml_content).expect("config should've been parsed");

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
        let config = toml::from_str::<Config>(toml_content).expect("config should've been parsed");

        // THEN
        assert_yaml_snapshot!(config, @r"
        exit_codes: ~
        cues: {}
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
