use std::thread;
use std::io::{self, Write};
use std::net::{IpAddr, TcpStream};
use std::sync::mpsc::{Sender, channel};

pub fn whiff(thread_count: u16, ip: IpAddr) {
  let (sender, receiver) = channel();

  for i in 0..thread_count {
    let sender = sender.clone();

    thread::spawn(move || {
      scan(sender, i, ip, thread_count);
    });
  }

  let mut out = vec![];
  drop(sender);

  for p in receiver {
    out.push(p);
  }

  println!("");
  out.sort();

  println!("Resume:");
  println!("- Available ports in {}", ip.to_string());

  for v in out {
    println!("{}", v);
  }
}

fn scan(tx: Sender<u16>, start_from: u16, ip: IpAddr, thread_count: u16) {
  let mut current_port: u16 = start_from + 1;

  loop {
    let addr = format!("{}:{}", ip.to_string(), current_port);
    match TcpStream::connect(&addr) {
      Ok(_) => {
        println!("Connected to: {}", &addr);
        io::stdout().flush().unwrap();
        tx.send(current_port).unwrap();
      },
      Err(_) => {}
    }

    if (u16::MAX - current_port) <= thread_count {
      break;
    }

    current_port += thread_count;
  }
}
