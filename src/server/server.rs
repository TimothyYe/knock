extern crate pnet;

use pnet::datalink::Channel::Ethernet;
use pnet::datalink::{self, NetworkInterface};
use pnet::packet::ethernet::{EtherTypes, EthernetPacket};
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::tcp::TcpPacket;
use pnet::packet::Packet;

pub struct Server {
    interface: String,
}

impl Server {
    pub fn new(interface: String) -> Server {
        Server {
            interface: interface,
        }
    }

    pub fn start(&self) {
        let interface = datalink::interfaces()
            .into_iter()
            .find(|iface: &NetworkInterface| iface.name == self.interface)
            .expect("Failed to get interface");

        // Create a channel to receive on
        let (_, mut rx) = match datalink::channel(&interface, Default::default()) {
            Ok(Ethernet(tx, rx)) => (tx, rx),
            Ok(_) => panic!("Unhandled channel type"),
            Err(e) => panic!(
                "An error occurred when creating the datalink channel: {}",
                e
            ),
        };

        loop {
            match rx.next() {
                Ok(packet) => {
                    let packet = EthernetPacket::new(packet).unwrap();
                    match packet.get_ethertype() {
                        EtherTypes::Ipv4 => {
                            if let Some(header) =
                                pnet::packet::ipv4::Ipv4Packet::new(packet.payload())
                            {
                                if header.get_next_level_protocol() == IpNextHeaderProtocols::Tcp {
                                    if let Some(tcp) = TcpPacket::new(header.payload()) {
                                        // Check for SYN flag and that ACK flag is not set
                                        if tcp.get_flags() & pnet::packet::tcp::TcpFlags::SYN != 0
                                            && tcp.get_flags() & pnet::packet::tcp::TcpFlags::ACK
                                                == 0
                                        {
                                            println!(
                                                "SYN packet detected from {:?} to {:?}",
                                                tcp.get_source(),
                                                tcp.get_destination()
                                            );
                                        }
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
                Err(e) => {
                    panic!("An error occurred while reading: {}", e);
                }
            }
        }
    }
}
