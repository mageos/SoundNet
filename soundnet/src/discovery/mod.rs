use soundnet_types::discovery::{DiscoveryRequest, DiscoveryResponse};
use soundnet_types::DeviceMode;
use std::net::{Ipv6Addr, SocketAddr, SocketAddrV6};
use tokio::net::UdpSocket;
use tracing::{debug, error, info};

const DISCOVERY_MULTICAST_ADDR: Ipv6Addr = Ipv6Addr::new(0xff02, 0, 0, 0, 0, 0, 0, 0x1235);
const DISCOVERY_PORT: u16 = 54322;

pub async fn listen() -> Result<(), anyhow::Error> {
    info!("Starting discovery listener");
    let socket = UdpSocket::bind(SocketAddr::V6(SocketAddrV6::new(
        Ipv6Addr::UNSPECIFIED,
        DISCOVERY_PORT,
        0,
        0,
    )))
    .await?;
    socket.join_multicast_v6(&DISCOVERY_MULTICAST_ADDR, 0)?;
    info!("Listening for discovery requests on {}:{}", DISCOVERY_MULTICAST_ADDR, DISCOVERY_PORT);

    let mut buf = [0u8; 1024];
    loop {
        match socket.recv_from(&mut buf).await {
            Ok((len, addr)) => {
                match bincode::deserialize::<DiscoveryRequest>(&buf[..len]) {
                    Ok(request) => {
                        debug!("Received discovery request from {}: {:?}", addr, request);

                        let response = DiscoveryResponse {
                            friendly_name: "SoundNet Device".to_string(),
                            mode: DeviceMode::Idle, // TODO: get the actual mode
                            api_port: 8080, // TODO: get the actual port
                        };

                        match bincode::serialize(&response) {
                            Ok(serialized_response) => {
                                if let Err(e) = socket.send_to(&serialized_response, &addr).await {
                                    error!("Failed to send discovery response: {}", e);
                                }
                            }
                            Err(e) => {
                                error!("Failed to serialize discovery response: {}", e);
                            }
                        }
                    }
                    Err(e) => {
                        error!("Failed to deserialize discovery request: {}", e);
                    }
                }
            }
            Err(e) => {
                error!("Failed to receive discovery request: {}", e);
            }
        }
    }
}