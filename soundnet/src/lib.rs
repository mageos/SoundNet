use clap::Parser;
use figment::providers::Format;
use tracing::info;

pub mod audio;
pub mod network;

/// A low-latency audio streaming server and client for single-board computers.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Path to the configuration file.
    #[arg(short, long, value_name = "FILE", default_value = "soundnet.toml")]
    pub config: String,
}

pub async fn run() -> Result<(), anyhow::Error> {
    let args = Args::parse();
    info!("SoundNet starting up...");
    info!("Using configuration file: {}", args.config);

    let _config: figment::Figment = figment::Figment::new()
        .merge(figment::providers::Toml::file(&args.config))
        .merge(figment::providers::Env::prefixed("SOUNDNET_"));

    let _state = soundnet_types::SharedState::new();

    // let (tx, rx) = tokio::sync::mpsc::channel(1024);

    // let capture_handle = std::thread::spawn(move || {
    //     audio::capture(tx).unwrap();
    // });

    // let playback_handle = std::thread::spawn(move || {
    //     audio::playback(rx).unwrap();
    // });

    // capture_handle.join().unwrap();
    // playback_handle.join().unwrap();

    let broadcast_handle = tokio::spawn(async move {
        network::broadcast().await.unwrap();
    });

    tokio::try_join!(broadcast_handle)?;

    Ok(())
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
