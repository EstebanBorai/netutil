use netutil::packet::{self, Packet};

/// Execute this program piping the output to a file for a better
/// inspection on packages.
///
/// Example: cargo build && sudo ./target/debug/packet_sniffer > info.log
fn main() {
    // Create an instance of a `PacketSniffer` and retrieve a receiver
    // end from the messages channel
    let (mut sniffer, packet_rx) = packet::sniff::PacketSniffer::new();

    // Make sure the user knows the packet sniffer is starting to work
    println!("Initializing Packet Sniffer...");

    // Spawn a thread to listen to packets from a socket and send them
    // through the messages channel
    std::thread::spawn(move || {
        sniffer.sniff();
    });

    // Receive incoming packages from the channel receiver end and log
    // them accordingly to stdout
    loop {
        if let Ok(packet) = packet_rx.try_recv() {
            match packet {
                Packet::Tcp(packet) => {
                    println!("{}", packet);
                    println!("{}", packet.payload);
                }
                Packet::Unknown(_) => println!("Unhandled (Unknown) Package"),
            }
        }
    }
}
