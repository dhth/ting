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
    let mut cmd = fx.cmd(["--help"]);

    // WHEN
    // THEN
    assert_cmd_snapshot!(cmd, @r"
    success: true
    exit_code: 0
    ----- stdout -----
    ting - audio feedback for command exit codes

    USAGE: ting <EXIT_CODE>

    ARGUMENTS:
      <EXIT_CODE>  The exit code from the previous command

    OPTIONS:
      -h, --help              Print help

    EXAMPLES:
      cargo check; ting $?

      alias t='ting $?'
      cargo build; t

    ----- stderr -----
    ");
}
