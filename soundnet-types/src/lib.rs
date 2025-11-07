use serde::{Serialize, Deserialize};
use std::sync::{Arc, Mutex};

pub mod discovery;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum DeviceMode {
    Idle,
    Server,
    Client,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AudioFormat {
    pub codec: String,
    pub sample_rate: u32,
    pub bitrate: u32,
    pub volume: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedState {
    pub friendly_name: String,
    pub mode: DeviceMode,
    pub format: AudioFormat,
    pub api_port: u16,
}

impl SharedState {
    pub fn new() -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(Self {
            friendly_name: "SoundNet Device".to_string(),
            mode: DeviceMode::Idle,
            format: AudioFormat {
                codec: "opus".to_string(),
                sample_rate: 48000,
                bitrate: 64000,
                volume: 1.0,
            },
            api_port: 8080,
        }))
    }
}