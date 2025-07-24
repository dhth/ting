use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: TingCommand,
    /// Output debug information without doing anything
    #[arg(long = "debug", global = true)]
    pub debug: bool,
}

#[derive(Subcommand, Debug)]
pub enum TingCommand {
    /// Play sound
    Play {
        /// Path to the config file (overrides ting's default config path)
        #[arg(short = 'C', long = "config-path", value_name = "PATH")]
        maybe_config_path: Option<PathBuf>,
        /// Cue to play sound for (configured via ting's config file)
        #[arg(short = 'c', long = "cue", value_name = "STRING")]
        maybe_cue: Option<String>,
        /// Play sound based on exit code (0=success, non-zero=error)
        #[arg(short = 'e', long = "exit-code", value_name = "EXIT CODE")]
        maybe_exit_code: Option<i32>,
        /// Don't exit ting with the same code as the input
        #[arg(long = "no-match-exit-code")]
        no_match_exit_code: bool,
    },
    /// Interact with ting's config
    Config {
        #[command(subcommand)]
        config_command: ConfigCommand,
    },
}

#[derive(Subcommand, Debug)]
pub enum ConfigCommand {
    /// Print sample config for ting
    Sample {},
}

impl std::fmt::Display for Args {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match &self.command {
            TingCommand::Play {
                maybe_config_path,
                maybe_cue,
                maybe_exit_code,
                no_match_exit_code,
            } => format!(
                r#"
command:                  play sound
flags:
  config path:            {:?}
  cue:                    {:?}
  exit code:              {:?}
  don't match exit code:  {}
"#,
                &maybe_config_path, &maybe_cue, &maybe_exit_code, no_match_exit_code,
            ),
            TingCommand::Config { config_command } => match config_command {
                ConfigCommand::Sample {} => "
command:              print sample config
"
                .to_string(),
            },
        };
        f.write_str(&output)
    }
}
