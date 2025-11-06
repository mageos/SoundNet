use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AudioPacket {
    pub timestamp: u64,
    pub audio_data: Vec<f32>,
}
