use std::net::{SocketAddr};

#[derive(Debug)]
pub struct Response {
  pub from_addrs: SocketAddr,
  pub status_code: u16,
}

impl Response {
  pub fn new(status_code: u16, from_addrs: SocketAddr) -> Self {
    Response {
      from_addrs,
      status_code
    }
  }
}