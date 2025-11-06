use crate::jitter_buffer::JitterBuffer;
use crate::network::packet::AudioPacket;
use socket2::{Domain, Protocol, Socket, Type};
use std::net::{Ipv6Addr, SocketAddr, SocketAddrV6};
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::net::UdpSocket;
use tokio::sync::mpsc;
use tracing::{debug, error, info};

pub mod packet;

const MULTICAST_ADDR: Ipv6Addr = Ipv6Addr::new(0xff02, 0, 0, 0, 0, 0, 0, 0x1234);
const MULTICAST_PORT: u16 = 54321;

pub async fn broadcast(mut rx: mpsc::Receiver<Vec<f32>>) -> Result<(), anyhow::Error> {
    info!("Starting network broadcast");
    let socket = Socket::new(Domain::IPV6, Type::DGRAM, Some(Protocol::UDP))?;
    socket.set_multicast_hops_v6(255)?;
    socket.bind(&SocketAddr::V6(SocketAddrV6::new(Ipv6Addr::UNSPECIFIED, 0, 0, 0)).into())?;
    let std_socket: std::net::UdpSocket = socket.into();
    let socket = UdpSocket::from_std(std_socket)?;
    info!("Broadcasting to {}:{}", MULTICAST_ADDR, MULTICAST_PORT);

    while let Some(audio_data) = rx.recv().await {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos() as u64;
        let packet = AudioPacket {
            timestamp,
            audio_data,
        };
        let serialized_packet = bincode::serialize(&packet)?;
        let addr = SocketAddr::V6(SocketAddrV6::new(MULTICAST_ADDR, MULTICAST_PORT, 0, 0));
        if let Err(e) = socket.send_to(&serialized_packet, &addr).await {
            error!("Failed to send packet: {}", e);
        }
    }

    Ok(())
}

pub async fn receive(jitter_buffer: Arc<Mutex<JitterBuffer>>) -> Result<(), anyhow::Error> {
    info!("Starting network receive");
    let socket = Socket::new(Domain::IPV6, Type::DGRAM, Some(Protocol::UDP))?;
    socket.set_reuse_address(true)?;
    socket.bind(&SocketAddr::V6(SocketAddrV6::new(Ipv6Addr::UNSPECIFIED, MULTICAST_PORT, 0, 0)).into())?;
    socket.join_multicast_v6(&MULTICAST_ADDR, 0)?;
    let std_socket: std::net::UdpSocket = socket.into();
    let socket = UdpSocket::from_std(std_socket)?;
    info!("Listening on {}:{}", MULTICAST_ADDR, MULTICAST_PORT);

    let mut buf = [0u8; 1024];
    loop {
        match socket.recv_from(&mut buf).await {
            Ok((len, _addr)) => {
                match bincode::deserialize::<AudioPacket>(&buf[..len]) {
                    Ok(packet) => {
                        debug!("Received packet with timestamp: {}", packet.timestamp);
                        jitter_buffer.lock().unwrap().add(packet);
                    }
                    Err(e) => {
                        error!("Failed to deserialize packet: {}", e);
                    }
                }
            }
            Err(e) => {
                error!("Failed to receive packet: {}", e);
            }
        }
    }
}