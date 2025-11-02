use serde::{Serialize, Deserialize};
use std::sync::{Arc, Mutex};

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
}

#[derive(Debug, Clone)]
pub struct SharedState {
    pub mode: DeviceMode,
    pub format: AudioFormat,
}

impl SharedState {
    pub fn new() -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(Self {
            mode: DeviceMode::Idle,
            format: AudioFormat {
                codec: "opus".to_string(),
                sample_rate: 48000,
                bitrate: 64000,
            },
        }))
    }
}