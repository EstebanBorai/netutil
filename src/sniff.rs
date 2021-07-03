use libc::{malloc, recvfrom, sockaddr, socket};
use libc::{AF_INET, IPPROTO_TCP, SOCK_RAW};
use std::mem::size_of;

use crate::packet::raw::Raw;
use crate::packet::Packet;

/// Size of the buffer to store packet data received from
/// the socket
pub const SOCKET_MESSAGE_BUFFER_SIZE: usize = 65536;

/// Packet counter for different protocols
pub struct PacketCount {
    tcp: u64,
    unknown: u64,
}

/// Packet Sniffer to listen to a socket and log packets received
/// in such socket
pub struct PacketSniffer {
    packet_count: PacketCount,
}

impl PacketSniffer {
    /// Creates a new `PacketSniffer` instance with packet
    /// counters set to `0`
    pub fn new() -> Self {
        PacketSniffer {
            packet_count: PacketCount { tcp: 0, unknown: 0 },
        }
    }

    /// Plugs into a socket and begins with the packet listening
    pub fn sniff(&mut self) {
        let socket_addr_size = unsafe { malloc(size_of::<u32>()) as *mut u32 };
        let socket_addr = unsafe { malloc(size_of::<sockaddr>()) as *mut sockaddr };
        let packet_buffer = unsafe { malloc(SOCKET_MESSAGE_BUFFER_SIZE) };

        // Connects a socket in the AF_INET domain using the TCP protocol
        let afinet_tcp_socket = unsafe { socket(AF_INET, SOCK_RAW, IPPROTO_TCP) };

        if afinet_tcp_socket < 0 {
            panic!("Socket connection error");
        }

        loop {
            let packet_size: isize;

            unsafe {
                // Receive a message from a socket and retrieve total bytes received in the socket_out_len
                // variable
                packet_size = recvfrom(
                    afinet_tcp_socket,
                    packet_buffer,
                    SOCKET_MESSAGE_BUFFER_SIZE,
                    0,
                    socket_addr,
                    socket_addr_size,
                );
            }

            if packet_size > 0 {
                let raw_packet = Raw::new(packet_buffer, packet_size);
                let packet = raw_packet.instrospect();

                self.handle_packet(packet);
            }
        }

        // Would be nice to have a Ctrl+C handler to gracefully
        // shutdown the sniffer by breaking the loop and then close
        // the socket connection as follows
        // unsafe { libc::close(afinet_tcp_socket) };
    }

    fn handle_packet(&mut self, packet: Packet) {
        match packet {
            Packet::Tcp(_tcp) => {
                self.packet_count.tcp += 1;
            }
            Packet::Unknown(_) => {
                self.packet_count.unknown += 1;
            }
        }
    }
}
