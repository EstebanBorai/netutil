use crate::cli::{LOG_LEVEL, RANGE, TARGET};
use crate::log::Log;
use crate::whiff::Response;
use clap::ArgMatches;
use colored::*;
use std::io::Write;
use std::net::{SocketAddr, TcpStream};
use std::str::FromStr;
use std::sync::mpsc::{channel, Sender};
use std::thread;

#[derive(Copy, Clone, Debug)]
pub struct Whiff {
  target: SocketAddr,
  range: u16,
  log_level: Log,
}

impl From<ArgMatches<'_>> for Whiff {
  fn from(args: ArgMatches<'_>) -> Whiff {
    let target: SocketAddr;
    let range: u16;
    let log_level: Log;

    if let Some(value) = args.value_of(TARGET) {
      target = SocketAddr::from_str(value).unwrap();
    } else {
      panic!("Missing required argument: \"target\"");
    }

    if let Some(value) = args.value_of(RANGE) {
      range = value.parse::<u16>().unwrap();
    } else {
      println!(
        "{}",
        "Range not defined, using default value \"500\"".yellow()
      );
      range = 500;
    }

    match args.value_of(LOG_LEVEL) {
      Some(value) => log_level = Log::from_str(value).unwrap(),
      None => log_level = Log::NoLogs,
    }

    return Whiff {
      target,
      range,
      log_level,
    };
  }
}

impl Whiff {
  pub fn run(&self) {
    self.scan();
  }

  /// Send Tcp connection requests to a range
  /// of ports on the provided address.
  fn scan(&self) {
    println!("Scanning on {}", self.target.to_string().cyan());

    let mut open_ports = Vec::<SocketAddr>::new();
    let (sender, receiver) = channel::<Response>();

    for i in 0..self.range {
      let sender = sender.clone();
      let mut addr = self.target.clone();

      addr.set_port(self.target.port() + i);

      thread::spawn(move || {
        request(sender, addr);
      });
    }

    drop(sender);

    for res in receiver {
      if res.status_code == 200 {
        open_ports.push(res.from_addrs);
      }
    }

    if open_ports.len() == 0 {
      // no ports available
      println!(
        "No ports open in address {} for the port range {} to {}",
        self.target.to_string(),
        self.target.port(),
        self.target.port() + self.range
      );
    } else {
      println!("Open ports:");
      for addr in open_ports {
        println!("{} {}", "•".green(), addr.to_string());
      }
    }
  }
}

/// Sends a single request to the provided `SocketAddr`.
/// The provided `sender` (Channel Sender) will receive
/// the results of the request.
///
/// If it fails, a Response with status 404 (Not handled yet)
/// is sent.
/// Otherwise, a Response with status 200 is sent.
fn request(sender: Sender<Response>, addrs: SocketAddr) {
  match TcpStream::connect(&addrs) {
    Ok(mut res) => {
      sender.send(Response::new(200 as u16, addrs));
      res.flush().unwrap();
    }
    Err(_) => {
      sender.send(Response::new(404 as u16, addrs));
    }
  };
}

// pub fn whiff(thread_count: u16, ip: IpAddr) {
//   println!("Connecting to: {}", ip.to_string().green());
//   println!("Thread Count/Port Range: {}", thread_count.to_string().green());

//   let (sender, receiver) = channel();

//   for i in 0..thread_count {
//     let sender = sender.clone();

//     thread::spawn(move || {
//       scan(sender, i, ip, thread_count);
//     });
//   }

//   let mut out = vec![];
//   drop(sender);

//   for p in receiver {
//     out.push(p);
//   }

//   println!("");
//   out.sort();

//   println!("Resume:");
//   println!("- Available ports in {}", ip.to_string());

//   for v in out {
//     println!("{}", v);
//   }
// }

// fn scan(tx: Sender<u16>, start_from: u16, ip: IpAddr, thread_count: u16) {
//   let mut current_port: u16 = start_from + 1;

//   loop {
//     let addr = SocketAddr::new(ip, current_port);

//     match TcpStream::connect(&addr) {
//       Ok(_) => {
//         println!("Connected to: {}", &addr);
//         io::stdout().flush().unwrap();
//         tx.send(current_port).unwrap();
//       },
//       Err(err) => {
//         let err_description: &str = &err.to_string();

//         println!("Error connecting to: {}", &addr);
//         println!("{}", err_description.red());
//       }
//     }

//     if (u16::MAX - current_port) <= thread_count {
//       break;
//     }

//     current_port += thread_count;
//   }
// }
