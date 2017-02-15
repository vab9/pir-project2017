pub mod config;
pub mod util;

extern crate clap;

use env;
use log::LogLevelFilter;
use std::convert;
use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::iter::FromIterator;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use structs::flower::Flower;
use structs::{self, Data};

use self::clap::{App, AppSettings, Arg, ArgMatches, SubCommand};


/// Reads the arguments given to this program at execution and returns them
pub fn read_arguments<T>() -> config::GlobalConfig<T>
    where T: FromStr,
          T::Err: convert::From<io::Error>,
          Result<Vec<T>, T::Err>: FromIterator<Result<T, io::Error>>,
          Result<Vec<T>, io::Error>: FromIterator<Result<T, T::Err>>
{
    let matches = App::new("rustle my net")
        .subcommand(SubCommand::with_name("learn")
            .arg(Arg::with_name("learning_rate")
                .long("eta")
                .takes_value(true)
                .help("The learning rate eta. Should be between 0.0 and 1.0")
                .default_value("0.05"))
            .arg(Arg::with_name("epochs")
                .long("epochs")
                .takes_value(true)
                .help("The number of training epochs")
                .default_value("100"))
            .arg(Arg::with_name("mini_batch_size")
                .long("batchsize")
                .takes_value(true)
                .help("The size of the mini batches for the learning process.")
                .default_value("32")))
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
        .arg(Arg::with_name("network_model")
            .long("model")
            .short("m")
            .takes_value(true))
        .get_matches();

    config::GlobalConfig::from_arguments(matches)
}

// TODO: ueberbrueckungsmethode
fn do_lame_stuff(matches: ArgMatches)
                 -> (Result<Vec<Flower>, io::Error>, String, String, Option<LogLevelFilter>) {
    let verbosity = match matches.value_of("verbosity") {
        Some("debug") => Some(LogLevelFilter::Debug),
        Some("info") => Some(LogLevelFilter::Info),
        Some("error") => Some(LogLevelFilter::Error),
        Some("off") => Some(LogLevelFilter::Off),
        _ => None,
    };
    let data = parse_data(matches.value_of("data").unwrap());

    (data,
     "data/config.json".to_string(),
     matches.subcommand_name().unwrap().to_string(),
     verbosity)
}

// TODO: put this function somewhere else. Perhaps in main.rs?!
fn do_awesome_stuff(matches: ArgMatches) {
    // let data = ...
    match matches.subcommand_name() {
        Some("learn") => {
            unimplemented!();
            // let learn_params = ...
            // learn(data, learn_params);
        }
        Some("classify") => {
            unimplemented!();
            // classify(data)
        }
        None => unreachable!(),
        _ => unreachable!(),
    }
}

/// reads content of given file and returns a result with
/// either the Vector of Flowers or Err
pub fn read_data<T>(filename: &Path) -> io::Result<Vec<T>>
    where T: FromStr,
          T::Err: convert::From<io::Error>,
          Result<Vec<T>, T::Err>: FromIterator<Result<T, io::Error>>,
          Result<Vec<T>, io::Error>: FromIterator<Result<T, T::Err>>
{
    let f = File::open(filename)?;
    let reader = BufReader::new(&f);
    reader.lines().map(|l| l?.parse::<T>()).collect()
}

// TODO: DOK
pub fn into_data_vec(input: Vec<Flower>) -> Vec<Data> {
    let mut input_data = Vec::with_capacity(input.len());
    for i in 0..input.len() {
        input_data.push(structs::Data::from_flower(input[i]));
    }
    input_data
}

// TODO: DOK
fn parse_data<T>(datafile: &str) -> Result<Vec<T>, io::Error>
    where T: FromStr,
          T::Err: convert::From<io::Error>,
          Result<Vec<T>, T::Err>: FromIterator<Result<T, io::Error>>,
          Result<Vec<T>, io::Error>: FromIterator<Result<T, T::Err>>
{
    let mut data_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    data_path.push(Path::new(datafile));
    read_data(&data_path)
}
