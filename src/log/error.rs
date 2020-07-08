#[derive(Debug)]
pub struct LogError {
  message: String
}

impl LogError {
  pub fn new(message: String) -> Self {
    LogError {
      message 
    }
  }
}