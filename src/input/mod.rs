pub mod config;
pub mod util;

extern crate clap;

use env;
use std::convert;
use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::iter::FromIterator;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use self::clap::{App, AppSettings, Arg, SubCommand};


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
