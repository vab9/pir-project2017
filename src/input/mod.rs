extern crate clap;

use log::LogLevelFilter;
use std::io::{self, BufReader, BufRead};
use std::fs::File;
use std::path::Path;
use structs::flower::Flower;

use self::clap::{App, AppSettings, Arg, SubCommand};


/// reads content of given file and returns a result with
/// either the Vector of Flowers or Err
pub fn read(filename: &Path) -> io::Result<Vec<Flower>> {
    let f = File::open(filename)?;
    let reader = BufReader::new(&f);
    reader.lines().map(|l| l?.parse::<Flower>()).collect()
}

/// parses commands for the programm and returns a tuple of strings
pub fn parse_commands() -> (String, String, String, Option<LogLevelFilter>) {
    let matches = App::new("rustle my net")
        .subcommand(SubCommand::with_name("learn"))
        .subcommand(SubCommand::with_name("classify"))
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .arg(Arg::with_name("verbosity")
            .long("verbosity")
            .short("v")
            .takes_value(true)
            .possible_values(&["debug", "info", "error", "off"])
            // TODO: change default to warn for production
            .default_value("debug"))
        .arg(Arg::with_name("data")
            .long("data")
            .short("d")
            .takes_value(true)
            .default_value("data/iris_flowers.txt"))
        .arg(Arg::with_name("config")
            .long("config")
            .short("c")
            .takes_value(true)
            .default_value("data/config.json"))
        .get_matches();

    let verbosity = match matches.value_of("verbosity") {
        Some("debug") => Some(LogLevelFilter::Debug),
        Some("info") => Some(LogLevelFilter::Info),
        Some("error") => Some(LogLevelFilter::Error),
        Some("off") => Some(LogLevelFilter::Off),
        _ => None,
    };

    // TODO: return a struct or a hashmap or something more elegant instead of a tuple
    (matches.value_of("data").unwrap().parse().unwrap(),
     matches.value_of("config").unwrap().parse().unwrap(),
     matches.subcommand_name().unwrap().to_string(),
     verbosity)
}
