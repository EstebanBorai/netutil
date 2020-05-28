use crate::cmd::Sniff;
use crate::cli::validator::validate_target;
use clap::{App, Arg};

pub const TARGET: &str = "target";
pub const PORT_RANGE_END: &str = "rto";
pub const VERBOSE: &str = "verbose";

pub struct Cli {
  pub target_host: String,
}

impl Cli {
  pub fn start() {
    let target_arg = Arg::with_name(TARGET)
      .index(1)
      .help("Adress to connect to")
      .default_value("127.0.0.1")
      .takes_value(true)
      .validator(validate_target)
      .value_name("TARGET IP")
      .required(true);

    let port_range_end = Arg::with_name(PORT_RANGE_END)
      .help("The last port to attempt a connection to")
      .default_value("0")
      .takes_value(true)
      .short("r")
      .long(PORT_RANGE_END)
      .value_name("LAST PORT")
      .required(false);

    let app = App::new("netutil")
      .version("0.0.1")
      .author("Esteban Borai <estebanborai@gmail.com> (https://github.com/EstebanBorai)")
      .arg(target_arg)
      .arg(port_range_end);

    Sniff::from(app.get_matches()).run();
  }
}
