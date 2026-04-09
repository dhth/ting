use anyhow::Context;
use rodio::{Decoder, DeviceSinkBuilder, Player};
use std::io::Cursor;

pub fn play_audio(sound_bytes: Vec<u8>) -> anyhow::Result<()> {
    let sink =
        DeviceSinkBuilder::open_default_sink().context("couldn't open default output stream")?;
    let player = Player::connect_new(sink.mixer());

    let cursor = Cursor::new(sound_bytes);
    let source = Decoder::new(cursor).context("couldn't decode audio bytes")?;

    player.append(source);
    player.sleep_until_end();

    Ok(())
}
