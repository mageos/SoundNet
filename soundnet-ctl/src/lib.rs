use clap::{Parser, Subcommand};
use socket2::{Domain, Protocol, Socket, Type};
use soundnet_types::discovery::{DiscoveryRequest, DiscoveryResponse};
use std::net::{Ipv6Addr, SocketAddr, SocketAddrV6};
use tokio::net::UdpSocket;
use tracing::info;

const DISCOVERY_MULTICAST_ADDR: Ipv6Addr = Ipv6Addr::new(0xff02, 0, 0, 0, 0, 0, 0, 0x1235);
const DISCOVERY_PORT: u16 = 54322;

#[derive(Parser, Debug)]
#[command(author, version, about = "A command-line utility to control SoundNet devices.", long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug, PartialEq)]
pub enum Commands {
    /// Discover SoundNet devices on the local network.
    Discover,
}

pub async fn run() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Discover => {
            info!("Broadcasting discovery request...");
            let socket = Socket::new(Domain::IPV6, Type::DGRAM, Some(Protocol::UDP))?;
            socket.set_multicast_hops_v6(255)?;
            socket.bind(&SocketAddr::V6(SocketAddrV6::new(Ipv6Addr::UNSPECIFIED, 0, 0, 0)).into())?;
            let std_socket: std::net::UdpSocket = socket.into();
            let socket = UdpSocket::from_std(std_socket)?;

            let request = DiscoveryRequest;
            let serialized_request = bincode::serialize(&request)?;
            let addr = SocketAddr::V6(SocketAddrV6::new(
                DISCOVERY_MULTICAST_ADDR,
                DISCOVERY_PORT,
                0,
                0,
            ));
            socket.send_to(&serialized_request, &addr).await?;

            println!("\nSearching for SoundNet devices on the network...");

            let mut buf = [0u8; 1024];
            loop {
                let (len, addr) = socket.recv_from(&mut buf).await?;
                let response: DiscoveryResponse = bincode::deserialize(&buf[..len])?;
                println!(
                    "Discovered device: {} at {} (mode: {:?}, api_port: {})",
                    response.friendly_name,
                    addr,
                    response.mode,
                    response.api_port
                );
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_discover_command() {
        let cli = Cli::try_parse_from(&["soundnet-ctl", "discover"]).unwrap();
        assert_eq!(cli.command, Commands::Discover);
    }

    #[test]
    fn test_cli_no_command_fails() {
        let result = Cli::try_parse_from(&["soundnet-ctl"]);
        assert!(result.is_err());
    }
}