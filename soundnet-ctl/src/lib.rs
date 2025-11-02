use clap::{Parser, Subcommand};
use tracing::info;

#[derive(Parser, Debug)]
#[command(author, version, about = "A command-line utility to control SoundNet devices.", long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug, PartialEq)]
pub enum Commands {
    /// Discover SoundNet devices on the local network.
    Discover,
}

pub async fn run() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Discover => {
            info!("Broadcasting discovery request...");
            // Discovery logic will be implemented in Task 5.3.
            println!("\nSearching for SoundNet devices on the network...");
            println!("(Discovery logic not yet implemented)");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_discover_command() {
        let cli = Cli::try_parse_from(&["soundnet-ctl", "discover"]).unwrap();
        assert_eq!(cli.command, Commands::Discover);
    }

    #[test]
    fn test_cli_no_command_fails() {
        let result = Cli::try_parse_from(&["soundnet-ctl"]);
        assert!(result.is_err());
    }
}