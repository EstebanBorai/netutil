use std::fmt;
use std::str::FromStr;
use std::error::Error;
use crate::log::{LogError};

#[derive(Copy, Clone, Debug)]
pub enum Log {
  Info,
  Warning,
  Error,
  NoLogs
}

impl FromStr for Log {
  type Err = LogError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "info" => Ok(Log::Info),
      "warning" => Ok(Log::Warning),
      "error" => Ok(Log::Error),
      _ => Err(LogError::new(format!("Invalid level provided! {}, valid levels are info, warning and error", s)))
    }
  }
}