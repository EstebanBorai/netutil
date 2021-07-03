use std::mem::transmute;

use super::raw::Raw;

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
}

impl From<Raw> for Tcp {
    fn from(raw: Raw) -> Self {
        let tcp_header = unsafe {
            let tcp_buffer = raw.buffer;

            transmute::<*mut libc::c_void, *mut crate::tcphdr>(tcp_buffer)
        };

        let source_port = unsafe { (*tcp_header).__bindgen_anon_1.__bindgen_anon_2.source };
        let destination_port = unsafe { (*tcp_header).__bindgen_anon_1.__bindgen_anon_2.dest };
        let sequence_number = unsafe { (*tcp_header).__bindgen_anon_1.__bindgen_anon_2.seq };
        let urgent_flag = unsafe { (*tcp_header).__bindgen_anon_1.__bindgen_anon_2.urg() };
        let acknowledgement_flag = unsafe { (*tcp_header).__bindgen_anon_1.__bindgen_anon_2.ack() };
        let push_flag = unsafe { (*tcp_header).__bindgen_anon_1.__bindgen_anon_2.psh() };
        let reset_flag = unsafe { (*tcp_header).__bindgen_anon_1.__bindgen_anon_2.rst() };
        let synchronise_flag = unsafe { (*tcp_header).__bindgen_anon_1.__bindgen_anon_2.syn() };
        let finish_flag = unsafe { (*tcp_header).__bindgen_anon_1.__bindgen_anon_2.fin() };
        let window = unsafe { (*tcp_header).__bindgen_anon_1.__bindgen_anon_2.window };
        let checksum = unsafe { (*tcp_header).__bindgen_anon_1.__bindgen_anon_2.check };
        let urgent_ptr = unsafe { (*tcp_header).__bindgen_anon_1.__bindgen_anon_2.urg_ptr };

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
        }
    }
}
