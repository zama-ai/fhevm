use alloy::transports::http::reqwest::Url;
use clap::{FromArgMatches, Subcommand, command};
use std::path::PathBuf;

/// The CLI shared among the KMS connector's subcomponents.
pub struct Cli {
    /// The name of the subcomponent that is using the CLI.
    pub component_name: &'static str,
}

impl Cli {
    /// Creates the CLI instance using the subcomponent's name.
    pub fn new(component_name: &'static str) -> Self {
        Self { component_name }
    }

    /// Parses the subcommand of the CLI.
    pub fn parse(&self) -> Subcommands {
        let command = command!()
            .name(self.component_name)
            .about(format!("{} CLI", self.component_name));

        let cli = Subcommands::augment_subcommands(command);
        let matches = cli.get_matches();
        Subcommands::from_arg_matches(&matches)
            .map_err(|err| err.exit())
            .unwrap()
    }
}

#[derive(Subcommand)]
pub enum Subcommands {
    /// Start the component
    Start {
        /// Configuration file path (optional if using environment variables)
        #[arg(short, long, value_name = "FILE")]
        config: Option<PathBuf>,
    },

    /// Validate a configuration file
    Validate {
        /// Configuration file path
        #[arg(short, long, value_name = "FILE")]
        config: PathBuf,
    },

    /// Check the health of a running instance of the component
    Health {
        /// Healthcheck endpoint to query (optional if using environment variable instead)
        #[arg(short, long, value_name = "URL")]
        endpoint: Option<Url>,
    },
}
