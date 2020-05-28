use crate::cmd::sniff::{ResponseStatus};
use std::net::{SocketAddr};

#[derive(Debug)]
pub struct Response {
  pub from_addrs: SocketAddr,
  pub response_status: ResponseStatus,
}

impl Response {
  pub fn new(response_status: ResponseStatus, from_addrs: SocketAddr) -> Self {
    Response {
      from_addrs,
      response_status
    }
  }

  pub fn log(&self) {
    println!("{}\t{}", self.from_addrs, self.response_status.to_string());
  }
}