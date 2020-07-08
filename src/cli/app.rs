use crate::whiff::Whiff;
use crate::cli::validator::validate_target;
use clap::{App, Arg};

pub const TARGET: &str = "target";
pub const RANGE: &str = "range";
pub const LOG_LEVEL: &str = "log_level";

pub struct Cli {
  pub target_host: String,
}

impl Cli {
  pub fn start() {
    let target_arg = Arg::with_name(TARGET)
      .index(1)
      .help("Adress to connect to")
      .default_value("0.0.0.0")
      .takes_value(true)
      .validator(validate_target)
      .value_name("TARGET IP")
      .required(true);

    let range_arg = Arg::with_name(RANGE)
      .help("Port range to test. If not specified, the range defaults to 500")
      .default_value("1")
      .takes_value(true)
      .short("r")
      .long(RANGE)
      .value_name("RANGE PORTS")
      .required(false);

    let log_level_arg = Arg::with_name(LOG_LEVEL)
    .help("Log level for the process. Defaults to error")
    .default_value("error")
    .takes_value(true)
    .long(LOG_LEVEL)
    .value_name("LOG LEVEL")
    .required(false);

    let app = App::new("whiff")
      .version("0.1.0")
      .author("Esteban Borai <estebanborai@gmail.com> (https://github.com/estebanborai)")
      .arg(target_arg)
      .arg(range_arg)
      .arg(log_level_arg);

    Whiff::from(app.get_matches()).run();
  }
}
