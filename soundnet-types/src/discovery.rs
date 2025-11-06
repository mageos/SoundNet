use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct DiscoveryRequest;

#[derive(Serialize, Deserialize, Debug)]
pub struct DiscoveryResponse {
    pub friendly_name: String,
    pub mode: crate::DeviceMode,
    pub api_port: u16,
}
