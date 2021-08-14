pub mod ip_header;
pub mod payload;
pub mod raw;
pub mod sniff;
pub mod tcp;

use std::fmt;

#[derive(Debug)]
pub enum Packet {
    Tcp(tcp::Tcp),
    Unknown(raw::Raw),
}

impl fmt::Display for Packet {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Packet::Tcp(packet) => {
                println!("{}", packet);
                println!("{}", packet.payload);
            }
            Packet::Unknown(_) => println!("Unhandled (Unknown) Package"),
        }

        Ok(())
    }
}
