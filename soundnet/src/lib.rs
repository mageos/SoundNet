use clap::Parser;
use tracing::info;

/// A low-latency audio streaming server and client for single-board computers.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Path to the configuration file.
    #[arg(short, long, value_name = "FILE", default_value = "soundnet.toml")]
    pub config: String,
}

pub async fn run() {
    let args = Args::parse();
    info!("SoundNet starting up...");
    info!("Using configuration file: {}", args.config);

    let config: figment::Figment = figment::Figment::new()
        .with(figment::providers::Format::Toml, &args.config)
        .with(figment::providers::Env::prefixed("SOUNDNET_"));

    // The rest of the application logic will go here.
    // For now, we just idle until Ctrl-C is pressed.
    info!("Initialization complete. Idling.");
    tokio::signal::ctrl_c()
        .await
        .expect("failed to listen for ctrl-c");
    info!("Shutting down.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_args_default_config() {
        let args = Args::try_parse_from(&["soundnet"]).unwrap();
        assert_eq!(args.config, "soundnet.toml");
    }

    #[test]
    fn test_cli_args_short_config() {
        let args = Args::try_parse_from(&["soundnet", "-c", "myconfig.toml"]).unwrap();
        assert_eq!(args.config, "myconfig.toml");
    }

    #[test]
    fn test_cli_args_long_config() {
        let args = Args::try_parse_from(&["soundnet", "--config", "myconfig.toml"]).unwrap();
        assert_eq!(args.config, "myconfig.toml");
    }
}