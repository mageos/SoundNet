use crate::jitter_buffer::JitterBuffer;
use clap::{Parser, Subcommand};
use crossbeam_channel::{unbounded, Sender, Receiver};
use figment::{
    providers::{Format, Toml, Env},
    Figment,
};
use serde::Deserialize;
use soundnet_types::{DeviceMode, SharedState};
use std::sync::{Arc, Mutex};
use tokio::task::JoinHandle;
use tracing::{error, info};

pub mod api;
pub mod audio;
pub mod discovery;
pub mod jitter_buffer;
pub mod network;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub friendly_name: String,
    pub api_port: u16,
    pub jitter_buffer_size: u64,
}

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
        #[arg(long)]
        jitter_buffer_size: Option<u64>,
    },
}

pub struct AppState {
    pub state: Arc<Mutex<SharedState>>,
    pub tasks: Vec<JoinHandle<()>>,
    pub config: Config,
    stop_tx: Sender<()>,
    stop_rx: Receiver<()>,
}

impl AppState {
    pub fn new(state: Arc<Mutex<SharedState>>, config: Config) -> Self {
        let (stop_tx, stop_rx) = unbounded();
        AppState {
            state,
            tasks: Vec::new(),
            config,
            stop_tx,
            stop_rx,
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
                let stop_rx = self.stop_rx.clone();
                let _capture_handle = std::thread::spawn(move || {
                    if let Err(e) = audio::capture(tx, stop_rx) {
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
                let size = jitter_buffer_size.unwrap_or(self.config.jitter_buffer_size);
                let jitter_buffer = Arc::new(Mutex::new(JitterBuffer::new(size as usize)));
                let jitter_buffer_clone = jitter_buffer.clone();

                let receive_handle = tokio::spawn(async move {
                    if let Err(e) = network::receive(jitter_buffer_clone).await {
                        error!("Network receive error: {}", e);
                    }
                });

                let stop_rx = self.stop_rx.clone();
                let _playback_handle = {
                    let state = self.state.clone();
                    std::thread::spawn(move || {
                        if let Err(e) = audio::playback(jitter_buffer, state, stop_rx) {
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
        self.stop_tx.send(()).ok();
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

    let config: Config = Figment::new()
        .merge(Toml::file(&args.config))
        .merge(Env::prefixed("SOUNDNET_"))
        .extract()?;

    let state = SharedState::new();
    state.lock().unwrap().friendly_name = config.friendly_name.clone();
    state.lock().unwrap().api_port = config.api_port;

    let app_state = Arc::new(Mutex::new(AppState::new(state.clone(), config)));

    let discovery_handle = tokio::spawn({
        let state = state.clone();
        async move {
            if let Err(e) = discovery::listen(state).await {
                error!("Discovery listener error: {}", e);
            }
        }
    });

    let api_handle = tokio::spawn({
        let app_state = app_state.clone();
        async move {
            api::run(app_state).await.unwrap();
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
