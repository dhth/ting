use crate::config::Config;
use crate::utils::expand_tilde;
use anyhow::Context;

#[allow(unused)]
#[derive(Debug)]
#[cfg_attr(test, derive(serde::Serialize))]
pub struct PlayData {
    pub kind: PlayKind,
}

#[derive(Debug)]
#[cfg_attr(test, derive(serde::Serialize))]
pub struct PlayBehaviours {
    pub match_exit_code: bool,
}

#[allow(unused)]
#[derive(Debug)]
#[cfg_attr(test, derive(serde::Serialize))]
pub enum PlayKind {
    ExitCode {
        code: i32,
        success: AudioSource,
        error: AudioSource,
    },
    // most external files are expected to be tiny, so loading them into memory simplifies things a
    // lot
    #[cfg_attr(test, serde(serialize_with = "serialize_bytes_for_testing"))]
    Cue(Vec<u8>),
}

#[derive(Debug)]
#[cfg_attr(test, derive(serde::Serialize))]
pub enum AudioSource {
    Builtin,
    #[cfg_attr(test, serde(serialize_with = "serialize_bytes_for_testing"))]
    External(Vec<u8>),
}

#[derive(Debug, Clone)]
pub enum PlayInputKind {
    ExitCode(i32),
    Cue(String),
}

impl PlayKind {
    fn exit_code_with_builtin_sounds(code: i32) -> Self {
        PlayKind::ExitCode {
            code,
            success: AudioSource::Builtin,
            error: AudioSource::Builtin,
        }
    }

    fn exit_code(
        code: i32,
        success_sound_file: Option<String>,
        error_sound_file: Option<String>,
    ) -> anyhow::Result<Self> {
        let (success, error) = match (success_sound_file, error_sound_file) {
            (None, None) => (AudioSource::Builtin, AudioSource::Builtin),
            (None, Some(e)) => (
                AudioSource::Builtin,
                AudioSource::External(read_file_for_exit_code(false, &e)?),
            ),
            (Some(s), None) => (
                AudioSource::External(read_file_for_exit_code(true, &s)?),
                AudioSource::Builtin,
            ),
            (Some(s), Some(e)) => (
                AudioSource::External(read_file_for_exit_code(true, &s)?),
                AudioSource::External(read_file_for_exit_code(false, &e)?),
            ),
        };

        Ok(PlayKind::ExitCode {
            code,
            success,
            error,
        })
    }
}

fn read_file_for_exit_code(success: bool, path: &str) -> anyhow::Result<Vec<u8>> {
    let path = expand_tilde(path);
    let bytes = std::fs::read(&path).with_context(|| {
        let code_type = if success { "success" } else { "error" };
        format!(
            r#"couldn't read file configured for {} exit code ("{}")"#,
            code_type,
            path.to_string_lossy()
        )
    })?;

    Ok(bytes)
}

pub struct PlayArgs {
    pub input_kind: PlayInputKind,
    pub match_exit_code: bool,
}

pub fn parse_args_and_config(
    args: PlayArgs,
    maybe_config: Option<Config>,
) -> anyhow::Result<(PlayData, PlayBehaviours)> {
    let kind = if let Some(config) = maybe_config {
        match args.input_kind {
            PlayInputKind::ExitCode(e) => match config.exit_codes {
                Some(exit_code_sounds) => {
                    PlayKind::exit_code(e, exit_code_sounds.success, exit_code_sounds.error)?
                }

                None => PlayKind::exit_code_with_builtin_sounds(e),
            },
            PlayInputKind::Cue(cue) => match config.cues.get(&cue) {
                Some(path) => {
                    let path = expand_tilde(path);
                    let bytes = std::fs::read(&path).with_context(|| {
                        format!(
                            r#"couldn't read file configured for cue {cue} ('{}')"#,
                            path.to_string_lossy()
                        )
                    })?;
                    PlayKind::Cue(bytes)
                }
                None => anyhow::bail!(r#"cue not found: '{cue}'"#),
            },
        }
    } else {
        match args.input_kind {
            PlayInputKind::ExitCode(e) => PlayKind::exit_code_with_builtin_sounds(e),
            PlayInputKind::Cue(_) => anyhow::bail!("no cues configured"),
        }
    };

    Ok((
        PlayData { kind },
        PlayBehaviours {
            match_exit_code: args.match_exit_code,
        },
    ))
}

#[cfg(test)]
fn serialize_bytes_for_testing<S>(_bytes: &[u8], serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str("<bytes>")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::ExitCodeSounds;
    use insta::{assert_snapshot, assert_yaml_snapshot};
    use std::collections::BTreeMap;

    //-------------//
    //  SUCCESSES  //
    //-------------//

    #[test]
    fn parsing_exit_code_with_no_config_uses_builtin_sounds() {
        // GIVEN
        let args = PlayArgs {
            input_kind: PlayInputKind::ExitCode(0),
            match_exit_code: true,
        };
        let config = None;

        // WHEN
        let (play_data, _play_behaviours) =
            parse_args_and_config(args, config).expect("parsing should've succeeded");

        // THEN
        assert_yaml_snapshot!(play_data, @r"
        kind:
          ExitCode:
            code: 0
            success: Builtin
            error: Builtin
        ");
    }

    #[test]
    fn parsing_config_with_no_custom_exit_codes_uses_builtin_sounds() {
        // GIVEN
        let args = PlayArgs {
            input_kind: PlayInputKind::ExitCode(1),
            match_exit_code: false,
        };
        let mut cues = BTreeMap::new();
        cues.insert("test-cue".to_string(), "/path/to/test.wav".to_string());
        let config = Some(Config {
            exit_codes: None,
            cues,
        });

        // WHEN
        let (play_data, _play_behaviours) =
            parse_args_and_config(args, config).expect("parsing should've succeeded");

        // THEN
        assert_yaml_snapshot!(play_data, @r"
        kind:
          ExitCode:
            code: 1
            success: Builtin
            error: Builtin
        ");
    }

    #[test]
    fn parsing_config_with_custom_success_code_sounds_works() {
        // GIVEN
        let args = PlayArgs {
            input_kind: PlayInputKind::ExitCode(0),
            match_exit_code: true,
        };
        let config = Some(Config {
            exit_codes: Some(ExitCodeSounds {
                success: Some("src/cmds/assets/audio/success.wav".to_string()),
                error: None,
            }),
            cues: BTreeMap::new(),
        });

        // WHEN
        let (play_data, _play_behaviours) =
            parse_args_and_config(args, config).expect("parsing should've succeeded");

        // THEN
        assert_yaml_snapshot!(play_data, @r#"
        kind:
          ExitCode:
            code: 0
            success:
              External: "<bytes>"
            error: Builtin
        "#);
    }

    #[test]
    fn parsing_config_with_custom_error_code_sounds_works() {
        // GIVEN
        let args = PlayArgs {
            input_kind: PlayInputKind::ExitCode(1),
            match_exit_code: false,
        };
        let config = Some(Config {
            exit_codes: Some(ExitCodeSounds {
                success: None,
                error: Some("src/cmds/assets/audio/error.wav".to_string()),
            }),
            cues: BTreeMap::new(),
        });

        // WHEN
        let (play_data, _play_behaviours) =
            parse_args_and_config(args, config).expect("parsing should've succeeded");

        // THEN
        assert_yaml_snapshot!(play_data, @r#"
        kind:
          ExitCode:
            code: 1
            success: Builtin
            error:
              External: "<bytes>"
        "#);
    }

    #[test]
    fn parsing_config_with_custom_exit_code_sounds_works() {
        // GIVEN
        let args = PlayArgs {
            input_kind: PlayInputKind::ExitCode(0),
            match_exit_code: true,
        };
        let config = Some(Config {
            exit_codes: Some(ExitCodeSounds {
                success: Some("src/cmds/assets/audio/success.wav".to_string()),
                error: Some("src/cmds/assets/audio/error.wav".to_string()),
            }),
            cues: BTreeMap::new(),
        });

        // WHEN
        let (play_data, _play_behaviours) =
            parse_args_and_config(args, config).expect("parsing should've succeeded");

        // THEN
        assert_yaml_snapshot!(play_data, @r#"
        kind:
          ExitCode:
            code: 0
            success:
              External: "<bytes>"
            error:
              External: "<bytes>"
        "#);
    }

    #[test]
    fn parsing_config_with_valid_cue_works() {
        // GIVEN
        let args = PlayArgs {
            input_kind: PlayInputKind::Cue("build-success".to_string()),
            match_exit_code: true,
        };
        let mut cues = BTreeMap::new();
        cues.insert(
            "build-success".to_string(),
            "src/cmds/assets/audio/success.wav".to_string(),
        );
        let config = Some(Config {
            exit_codes: None,
            cues,
        });

        // WHEN
        let (play_data, _play_behaviours) =
            parse_args_and_config(args, config).expect("parsing should've succeeded");

        // THEN
        assert_yaml_snapshot!(play_data, @r#"
        kind:
          Cue: "<bytes>"
        "#);
    }

    //------------//
    //  FAILURES  //
    //------------//

    #[test]
    fn parsing_cue_with_no_config_fails() {
        // GIVEN
        let args = PlayArgs {
            input_kind: PlayInputKind::Cue("test-cue".to_string()),
            match_exit_code: true,
        };
        let config = None;

        // WHEN
        let error = parse_args_and_config(args, config).expect_err("parsing should've failed");

        // THEN
        assert_snapshot!(format!("{:#}", error), @"no cues configured");
    }

    #[test]
    fn parsing_cue_absent_in_config_fails() {
        // GIVEN
        let args = PlayArgs {
            input_kind: PlayInputKind::Cue("nonexistent-cue".to_string()),
            match_exit_code: true,
        };
        let mut cues = BTreeMap::new();
        cues.insert("other-cue".to_string(), "/path/to/other.wav".to_string());
        let config = Some(Config {
            exit_codes: None,
            cues,
        });

        // WHEN
        let error = parse_args_and_config(args, config).expect_err("parsing should've failed");

        // THEN
        assert_snapshot!(format!("{:#}", error), @"cue not found: 'nonexistent-cue'");
    }

    #[test]
    fn parsing_exit_code_with_missing_file_fails() {
        // GIVEN
        let args = PlayArgs {
            input_kind: PlayInputKind::ExitCode(0),
            match_exit_code: true,
        };
        let config = Some(Config {
            exit_codes: Some(ExitCodeSounds {
                success: Some("/nonexistent/path/success.wav".to_string()),
                error: None,
            }),
            cues: BTreeMap::new(),
        });

        // WHEN
        let error = parse_args_and_config(args, config).expect_err("parsing should've failed");

        // THEN
        assert_snapshot!(format!("{:#}", error), @r#"couldn't read file configured for success exit code ("/nonexistent/path/success.wav"): No such file or directory (os error 2)"#);
    }

    #[test]
    fn parsing_cue_with_missing_file_fails() {
        // GIVEN
        let args = PlayArgs {
            input_kind: PlayInputKind::Cue("test-cue".to_string()),
            match_exit_code: true,
        };
        let mut cues = BTreeMap::new();
        cues.insert(
            "test-cue".to_string(),
            "/nonexistent/path/cue.wav".to_string(),
        );
        let config = Some(Config {
            exit_codes: None,
            cues,
        });

        // WHEN
        let error = parse_args_and_config(args, config).expect_err("parsing should've failed");

        // THEN
        assert_snapshot!(format!("{:#}", error), @"couldn't read file configured for cue test-cue ('/nonexistent/path/cue.wav'): No such file or directory (os error 2)");
    }
}
