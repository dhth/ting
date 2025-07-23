use anyhow::Context;
use rodio::{Decoder, OutputStreamBuilder, Sink};
use std::{env, io::Cursor, process};

const SUCCESS_SOUND: &[u8] = include_bytes!("assets/success.wav");
const ERROR_SOUND: &[u8] = include_bytes!("assets/error.wav");
const TESTING_ENV_VAR: &str = "TING_TESTING";

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    let testing = std::env::var(TESTING_ENV_VAR)
        .unwrap_or("0".to_string())
        .as_str()
        == "1";

    if args.len() < 2 {
        print_help();
        return Ok(());
    }

    if args[1] == "--help" || args[1] == "-h" {
        print_help();
        return Ok(());
    }

    let exit_code: i32 = args[1].parse().context("couldn't parse exit code")?;

    if testing {
        if exit_code == 0 {
            eprintln!("playing success sound")
        } else {
            eprintln!("playing error sound")
        }
    } else {
        let sound_bytes = if exit_code == 0 {
            SUCCESS_SOUND
        } else {
            ERROR_SOUND
        };

        if let Err(e) = play_sound(sound_bytes) {
            eprintln!("Warning: failed to play sound: {e}");
        }
    }

    if exit_code != 0 {
        process::exit(exit_code);
    }

    Ok(())
}

fn play_sound(sound_bytes: &'static [u8]) -> anyhow::Result<()> {
    let stream_handle = OutputStreamBuilder::open_default_stream()
        .context("couldn't open default output stream")?;
    let sink = Sink::connect_new(stream_handle.mixer());

    let cursor = Cursor::new(sound_bytes);
    let source = Decoder::new(cursor).context("couldn't decode audio bytes")?;

    sink.append(source);
    sink.sleep_until_end();

    Ok(())
}

fn print_help() {
    print!(
        "ting - audio feedback for command exit codes

USAGE: ting <EXIT_CODE>

ARGUMENTS:
  <EXIT_CODE>  The exit code from the previous command

OPTIONS:
  -h, --help              Print help

EXAMPLES:
  cargo check; ting $?

  alias t='ting $?'
  cargo build; t
"
    );
}
