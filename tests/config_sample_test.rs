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
    let mut cmd = fx.cmd(["config", "sample", "--help"]);

    // WHEN
    // THEN
    assert_cmd_snapshot!(cmd, @r"
    success: true
    exit_code: 0
    ----- stdout -----
    Print sample config for ting

    Usage: ting config sample [OPTIONS]

    Options:
          --debug  Output debug information without doing anything
      -h, --help   Print help

    ----- stderr -----
    ");
}

#[test]
fn debug_flag_works() {
    // GIVEN
    let fx = Fixture::new();
    let mut success_cmd = fx.cmd(["config", "sample", "--debug"]);

    // WHEN
    // THEN
    assert_cmd_snapshot!(success_cmd, @r"
    success: true
    exit_code: 0
    ----- stdout -----
    DEBUG INFO:

    command:              print sample config

    ----- stderr -----
    ");
}
