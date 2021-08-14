use libc::{c_void, malloc, recvfrom, sockaddr, socket};
use libc::{AF_INET, IPPROTO_TCP, SOCK_RAW};
use std::mem::size_of;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};

use crate::packet::raw::Raw;
use crate::packet::Packet;

/// Size of the buffer to store packet data received from
/// the socket
pub const SOCKET_MESSAGE_BUFFER_SIZE: usize = 65536;

#[derive(Debug)]
pub struct PacketBuffer(pub(crate) *mut c_void);

unsafe impl Send for PacketBuffer {}

impl PacketBuffer {
    pub fn new(ptr: *mut c_void) -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(PacketBuffer(ptr)))
    }
}

/// Packet counter for different protocols
pub struct PacketCount {
    tcp: u64,
    unknown: u64,
}

/// Packet Sniffer to listen to a socket and log packets received
/// in such socket
pub struct PacketSniffer {
    packet_count: PacketCount,
    packet_tx: Sender<Packet>,
}

impl PacketSniffer {
    /// Creates a new `PacketSniffer` instance with packet
    /// counters set to `0`
    pub fn new() -> (Self, Receiver<Packet>) {
        let (packet_tx, packet_rx) = channel::<Packet>();

        (
            PacketSniffer {
                packet_count: PacketCount { tcp: 0, unknown: 0 },
                packet_tx,
            },
            packet_rx,
        )
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
                let packet_buffer = PacketBuffer::new(packet_buffer);
                let raw_packet = Raw::new(Arc::clone(&packet_buffer), packet_size);
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
            Packet::Tcp(tcp) => {
                self.packet_count.tcp += 1;

                if let Err(err) = self.packet_tx.send(Packet::Tcp(tcp)) {
                    eprintln!("Failed to send TCP package:\nType: TCP\nError: {}", err);
                }
            }
            Packet::Unknown(raw) => {
                self.packet_count.unknown += 1;

                if let Err(err) = self.packet_tx.send(Packet::Unknown(raw)) {
                    eprintln!("Failed to send TCP package:\nType: Unknown\nError: {}", err);
                }
            }
        }
    }
}
