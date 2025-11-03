use socket2::{Domain, Protocol, Socket, Type};
use std::net::{Ipv6Addr, SocketAddr, SocketAddrV6};
use tokio::net::UdpSocket;

const MULTICAST_ADDR: Ipv6Addr = Ipv6Addr::new(0xff02, 0, 0, 0, 0, 0, 0, 0x1234);
const MULTICAST_PORT: u16 = 54321;

pub async fn broadcast() -> Result<(), anyhow::Error> {
    let socket = Socket::new(Domain::IPV6, Type::DGRAM, Some(Protocol::UDP))?;
    socket.set_multicast_hops_v6(255)?;
    socket.bind(&SocketAddr::V6(SocketAddrV6::new(Ipv6Addr::UNSPECIFIED, 0, 0, 0)).into())?;
    let std_socket: std::net::UdpSocket = socket.into();
    let socket = UdpSocket::from_std(std_socket)?;

    let mut i = 0;
    loop {
        let packet = format!("hello {}", i);
        let addr = SocketAddr::V6(SocketAddrV6::new(MULTICAST_ADDR, MULTICAST_PORT, 0, 0));
        socket.send_to(packet.as_bytes(), &addr).await?;
        i += 1;
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
}
