use anyhow::Context;
use rodio::{Decoder, OutputStreamBuilder, Sink};
use std::{io::Cursor, process::Command};

const SUCCESS_SOUND: &[u8] = include_bytes!("assets/success.wav");
const ERROR_SOUND: &[u8] = include_bytes!("assets/error.wav");

fn main() -> anyhow::Result<()> {
    let exit_code = get_exit_code_of_last_cmd()?;
    let sound_bytes = if exit_code == 0 {
        SUCCESS_SOUND
    } else {
        ERROR_SOUND
    };

    play_sound(sound_bytes)?;

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

fn get_exit_code_of_last_cmd() -> anyhow::Result<u8> {
    let output = Command::new("bash")
        .args(["-c", "echo $?"])
        .output()
        .context("couldn't run echo command")?;

    let out =
        String::from_utf8(output.stdout).context("couldn't convert command output to string")?;

    let code: u8 = out
        .trim()
        .parse()
        .context("couldn't parse output to a u8")?;

    Ok(code)
}
