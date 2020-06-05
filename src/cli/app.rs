use colored::*;
use clap::{App, Arg};
use crate::whiff::{whiff as start_whiff};
use std::str::FromStr;
use std::net::{IpAddr};

pub struct Cli {
  pub target_host: String
}

impl Cli {
  pub fn start() {
    let target_host: &str;
    let mut range_ports: u16 = 500;

    let app = App::new("whiff")
      .version("0.1.0")
      .author("Esteban Borai <estebanborai@gmail.com> (https://github.com/estebanborai)")
      .arg(
        Arg::with_name("target_host")
          .help("Target host to connect")
          .takes_value(true)
          .short("t")
          .long("target")
          .value_name("TARGET IP")
          .required(true)
      )
      .arg(
        Arg::with_name("port_count")
          .help("Range of ports to test from 0")
          .default_value("500")
          .takes_value(true)
          .short("r")
          .long("range")
          .value_name("RANGE PORTS")
          .required(false)
      );

      let matches = app.get_matches();

      if let Some(value) = matches.value_of("target_host") {
        target_host = value;
      } else {
        panic!("Missing \"target\" argument!");
      }

      if let Some(value) = matches.value_of("range") {
        range_ports = value.parse::<u16>().unwrap();
      } else {
        println!("{}","Range not defined, using default value \"500\"".yellow());
      }

      start_whiff(range_ports, IpAddr::from_str(target_host).unwrap());
  }
}
