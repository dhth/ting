mod common;

use common::Fixture;
use insta_cmd::assert_cmd_snapshot;

//-------------//
//  SUCCESSES  //
//-------------//

#[test]
fn plays_success_sound_when_appropriate() {
    // GIVEN
    let fx = Fixture::new();
    let mut cmd = fx.cmd(["0"]);

    // WHEN
    // THEN
    assert_cmd_snapshot!(cmd, @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    playing success sound
    ");
}

#[test]
fn plays_error_sound_when_appropriate() {
    // GIVEN
    let fx = Fixture::new();
    let mut cmd = fx.cmd(["1"]);

    // WHEN
    // THEN
    assert_cmd_snapshot!(cmd, @r"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    playing error sound
    ");
}

#[test]
fn handles_error_codes_other_than_one() {
    // GIVEN
    let fx = Fixture::new();
    let mut cmd = fx.cmd(["255"]);

    // WHEN
    // THEN
    assert_cmd_snapshot!(cmd, @r"
    success: false
    exit_code: 255
    ----- stdout -----

    ----- stderr -----
    playing error sound
    ");
}
