use std::fmt;
use std::mem::transmute;
use std::sync::{Arc, Mutex};

use libc::c_void;

use crate::utils::ntohs;

use super::payload::Payload;
use super::raw::Raw;
use super::sniff::PacketBuffer;

#[derive(Debug)]
pub struct Tcp {
    pub source_port: u16,
    pub destination_port: u16,
    pub sequence_number: u32,
    pub urgent_flag: u16,
    pub acknowledgement_flag: u16,
    pub push_flag: u16,
    pub reset_flag: u16,
    pub synchronise_flag: u16,
    pub finish_flag: u16,
    pub window: u16,
    pub checksum: u16,
    pub urgent_ptr: u16,
    pub payload: String,
}

impl fmt::Display for Tcp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            r#"
TCP Packet
------------------------------------
Source Port: {source_port}
Destination Port: {destination_port}
Sequence Number: {sequence_number}
Urgent Flag: {urgent_flag}
Acknowledgement Flag: {acknowledgement_flag}
Push Flag: {push_flag}
Reset Flag: {reset_flag}
Synchronise Flag: {synchronise_flag}
Finish Flag: {finish_flag}
Window: {window}
Checksum: {checksum}
Urgent Pointer: {urgent_ptr}
------------------------------------
"#,
            source_port = self.source_port,
            destination_port = self.destination_port,
            sequence_number = self.sequence_number,
            urgent_flag = self.urgent_flag,
            acknowledgement_flag = self.acknowledgement_flag,
            push_flag = self.push_flag,
            reset_flag = self.reset_flag,
            synchronise_flag = self.synchronise_flag,
            finish_flag = self.finish_flag,
            window = self.window,
            checksum = self.checksum,
            urgent_ptr = self.urgent_ptr,
        )
    }
}

impl From<Raw> for Tcp {
    fn from(raw: Raw) -> Self {
        let raw_buffer = raw.buffer.lock().unwrap().0;
        let payload = Payload::from(raw_buffer as *mut libc::c_char);
        let tcp_header = unsafe {
            let tcp_buffer = transmute::<*mut libc::c_void, *mut crate::tcphdr>(raw_buffer);

            (*tcp_buffer).__bindgen_anon_1.__bindgen_anon_2
        };

        let source_port = tcp_header.source;
        let destination_port = tcp_header.dest;
        let sequence_number = tcp_header.seq;
        let urgent_flag = tcp_header.urg();
        let acknowledgement_flag = tcp_header.ack();
        let push_flag = tcp_header.psh();
        let reset_flag = tcp_header.rst();
        let synchronise_flag = tcp_header.syn();
        let finish_flag = tcp_header.fin();
        let window = ntohs(tcp_header.window);
        let checksum = ntohs(tcp_header.check);
        let urgent_ptr = tcp_header.urg_ptr;

        Tcp {
            source_port,
            destination_port,
            sequence_number,
            urgent_flag,
            acknowledgement_flag,
            push_flag,
            reset_flag,
            synchronise_flag,
            finish_flag,
            window,
            checksum,
            urgent_ptr,
            payload: payload.digest((tcp_header.doff() * 4) as usize),
        }
    }
}
