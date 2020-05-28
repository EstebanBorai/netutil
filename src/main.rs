use std::str::FromStr;
use std::net::{IpAddr};

mod whiff;

fn main() {
  // Hardcoded improve using a CLI or something
  whiff::whiff(5, IpAddr::from_str("127.0.0.1").unwrap());
}
