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

const DEFAULT_SAVE_FILE: &'static str = "model_state.ser";

/// Reads the arguments given to this program at execution and returns them
pub fn read_arguments() -> config::GlobalConfig {
    let matches = App::new("rustle my net")
        .subcommand(SubCommand::with_name("learn")
            .arg(Arg::with_name("topology")
                .long("topology")
                .short("t")
                .help("A list of values representing the topology of the neural network. For \
                       example, the input '-t 4 5 3' would create a network with: 4 nodes in \
                       the input layer, a single hidden layer of 5 nodes and 3 nodes in the \
                       output layer.")
                .multiple(true)
                .value_delimiter(" ")
                .required(true)
                .min_values(3))
            .arg(Arg::with_name("learning_rate")
                .long("eta")
                .takes_value(true)
                .help("The learning rate eta. Should be between 0.0 and 1.0. Default is 0.05.")
                .default_value("0.05"))
            .arg(Arg::with_name("epochs")
                .long("epochs")
                .takes_value(true)
                .help("The number of training epochs. The default value is 100.")
                .default_value("100"))
            .arg(Arg::with_name("mini_batch_size")
                .long("batchsize")
                .takes_value(true)
                .help("The size of the mini batches for the learning process. Default: 32.")
                .default_value("32"))
            .arg(Arg::with_name("test_data_size")
                .long("testsize")
                .takes_value(true)
                .help("The size of the data that is used for validation. Defaults to 20.")
                .default_value("20")))
        .subcommand(SubCommand::with_name("classify"))
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .arg(Arg::with_name("verbosity")
            .long("verbosity")
            .short("v")
            .takes_value(true)
            .possible_values(&["debug", "info", "error", "off"])
            .default_value("info"))
        .arg(Arg::with_name("data")
            .long("data")
            .short("i")
            .takes_value(true)
            .default_value("data/iris_flowers.txt"))
        .arg(Arg::with_name("datatype")
            .long("type")
            .short("d")
            .takes_value(true)
            .possible_values(&["flower", "mnist"])
            .default_value("flower"))
        .arg(Arg::with_name("save_file")
            .long("file")
            .short("f")
            .takes_value(true)
            .default_value(DEFAULT_SAVE_FILE))
        .get_matches();

    config::GlobalConfig::from_arguments(matches)
}


/// Generically parse data from given input file into a Vec<Data>
fn parse_data<T>(datafile: &str) -> Result<Vec<Data>, io::Error>
    where T: FromStr + Into<Data>,
          T::Err: convert::From<io::Error> + Debug,
          Result<Vec<T>, T::Err>: FromIterator<Result<T, io::Error>>,
          Result<Vec<T>, io::Error>: FromIterator<Result<T, T::Err>>
{
    let mut path = self::util::get_root_dir();
    path.push(Path::new(datafile));

    let f = File::open(path)?;
    let reader = BufReader::new(&f);

    // read lines, map string to type T, convert T to Data, collect into a vec, return as Result
    Ok(reader.lines()
        .map(|l| l.unwrap().parse::<T>().unwrap().into())
        .collect())

}
