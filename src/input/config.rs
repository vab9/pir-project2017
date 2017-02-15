use input;
use input::clap::ArgMatches;
use log::LogLevelFilter;
use std::convert;
use std::io;
use std::iter::FromIterator;
use std::str::FromStr;


/// Represents a configuration from command line arguments
#[derive(Debug)]
pub struct GlobalConfig<T> {
    pub verbosity: LogLevelFilter,
    pub data: Result<Vec<T>, io::Error>,
    pub datatype: String,
    pub learn_config: Option<LearningConfig>,
}

impl<T> GlobalConfig<T>
    where T: FromStr,
          T::Err: convert::From<io::Error>,
          Result<Vec<T>, T::Err>: FromIterator<Result<T, io::Error>>,
          Result<Vec<T>, io::Error>: FromIterator<Result<T, T::Err>>
{
    pub fn from_arguments<'a>(matches: ArgMatches) -> Self {
        let verbosity = match matches.value_of("verbosity") {
            Some("debug") => LogLevelFilter::Debug,
            Some("info") => LogLevelFilter::Info,
            Some("error") => LogLevelFilter::Error,
            Some("off") => LogLevelFilter::Off,
            // if something went wrong during parsing, we use the most verbose level
            _ => LogLevelFilter::Trace,
        };
        // TODO: Should we double down on error handling by providing defaults with
        // with unwrap_or() instaed of unwrap() ?
        let learn_config = matches.subcommand_matches("learn").map(|matches| {
            LearningConfig {
                learning_rate: matches.value_of("learning_rate").unwrap().parse().unwrap(),
                epochs: matches.value_of("epochs").unwrap().parse().unwrap(),
                batch_size: matches.value_of("mini_batch_size").unwrap().parse().unwrap(),
                init_vec: {
                    matches.value_of("topology")
                        .unwrap()
                        .split_whitespace()
                        .map(|s| s.parse().expect("Unable to parse topology vector!"))
                        .collect()
                },
                test_size: matches.value_of("test_data_size").unwrap().parse().unwrap(),
            }
        });

        let data = input::parse_data(matches.value_of("data").unwrap());

        GlobalConfig {
            verbosity: verbosity,
            data: data,
            datatype: matches.value_of("datatype").unwrap().to_string(),
            learn_config: learn_config,
        }
    }
}

#[derive(Debug, Clone)]
pub struct LearningConfig {
    pub learning_rate: f32,
    pub epochs: u32,
    pub batch_size: u8,
    pub init_vec: Vec<u8>,
    pub test_size: usize,
}
