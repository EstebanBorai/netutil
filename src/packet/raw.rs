use std::mem::transmute;

use super::tcp::Tcp;
use super::Packet;

pub const IPH_ICMP_PROTOCOL: u8 = 1;
pub const IPH_IGMP_PROTOCOL: u8 = 2;
pub const IPH_TCP_PROTOCOL: u8 = 6;
pub const IPH_UDP_PROTOCOL: u8 = 17;
pub const SOCKET_MESSAGE_BUFFER_SIZE: usize = 65536;

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
        let ip_header = unsafe { transmute::<*mut libc::c_void, *mut crate::iphdr>(self.buffer) };
        let protocol = unsafe { (*ip_header).protocol };

        match protocol {
            IPH_TCP_PROTOCOL => {
                let tcp_packet = Tcp::from(self);
                Packet::Tcp(tcp_packet)
            }
            _ => {
                println!("IP Header with Protocol ID: {}", protocol);
                Packet::Unknown(self)
            }
        }
    }
}
