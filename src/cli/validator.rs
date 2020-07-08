use std::net::SocketAddr;
use std::str::FromStr;

pub fn validate_target(val: String) -> Result<(), String> {
  match SocketAddr::from_str(&val) {
    Ok(_) => Ok(()),
    Err(err) => Err(err.to_string())
  }
}
