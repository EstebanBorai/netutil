use super::ip_header::IpHeader;
use super::tcp::Tcp;
use super::Packet;

pub const IPH_TCP_PROTOCOL: u8 = 6;

/// Raw socket packet received from a socket
/// connection using recvfrom
#[derive(Debug)]
pub struct Raw {
    pub(crate) buffer: *mut libc::c_void,
    pub(crate) size: isize,
}

impl Raw {
    pub fn new(buffer: *mut libc::c_void, size: isize) -> Self {
        Raw { buffer, size }
    }

    pub fn instrospect(self) -> Packet {
        let ip_header = IpHeader::from(&self);

        match ip_header.protocol {
            IPH_TCP_PROTOCOL => {
                let tcp_packet = Tcp::from(self);
                Packet::Tcp(tcp_packet)
            }
            _ => {
                println!("IP Header with Protocol ID: {}", ip_header.protocol);
                Packet::Unknown(self)
            }
        }
    }
}
