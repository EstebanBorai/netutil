use std::mem::transmute;
use std::net::Ipv4Addr;

use crate::utils::parse_ipv4_address;

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
}

impl From<&Raw> for IpHeader {
    fn from(raw: &Raw) -> Self {
        let ip_header = unsafe { transmute::<*mut libc::c_void, *mut crate::iphdr>(raw.buffer) };
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
        }
    }
}
