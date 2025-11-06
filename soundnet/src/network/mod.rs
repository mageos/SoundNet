use crate::jitter_buffer::JitterBuffer;
use crate::network::packet::AudioPacket;
use socket2::{Domain, Protocol, Socket, Type};
use std::net::{Ipv6Addr, SocketAddr, SocketAddrV6};
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::net::UdpSocket;
use tokio::sync::mpsc;

pub mod packet;

const MULTICAST_ADDR: Ipv6Addr = Ipv6Addr::new(0xff02, 0, 0, 0, 0, 0, 0, 0x1234);
const MULTICAST_PORT: u16 = 54321;

pub async fn broadcast(mut rx: mpsc::Receiver<Vec<f32>>) -> Result<(), anyhow::Error> {
    let socket = Socket::new(Domain::IPV6, Type::DGRAM, Some(Protocol::UDP))?;
    socket.set_multicast_hops_v6(255)?;
    socket.bind(&SocketAddr::V6(SocketAddrV6::new(Ipv6Addr::UNSPECIFIED, 0, 0, 0)).into())?;
    let std_socket: std::net::UdpSocket = socket.into();
    let socket = UdpSocket::from_std(std_socket)?;

    while let Some(audio_data) = rx.recv().await {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos() as u64;
        let packet = AudioPacket {
            timestamp,
            audio_data,
        };
        let serialized_packet = bincode::serialize(&packet)?;
        let addr = SocketAddr::V6(SocketAddrV6::new(MULTICAST_ADDR, MULTICAST_PORT, 0, 0));
        socket.send_to(&serialized_packet, &addr).await?;
    }

    Ok(())
}

pub async fn receive(jitter_buffer: Arc<Mutex<JitterBuffer>>) -> Result<(), anyhow::Error> {
    let socket = Socket::new(Domain::IPV6, Type::DGRAM, Some(Protocol::UDP))?;
    socket.set_reuse_address(true)?;
    socket.bind(&SocketAddr::V6(SocketAddrV6::new(Ipv6Addr::UNSPECIFIED, MULTICAST_PORT, 0, 0)).into())?;
    socket.join_multicast_v6(&MULTICAST_ADDR, 0)?;
    let std_socket: std::net::UdpSocket = socket.into();
    let socket = UdpSocket::from_std(std_socket)?;

    let mut buf = [0u8; 1024];
    loop {
        let (len, _addr) = socket.recv_from(&mut buf).await?;
        let packet: AudioPacket = bincode::deserialize(&buf[..len])?;
        jitter_buffer.lock().unwrap().add(packet);
    }
}
