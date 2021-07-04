use std::fmt;
use std::mem::transmute;
use std::net::Ipv4Addr;

use crate::utils::parse_ipv4_address;
use crate::packet::payload::Payload;

use super::raw::Raw;

#[derive(Debug)]
pub struct IpHeader {
    pub id: u16,
    pub ttl: u8,
    pub protocol: u8,
    pub version: u32,
    pub dwords_len: u32,
    pub bytes_len: u32,
    pub type_of_service: u8,
    pub total_len: u16,
    pub checksum: u16,
    pub source_ip: Ipv4Addr,
    pub destination_ip: Ipv4Addr,
    pub payload: String,
}

impl fmt::Display for IpHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            r#"
IP Header
------------------------------------
ID: {id}
TTL: {ttl}
Protocol: {protocol}
Version: {version}
DWORDS: {dwords_len}
BYTES: {bytes_len}
Type Of Service: {type_of_service}
Total Len: {total_len}
Checksum: {checksum}
Source IP: {source_ip}
Destination IP: {destination_ip}
------------------------------------
"#,
            id = self.id,
            ttl = self.ttl,
            protocol = self.protocol,
            version = self.version,
            dwords_len = self.dwords_len,
            bytes_len = self.bytes_len,
            type_of_service = self.type_of_service,
            total_len = self.total_len,
            checksum = self.checksum,
            source_ip = self.source_ip,
            destination_ip = self.destination_ip,
        )
    }
}

impl From<&Raw> for IpHeader {
    fn from(raw: &Raw) -> Self {
        let ip_header = unsafe { transmute::<*mut libc::c_void, *mut crate::iphdr>(raw.buffer) };
        let ip_header_len = unsafe { ((*ip_header).ihl() * 4) as usize };
        let protocol = unsafe { (*ip_header).protocol };
        let version = unsafe { (*ip_header).version() };
        let dwords_len = unsafe { (*ip_header).ihl() };
        let bytes_len = dwords_len * 4;
        let type_of_service = unsafe { (*ip_header).tos };
        let total_len = unsafe { (*ip_header).tot_len };
        let id = unsafe { (*ip_header).id };
        let ttl = unsafe { (*ip_header).ttl };
        let checksum = unsafe { (*ip_header).check };
        let source_ip = unsafe { parse_ipv4_address((*ip_header).saddr) };
        let destination_ip = unsafe { parse_ipv4_address((*ip_header).daddr) };
        let payload = Payload::from(raw.buffer as *mut i8);

        IpHeader {
            id,
            ttl,
            protocol,
            version,
            dwords_len,
            bytes_len,
            type_of_service,
            total_len,
            checksum,
            source_ip,
            destination_ip,
            payload: payload.digest(ip_header_len),
        }
    }
}
