use soundnet_types::discovery::{DiscoveryRequest, DiscoveryResponse};
use soundnet_types::DeviceMode;
use std::net::{Ipv6Addr, SocketAddr, SocketAddrV6};
use tokio::net::UdpSocket;

const DISCOVERY_MULTICAST_ADDR: Ipv6Addr = Ipv6Addr::new(0xff02, 0, 0, 0, 0, 0, 0, 0x1235);
const DISCOVERY_PORT: u16 = 54322;

pub async fn listen() -> Result<(), anyhow::Error> {
    let socket = UdpSocket::bind(SocketAddr::V6(SocketAddrV6::new(
        Ipv6Addr::UNSPECIFIED,
        DISCOVERY_PORT,
        0,
        0,
    )))
    .await?;
    socket.join_multicast_v6(&DISCOVERY_MULTICAST_ADDR, 0)?;

    let mut buf = [0u8; 1024];
    loop {
        let (len, addr) = socket.recv_from(&mut buf).await?;
        let request: DiscoveryRequest = bincode::deserialize(&buf[..len])?;

        // For now, we just print the request. We will respond to it later.
        println!("received discovery request from {}: {:?}", addr, request);

        let response = DiscoveryResponse {
            friendly_name: "SoundNet Device".to_string(),
            mode: DeviceMode::Idle, // TODO: get the actual mode
            api_port: 8080, // TODO: get the actual port
        };

        let serialized_response = bincode::serialize(&response)?;
        socket.send_to(&serialized_response, &addr).await?;
    }
}
