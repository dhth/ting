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
    /// Play sound for an input
    #[command(name = "p")]
    Play {
        /// Exit code (0, 1, etc.) or cue name (configured via ting's config)
        input: String,
        /// Path to the config file (overrides ting's default config path)
        #[arg(short = 'C', long = "config-path", value_name = "PATH")]
        config_path: Option<PathBuf>,
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
    Sample,
}

impl std::fmt::Display for Args {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match &self.command {
            TingCommand::Play {
                input,
                config_path,
                no_match_exit_code,
            } => format!(
                r#"
command:                  play sound
flags:
  input:                  {}
  config path:            {}
  don't match exit code:  {}
"#,
                input,
                config_path
                    .as_deref()
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or("<NOT PROVIDED>".to_string()),
                no_match_exit_code,
            ),
            TingCommand::Config { config_command } => match config_command {
                ConfigCommand::Sample => "
command:              print sample config
"
                .to_string(),
            },
        };
        f.write_str(&output)
    }
}
