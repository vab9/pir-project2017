extern crate clap;

use structs::Flower;
use std::io::{self, BufReader, BufRead};
use std::fs::File;
use std::path::Path;

use self::clap::{Arg, App, AppSettings, SubCommand};


/// reads content of given file and returns a result with
/// either the Vector of Flowers or Err
pub fn read(filename: &Path) -> io::Result<Vec<Flower>> {
    let f = File::open(filename)?;
    let reader = BufReader::new(&f);
    reader.lines().map(|l| l?.parse::<Flower>()).collect()
}

/// parses commands for the programm and returns a tuple of strings
pub fn commands() -> (String, String, String) {
    let matches = App::new("rustle my net")
        .subcommand(SubCommand::with_name("learn"))
        .subcommand(SubCommand::with_name("classify"))
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .arg(Arg::with_name("data")
            .long("data")
            .short("d")
            .takes_value(true)
            .default_value("data/iris_flowers.txt"))
        .arg(Arg::with_name("config")
            .long("config")
            .short("c")
            .takes_value(true)
            .default_value("data/iris_flowers.txt"))
        .get_matches();
    if matches.subcommand_name().is_none() {
        panic!("Provide at least one subcommand: learn or classify");
    }
    (matches.value_of("data").unwrap().parse().unwrap(),
     matches.value_of("config").unwrap().parse().unwrap(),
     matches.subcommand_name().unwrap().to_string())
}
