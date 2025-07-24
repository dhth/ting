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
    let mut cmd = fx.cmd(["config", "validate", "--help"]);

    // WHEN
    // THEN
    assert_cmd_snapshot!(cmd, @r"
    success: true
    exit_code: 0
    ----- stdout -----
    Validate ting's config file

    Usage: ting config validate [OPTIONS]

    Options:
      -C, --config-path <PATH>  Path to the config file (overrides ting's default config path)
          --debug               Output debug information without doing anything
      -h, --help                Print help

    ----- stderr -----
    ");
}

#[test]
fn debug_flag_works() {
    // GIVEN
    let fx = Fixture::new();
    let mut success_cmd = fx.cmd(["config", "validate", "--debug"]);

    // WHEN
    // THEN
    assert_cmd_snapshot!(success_cmd, @r"
    success: true
    exit_code: 0
    ----- stdout -----
    DEBUG INFO:

    command:              validate config
    flags:
      config path:        <NOT PROVIDED>

    ----- stderr -----
    ");
}

#[test]
fn valid_config_passes_validation() {
    // GIVEN
    let fx = Fixture::new();
    let mut cmd = fx.cmd([
        "config",
        "validate",
        "-C",
        "tests/testdata/both-exit-codes.toml",
    ]);

    // WHEN
    // THEN
    assert_cmd_snapshot!(cmd, @r"
    success: true
    exit_code: 0
    ----- stdout -----
    config looks good ✅

    ----- stderr -----
    ");
}

#[test]
fn valid_config_with_cues_only_passes_validation() {
    // GIVEN
    let fx = Fixture::new();
    let mut cmd = fx.cmd(["config", "validate", "-C", "tests/testdata/cues-only.toml"]);

    // WHEN
    // THEN
    assert_cmd_snapshot!(cmd, @r"
    success: true
    exit_code: 0
    ----- stdout -----
    config looks good ✅

    ----- stderr -----
    ");
}

#[test]
fn empty_config_passes_validation() {
    // GIVEN
    let fx = Fixture::new();
    let mut cmd = fx.cmd([
        "config",
        "validate",
        "-C",
        "tests/testdata/empty-config.toml",
    ]);

    // WHEN
    // THEN
    assert_cmd_snapshot!(cmd, @r"
    success: true
    exit_code: 0
    ----- stdout -----
    config looks good ✅

    ----- stderr -----
    ");
}

//------------//
//  FAILURES  //
//------------//

#[test]
fn malformed_config_fails_validation() {
    // GIVEN
    let fx = Fixture::new();
    let mut cmd = fx.cmd(["config", "validate", "-C", "tests/testdata/malformed.toml"]);

    // WHEN
    // THEN
    assert_cmd_snapshot!(cmd, @r"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    Error: config validation failed

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
fn invalid_config_fails_validation() {
    // GIVEN
    let fx = Fixture::new();
    let mut cmd = fx.cmd([
        "config",
        "validate",
        "-C",
        "tests/testdata/invalid-data.toml",
    ]);

    // WHEN
    // THEN
    assert_cmd_snapshot!(cmd, @r"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    Error: config validation failed

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
fn validating_a_non_existent_config_file_fails() {
    // GIVEN
    let fx = Fixture::new();
    let mut cmd = fx.cmd([
        "config",
        "validate",
        "-C",
        "tests/testdata/does-not-exist.toml",
    ]);

    // WHEN
    // THEN
    assert_cmd_snapshot!(cmd, @r"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    Error: config validation failed

    Caused by:
        no file exists at path tests/testdata/does-not-exist.toml
    ");
}

#[test]
fn config_file_with_non_existent_sound_files_fails_validation() {
    // GIVEN
    let fx = Fixture::new();
    let mut cmd = fx.cmd([
        "config",
        "validate",
        "-C",
        "tests/testdata/missing-files.toml",
    ]);

    // WHEN
    // THEN
    assert_cmd_snapshot!(cmd, @r"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    Error: found 3 validation errors:
      1. file associated with exit_codes.success does not exist: 'does/not/exist.wav'
      2. file associated with cues.build-fail does not exist: 'yet/another/missing.wav'
      3. file associated with cues.test-cue does not exist: 'also/does/not/exist.wav'
    ");
}

#[test]
fn config_file_with_sound_paths_pointing_to_directories_fails_validation() {
    // GIVEN
    let fx = Fixture::new();
    let mut cmd = fx.cmd([
        "config",
        "validate",
        "-C",
        "tests/testdata/invalid-files.toml",
    ]);

    // WHEN
    // THEN
    assert_cmd_snapshot!(cmd, @r"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    Error: found 2 validation errors:
      1. path associated with exit_codes.success is not a file: 'tests'
      2. path associated with cues.test-cue is not a file: 'src'
    ");
}

#[test]
fn config_file_with_single_missing_sound_file_fails_validation() {
    // GIVEN
    let fx = Fixture::new();
    let mut cmd = fx.cmd([
        "config",
        "validate",
        "-C",
        "tests/testdata/single-missing-file.toml",
    ]);

    // WHEN
    // THEN
    assert_cmd_snapshot!(cmd, @r"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    Error: file associated with exit_codes.error does not exist: 'does/not/exist.wav'
    ");
}
