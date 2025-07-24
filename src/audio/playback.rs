use anyhow::Context;
use rodio::{Decoder, OutputStreamBuilder, Sink};
use std::io::Cursor;

pub fn play_audio(sound_bytes: Vec<u8>) -> anyhow::Result<()> {
    let stream_handle = OutputStreamBuilder::open_default_stream()
        .context("couldn't open default output stream")?;
    let sink = Sink::connect_new(stream_handle.mixer());

    let cursor = Cursor::new(sound_bytes);
    let source = Decoder::new(cursor).context("couldn't decode audio bytes")?;

    sink.append(source);
    sink.sleep_until_end();

    Ok(())
}
