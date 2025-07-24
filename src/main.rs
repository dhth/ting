mod args;
mod audio;
mod cmds;
mod config;
mod domain;
mod utils;

use anyhow::Context;
use args::Args;
use clap::Parser;
use config::get_config;
use domain::{PlayArgs, parse_args_and_config};

const TESTING_ENV_VAR: &str = "TING_TESTING";

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let testing = std::env::var(TESTING_ENV_VAR)
        .as_ref()
        .map(|t| t == "1")
        .unwrap_or(false);

    if args.debug {
        print_debug_info(&args);
        return Ok(());
    }

    match args.command {
        args::TingCommand::Play {
            input,
            config_path,
            no_match_exit_code,
        } => {
            let (maybe_exit_code, maybe_cue) = if let Ok(exit_code) = input.parse::<i32>() {
                (Some(exit_code), None)
            } else {
                (None, Some(input))
            };

            let play_args = PlayArgs {
                maybe_cue,
                maybe_exit_code,
                match_exit_code: !no_match_exit_code,
            };
            let maybe_config = get_config(config_path).context("couldn't get config")?;

            let (play_data, play_behaviours) = parse_args_and_config(play_args, maybe_config)?;

            cmds::play(play_data, testing);

            if let Some(exit_code) = maybe_exit_code {
                if play_behaviours.match_exit_code && exit_code != 0 {
                    std::process::exit(exit_code);
                }
            }
        }
        args::TingCommand::Config { config_command } => match config_command {
            args::ConfigCommand::Sample => {
                let default_config_path = config::get_default_config_path()
                    .context("couldn't determine default config path")?;
                print!(
                    r#"# place the following config in "{}":

{}"#,
                    default_config_path.to_string_lossy(),
                    cmds::get_sample_config()
                );
            }
        },
    }

    Ok(())
}

fn print_debug_info(args: &Args) {
    print!(
        r#"DEBUG INFO:
{args}"#
    )
}
