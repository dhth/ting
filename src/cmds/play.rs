use crate::audio::play_audio;
use crate::domain::{AudioSource, PlayData, PlayKind};

const SUCCESS_SOUND: &[u8] = include_bytes!("assets/audio/success.wav");
const ERROR_SOUND: &[u8] = include_bytes!("assets/audio/error.wav");

pub fn play(data: PlayData, testing: bool) {
    let audio_bytes = match data.kind {
        PlayKind::ExitCode {
            code,
            success,
            error,
        } => match code {
            0 => match success {
                AudioSource::Builtin => {
                    if testing {
                        eprintln!("playing built-in success sound");
                    }
                    SUCCESS_SOUND.to_vec()
                }
                AudioSource::External(bytes) => {
                    if testing {
                        eprintln!("playing success sound from external source");
                    }
                    bytes
                }
            },
            _ => match error {
                AudioSource::Builtin => {
                    if testing {
                        eprintln!("playing built-in error sound");
                    }
                    ERROR_SOUND.to_vec()
                }
                AudioSource::External(bytes) => {
                    if testing {
                        eprintln!("playing error sound from external source");
                    }
                    bytes
                }
            },
        },
        PlayKind::Cue(bytes) => {
            if testing {
                eprintln!("playing cue sound from external source");
            }
            bytes
        }
    };

    if testing {
        return;
    }

    if let Err(e) = play_audio(audio_bytes) {
        eprintln!("Warning: failed to play sound: {e}");
    }
}
