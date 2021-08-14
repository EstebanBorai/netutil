use std::sync::{Arc, Mutex};

use super::ip_header::IpHeader;
use super::sniff::PacketBuffer;
use super::tcp::Tcp;
use super::Packet;

pub const IPH_TCP_PROTOCOL: u8 = 6;

/// Raw socket packet received from a socket
/// connection using recvfrom
#[derive(Debug)]
pub struct Raw {
    pub(crate) buffer: Arc<Mutex<PacketBuffer>>,
    pub(crate) size: isize,
}

impl Raw {
    pub fn new(buffer: Arc<Mutex<PacketBuffer>>, size: isize) -> Self {
        Raw { buffer, size }
    }

    pub fn instrospect(self) -> Packet {
        let ip_header = IpHeader::from(&self);

        match ip_header.protocol {
            IPH_TCP_PROTOCOL => {
                let tcp_packet = Tcp::from(self);
                Packet::Tcp(tcp_packet)
            }
            _ => Packet::Unknown(self),
        }
    }
}
