use std::io::{stdin, stdout, Write};

pub fn prompt(message: &str) -> String {
  print!("{}", message);

  let mut s = String::new();
  let _ = stdout().flush();

  stdin().read_line(&mut s).expect("An error ocurred reading input");

  if let Some('\n') = s.chars().next_back() {
    s.pop();
  }

  if let Some('\r') = s.chars().next_back() {
    s.pop();
  }

  return s;
}