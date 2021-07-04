use std::net::Ipv4Addr;

/// Parse IPv4 address from a u32 integer taking presence of the CPU
/// endinaness to avoid having reversed IP address being parsed
pub fn parse_ipv4_address(decimal: u32) -> Ipv4Addr {
    if cfg!(target_endian = "little") {
        // due to a difference on how bytes are arranged on a
        // single word of memory by the CPU, swap bytes based
        // on CPU endianess to avoid having twisted IP addresses
        //
        // refer: https://github.com/rust-lang/rust/issues/48819
        return Ipv4Addr::from(decimal.swap_bytes());
    }

    Ipv4Addr::from(decimal)
}

/// Convert between host and network byte order
pub fn ntohs(u: u16) -> u16 {
    u16::from_be(u)
}
