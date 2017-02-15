pub mod config;
pub mod util;

extern crate clap;

use std::convert;
use std::fs::File;
use std::fmt::Debug;
use std::io::{self, BufReader, BufRead};
use std::iter::FromIterator;
use std::path::Path;
use std::str::FromStr;
use structs::Data;

use self::clap::{App, AppSettings, Arg, SubCommand};


/// Reads the arguments given to this program at execution and returns them
pub fn read_arguments() -> config::GlobalConfig {
    let matches = App::new("rustle my net")
        .subcommand(SubCommand::with_name("learn")
            .arg(Arg::with_name("topology")
                .long("topology")
                .short("t")
                // TODO: proper help
                .help("a list of values representing the number of nodes in each layer")
                .multiple(true)
                .value_delimiter(" ")
                .required(true))
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
                .default_value("32"))
            .arg(Arg::with_name("test_data_size")
                .long("testsize")
                .takes_value(true)
                .default_value("20")))
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
            .short("i")
            .takes_value(true)
            .default_value("data/iris_flowers.txt"))
        .arg(Arg::with_name("network_model")
            .long("model")
            .short("m")
            .takes_value(true))
        .arg(Arg::with_name("datatype")
            .long("type")
            .short("d")
            .takes_value(true)
            .possible_values(&["flower", "mnist"])
            .default_value("flower"))
        .get_matches();

    config::GlobalConfig::from_arguments(matches)
}

/// reads content of given file and returns a result with
/// either the Vector of Flowers or Err
pub fn read_data<T>(filename: &Path) -> Result<Vec<Data>, io::Error>
    where T: FromStr + Into<Data> + Debug + ?Sized,
          T::Err: convert::From<io::Error> + Debug,
          Result<Vec<T>, T::Err>: FromIterator<Result<T, io::Error>>,
          Result<Vec<T>, io::Error>: FromIterator<Result<T, T::Err>>
{

    let f = File::open(filename)?;
    let reader = BufReader::new(&f);

    // original function -- leave this
    // reader.lines().map(|l| l?.parse::<T>()).collect()

    // TODO: rm unwrapping
    let v: Vec<Data> = reader.lines()
        .map(|l| l.unwrap().parse::<T>().unwrap().into())
        .collect();
    Ok(v)
}


// TODO: DOKU
fn parse_data(datafile: &str) -> Result<Vec<Data>, io::Error> {
    use structs::flower::Flower;
    let mut path = self::util::get_root_dir();
    path.push(Path::new(datafile));
    // TODO: could not make this happen generically
    // here you can switch between flower and mnist for now
    // until we find a solution
    read_data::<Flower>(&path)
}
