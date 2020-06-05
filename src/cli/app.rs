use clap::{App, Arg};
use crate::whiff::{whiff as start_whiff};
use std::str::FromStr;
use std::net::{IpAddr};

pub struct Cli {
  pub target_host: String
}

impl Cli {
  pub fn start() {
    let mut target_host: &str = "";

    let app = App::new("whiff")
      .version("0.1.0")
      .author("Esteban Borai <estebanborai@gmail.com> (https://github.com/estebanborai)")
      .arg(
        Arg::with_name("target_host")
          .help("Port Sniffer Utility")
          .takes_value(true)
          .short("t")
          .long("target")
          .value_name("TARGET IP")
          .required(true)
      );

      let matches = app.get_matches();

      if let Some(value) = matches.value_of("target_host") {
        target_host = value;
      } else {
        panic!("Missing \"target\" argument!");
      }

      start_whiff(5, IpAddr::from_str(target_host).unwrap());
  }
}
