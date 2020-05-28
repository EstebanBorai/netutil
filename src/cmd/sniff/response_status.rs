use std::string::ToString;

#[derive(Debug, PartialEq)]
pub enum ResponseStatus {
  Success,
  Failure
}


impl ToString for ResponseStatus {
  fn to_string(&self) -> String {
    match self {
      ResponseStatus::Success => String::from("Success"),
      ResponseStatus::Failure => String::from("Failure")
    }
  }
}
