use crate::jitter_buffer::JitterBuffer;
use clap::{Parser, Subcommand};
use figment::providers::Format;
use std::sync::{Arc, Mutex};
use tracing::info;

pub mod audio;
pub mod jitter_buffer;
pub mod network;

/// A low-latency audio streaming server and client for single-board computers.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Path to the configuration file.
    #[arg(short, long, value_name = "FILE", default_value = "soundnet.toml")]
    pub config: String,

    #[command(subcommand)]
    pub mode: Mode,
}

#[derive(Subcommand, Debug)]
pub enum Mode {
    /// Run as a server, broadcasting audio.
    Server,
    /// Run as a client, receiving audio.
    Client {
        /// The size of the jitter buffer in milliseconds.
        #[arg(long, default_value = "20")]
        jitter_buffer_size: u64,
    },
}

pub async fn run() -> Result<(), anyhow::Error> {
    let args = Args::parse();
    info!("SoundNet starting up...");
    info!("Using configuration file: {}", args.config);

    let _config: figment::Figment = figment::Figment::new()
        .merge(figment::providers::Toml::file(&args.config))
        .merge(figment::providers::Env::prefixed("SOUNDNET_"));

    let _state = soundnet_types::SharedState::new();

    match args.mode {
        Mode::Server => {
            let (tx, rx) = tokio::sync::mpsc::channel(1024);
            let capture_handle = std::thread::spawn(move || {
                audio::capture(tx).unwrap();
            });
            let broadcast_handle = tokio::spawn(async move {
                network::broadcast(rx).await.unwrap();
            });
            tokio::try_join!(broadcast_handle)?;
            capture_handle.join().unwrap();
        }
        Mode::Client { jitter_buffer_size } => {
            let jitter_buffer = Arc::new(Mutex::new(JitterBuffer::new(jitter_buffer_size as usize)));
            let jitter_buffer_clone = jitter_buffer.clone();

            let receive_handle = tokio::spawn(async move {
                network::receive(jitter_buffer_clone).await.unwrap();
            });

            let playback_handle = std::thread::spawn(move || {
                audio::playback(jitter_buffer).unwrap();
            });

            tokio::try_join!(receive_handle)?;
            playback_handle.join().unwrap();
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_args_default_config() {
        let args = Args::try_parse_from(&["soundnet", "server"]).unwrap();
        assert_eq!(args.config, "soundnet.toml");
    }

    #[test]
    fn test_cli_args_short_config() {
        let args = Args::try_parse_from(&["soundnet", "-c", "myconfig.toml", "server"]).unwrap();
        assert_eq!(args.config, "myconfig.toml");
    }

    #[test]
    fn test_cli_args_long_config() {
        let args = Args::try_parse_from(&["soundnet", "--config", "myconfig.toml", "server"]).unwrap();
        assert_eq!(args.config, "myconfig.toml");
    }
}