mod common;

use common::Fixture;
use insta_cmd::assert_cmd_snapshot;

//-------------//
//  SUCCESSES  //
//-------------//

#[test]
fn showing_help_works() {
    // GIVEN
    let fx = Fixture::new();
    let mut cmd = fx.cmd(["p", "--help"]);

    // WHEN
    // THEN
    assert_cmd_snapshot!(cmd, @r"
    success: true
    exit_code: 0
    ----- stdout -----
    Play sound for an input

    Usage: ting p [OPTIONS] <INPUT>

    Arguments:
      <INPUT>  Exit code (0, 1, etc.) or cue name (configured via ting's config)

    Options:
      -C, --config-path <PATH>  Path to the config file (overrides ting's default config path)
          --no-match-exit-code  Don't exit ting with the same code as the input
          --debug               Output debug information without doing anything
      -h, --help                Print help

    ----- stderr -----
    ");
}

#[test]
fn debug_flag_works() {
    // GIVEN
    let fx = Fixture::new();
    let mut success_cmd = fx.cmd([
        "p",
        "--config-path",
        "tests/testdata/success-only.toml",
        "0",
        "--debug",
    ]);

    // WHEN
    // THEN
    assert_cmd_snapshot!(success_cmd, @r"
    success: true
    exit_code: 0
    ----- stdout -----
    DEBUG INFO:

    command:                  play sound
    flags:
      input:                  0
      config path:            tests/testdata/success-only.toml
      don't match exit code:  false

    ----- stderr -----
    ");
}

#[test]
fn plays_external_success_sound_if_configured() {
    // GIVEN
    let fx = Fixture::new();
    let mut success_cmd = fx.cmd([
        "p",
        "--config-path",
        "tests/testdata/success-only.toml",
        "0",
    ]);

    // WHEN
    // THEN
    assert_cmd_snapshot!(success_cmd, @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    playing success sound from external source
    ");
}

#[test]
fn plays_external_error_sound_if_configured() {
    // GIVEN
    let fx = Fixture::new();
    let mut success_cmd = fx.cmd(["p", "--config-path", "tests/testdata/error-only.toml", "1"]);

    // WHEN
    // THEN
    assert_cmd_snapshot!(success_cmd, @r"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    playing error sound from external source
    ");
}

#[test]
fn plays_builtin_sound_if_success_sound_not_configured() {
    // GIVEN
    let fx = Fixture::new();
    let mut success_cmd = fx.cmd(["p", "--config-path", "tests/testdata/error-only.toml", "0"]);

    // WHEN
    // THEN
    assert_cmd_snapshot!(success_cmd, @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    playing built-in success sound
    ");
}

#[test]
fn plays_builtin_sound_if_error_sound_not_configured() {
    // GIVEN
    let fx = Fixture::new();
    let mut success_cmd = fx.cmd([
        "p",
        "--config-path",
        "tests/testdata/success-only.toml",
        "1",
    ]);

    // WHEN
    // THEN
    assert_cmd_snapshot!(success_cmd, @r"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    playing built-in error sound
    ");
}

#[test]
fn plays_success_sound_correctly_when_both_exit_codes_are_set() {
    // GIVEN
    let fx = Fixture::new();
    let mut success_cmd = fx.cmd([
        "p",
        "--config-path",
        "tests/testdata/both-exit-codes.toml",
        "0",
    ]);

    // WHEN
    // THEN
    assert_cmd_snapshot!(success_cmd, @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    playing success sound from external source
    ");
}

#[test]
fn plays_error_sound_correctly_when_both_exit_codes_are_set() {
    // GIVEN
    let fx = Fixture::new();
    let mut success_cmd = fx.cmd([
        "p",
        "--config-path",
        "tests/testdata/both-exit-codes.toml",
        "1",
    ]);

    // WHEN
    // THEN
    assert_cmd_snapshot!(success_cmd, @r"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    playing error sound from external source
    ");
}

#[test]
fn cue_playback_works() {
    // GIVEN
    let fx = Fixture::new();
    let mut cmd = fx.cmd([
        "p",
        "--config-path",
        "tests/testdata/cues-only.toml",
        "build-success",
    ]);

    // WHEN
    // THEN
    assert_cmd_snapshot!(cmd, @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    playing cue sound from external source
    ");
}

#[test]
fn exits_with_same_success_code() {
    // GIVEN
    let fx = Fixture::new();
    let mut cmd = fx.cmd([
        "p",
        "--config-path",
        "tests/testdata/empty-config.toml",
        "0",
    ]);

    // WHEN
    // THEN
    assert_cmd_snapshot!(cmd, @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    playing built-in success sound
    ");
}

#[test]
fn exits_with_same_error_code() {
    // GIVEN
    let fx = Fixture::new();
    let mut cmd = fx.cmd([
        "p",
        "--config-path",
        "tests/testdata/empty-config.toml",
        "42",
    ]);

    // WHEN
    // THEN
    assert_cmd_snapshot!(cmd, @r"
    success: false
    exit_code: 42
    ----- stdout -----

    ----- stderr -----
    playing built-in error sound
    ");
}

#[test]
fn doesnt_follow_input_exit_code_if_requested() {
    // GIVEN
    let fx = Fixture::new();
    let mut cmd = fx.cmd([
        "p",
        "--config-path",
        "tests/testdata/empty-config.toml",
        "--no-match-exit-code",
        "42",
    ]);

    // WHEN
    // THEN
    assert_cmd_snapshot!(cmd, @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    playing built-in error sound
    ");
}

//------------//
//  FAILURES  //
//------------//

#[test]
fn playback_fails_if_config_is_malformed() {
    // GIVEN
    let fx = Fixture::new();
    let mut cmd = fx.cmd(["p", "--config-path", "tests/testdata/malformed.toml", "0"]);

    // WHEN
    // THEN
    assert_cmd_snapshot!(cmd, @r"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    Error: couldn't get config

    Caused by:
        0: couldn't parse config file at tests/testdata/malformed.toml
        1: TOML parse error at line 1, column 12
             |
           1 | [exit_codes
             |            ^
           unclosed table, expected `]`
    ");
}

#[test]
fn playback_fails_if_config_has_invalid_data() {
    // GIVEN
    let fx = Fixture::new();
    let mut cmd = fx.cmd([
        "p",
        "--config-path",
        "tests/testdata/invalid-data.toml",
        "0",
    ]);

    // WHEN
    // THEN
    assert_cmd_snapshot!(cmd, @r"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    Error: couldn't get config

    Caused by:
        0: couldn't parse config file at tests/testdata/invalid-data.toml
        1: TOML parse error at line 2, column 11
             |
           2 | success = true
             |           ^^^^
           invalid type: boolean `true`, expected a string
    ");
}

#[test]
fn cue_playback_fails_if_not_configured() {
    // GIVEN
    let fx = Fixture::new();
    let mut cmd = fx.cmd([
        "p",
        "--config-path",
        "tests/testdata/empty-config.toml",
        "build-success",
    ]);

    // WHEN
    // THEN
    assert_cmd_snapshot!(cmd, @r#"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    Error: cue not found: "build-success"
    "#);
}
