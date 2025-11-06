use crate::jitter_buffer::JitterBuffer;
use clap::{Parser, Subcommand};
use figment::providers::Format;
use soundnet_types::{DeviceMode, SharedState};
use std::sync::{Arc, Mutex};
use tokio::task::JoinHandle;
use tracing::{error, info};

pub mod api;
pub mod audio;
pub mod discovery;
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

#[derive(Subcommand, Debug, Clone)]
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

pub struct AppState {
    pub state: Arc<Mutex<SharedState>>,
    pub tasks: Vec<JoinHandle<()>>,
}

impl AppState {
    pub fn new(state: Arc<Mutex<SharedState>>) -> Self {
        AppState {
            state,
            tasks: Vec::new(),
        }
    }

    pub fn start_tasks(&mut self, mode: &Mode) {
        self.stop_tasks();
        info!("Starting tasks for mode: {:?}", mode);

        let new_mode = match mode {
            Mode::Server => DeviceMode::Server,
            Mode::Client { .. } => DeviceMode::Client,
        };

        match mode {
            Mode::Server => {
                let (tx, rx) = tokio::sync::mpsc::channel(1024);
                let _capture_handle = std::thread::spawn(move || {
                    if let Err(e) = audio::capture(tx) {
                        error!("Audio capture error: {}", e);
                    }
                });
                let broadcast_handle = tokio::spawn(async move {
                    if let Err(e) = network::broadcast(rx).await {
                        error!("Network broadcast error: {}", e);
                    }
                });
                self.tasks.push(broadcast_handle);
            }
            Mode::Client { jitter_buffer_size } => {
                let jitter_buffer = Arc::new(Mutex::new(JitterBuffer::new(*jitter_buffer_size as usize)));
                let jitter_buffer_clone = jitter_buffer.clone();

                let receive_handle = tokio::spawn(async move {
                    if let Err(e) = network::receive(jitter_buffer_clone).await {
                        error!("Network receive error: {}", e);
                    }
                });

                let _playback_handle = {
                    let state = self.state.clone();
                    std::thread::spawn(move || {
                        if let Err(e) = audio::playback(jitter_buffer, state) {
                            error!("Audio playback error: {}", e);
                        }
                    })
                };

                self.tasks.push(receive_handle);
            }
        }
        self.state.lock().unwrap().mode = new_mode;
    }

    pub fn stop_tasks(&mut self) {
        info!("Stopping tasks");
        for task in &self.tasks {
            task.abort();
        }
        self.tasks.clear();
        self.state.lock().unwrap().mode = DeviceMode::Idle;
    }
}

pub async fn run() -> Result<(), anyhow::Error> {
    let args = Args::parse();
    info!("SoundNet starting up...");
    info!("Using configuration file: {}", args.config);

    let _config: figment::Figment = figment::Figment::new()
        .merge(figment::providers::Toml::file(&args.config))
        .merge(figment::providers::Env::prefixed("SOUNDNET_"));

    let state = SharedState::new();
    let app_state = Arc::new(Mutex::new(AppState::new(state.clone())));

    let discovery_handle = tokio::spawn(async move {
        if let Err(e) = discovery::listen().await {
            error!("Discovery listener error: {}", e);
        }
    });

    let api_handle = tokio::spawn({
        let app_state = app_state.clone();
        async move {
            if let Err(e) = api::run(app_state).await {
                error!("API server error: {}", e);
            }
        }
    });

    app_state.lock().unwrap().start_tasks(&args.mode);

    tokio::try_join!(discovery_handle, api_handle)?;

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