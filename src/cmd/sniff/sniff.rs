use crate::cli::{PORT_RANGE_END, TARGET, VERBOSE};
use crate::util::prompt;
use crate::cmd::sniff::{Response, Operation, ResponseStatus};
use clap::ArgMatches;
use colored::*;
use std::io::Write;
use std::net::{TcpStream, SocketAddr};
use std::str::FromStr;
use std::sync::mpsc::{channel, Sender};
use std::thread;

#[derive(Copy, Clone, Debug)]
pub struct Sniff {
  target: SocketAddr,
  end_port: Option<u16>,
  operation: Operation,
  verbose: bool
}

impl From<ArgMatches<'_>> for Sniff {
  fn from(args: ArgMatches<'_>) -> Self {
    let target: SocketAddr;
    let end_port: Option<u16>;
    let operation: Operation;
    let mut verbose = false;

    if args.is_present(VERBOSE) {
      verbose = true;
    }

    // when application gets here, the target value will be
    // already validated.
    // the unwrap the value and assign consume it
    let target_value = args.value_of(TARGET).unwrap();

    target = SocketAddr::from_str(target_value).unwrap();

    let end_port_value = args.value_of(PORT_RANGE_END).unwrap();
    let end_port_value: u16 = end_port_value.parse::<u16>().unwrap();

    if end_port_value > 0 {
      end_port = Some(end_port_value);
      operation = Operation::ScanRange;
    } else {
      end_port = None;
      operation = Operation::SingleAddr;
    }

    Self {
      target,
      end_port,
      operation,
      verbose
    }
  }
}

impl Sniff {
  pub fn run(&self) {
    self.run_from_op();
  }

  fn run_from_op(&self) {
    match self.operation {
      Operation::ScanRange => {
        let ans = prompt(&format!("Are you sure you want to scan on {} from port {} to {}? (y/n): ",
          self.target, self.target.port(), self.end_port.unwrap())).to_lowercase();

        if ans == "y" {
          self.scan();
        } else {
          println!("{}", "Aborted".yellow());
        }
      },
      Operation::SingleAddr => {
        self.single();
      }
    }
  }

  fn single(&self) {
    let addr = self.target;
    let (sender, receiver) = channel::<Response>();
    let is_logging = self.verbose;

    request(sender.clone(), addr);

    drop(sender);

    for res in receiver {
      if is_logging {
        res.log();
      }

      if res.response_status == ResponseStatus::Success {
        println!("{} {} - Open", "•".green(), addr.to_string());
      } else {
        println!("{} {} - Closed", "•".red(), addr.to_string());
      }
    }
  }

  /// Send Tcp connection requests to a range
  /// of ports on the provided address.
  fn scan(&self) {
    let addr = self.target;
    let (sender, receiver) = channel::<Response>();
    let is_logging = self.verbose;
    let mut open_ports = Vec::<SocketAddr>::new();

    println!("Scanning on {}", addr.to_string().cyan());

    for current_port in self.target.port()..self.end_port.unwrap() {
      let sender = sender.clone();
      let mut current_addr = addr.clone();

      current_addr.set_port(current_port);

      thread::spawn(move || {
        request(sender, current_addr);
      });
    }

    drop(sender);

    for res in receiver {
      if is_logging {
        res.log();
      }

      if res.response_status == ResponseStatus::Success {
        open_ports.push(res.from_addrs);
      }
    }

    if open_ports.len() == 0 {
      // no ports available
      println!(
        "No ports open in address {} for the port range {} to {}",
        self.target.to_string(),
        self.target.port(),
        self.end_port.unwrap()
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
      sender.send(Response::new(ResponseStatus::Success, addrs)).unwrap();
      res.flush().unwrap();
    }
    Err(_) => {
      sender.send(Response::new(ResponseStatus::Failure, addrs)).unwrap();
    }
  };
}
