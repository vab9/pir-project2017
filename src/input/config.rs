use input;
use input::clap::ArgMatches;
use log::LogLevelFilter;
use std::io;
use structs::Data;

use structs::flower::Flower;
use structs::mnist::Mnist;

/// Represents a configuration from command line arguments
#[derive(Debug)]
pub struct GlobalConfig {
    pub verbosity: LogLevelFilter,
    pub save_file: String,
    pub data: Result<Vec<Data>, io::Error>,
    pub learn_config: Option<LearningConfig>,
}

impl GlobalConfig {
    pub fn from_arguments<'a>(matches: ArgMatches) -> Self {
        let verbosity = match matches.value_of("verbosity") {
            Some("debug") => LogLevelFilter::Debug,
            Some("info") => LogLevelFilter::Info,
            Some("error") => LogLevelFilter::Error,
            Some("off") => LogLevelFilter::Off,
            // if something went wrong during parsing, we use the most verbose level
            _ => LogLevelFilter::Trace,
        };

        let s_file = matches.value_of("save_file").unwrap();

        // TODO: Should we double down on error handling by providing defaults with
        // with unwrap_or() instaed of unwrap() ?
        let learn_config = matches.subcommand_matches("learn").map(|sub_matches| {
            LearningConfig {
                learning_rate: sub_matches.value_of("learning_rate").unwrap().parse().unwrap(),
                epochs: sub_matches.value_of("epochs").unwrap().parse().unwrap(),
                batch_size: sub_matches.value_of("mini_batch_size").unwrap().parse().unwrap(),
                init_vec: {
                    sub_matches.values_of("topology")
                        .unwrap()
                        .map(|s| s.parse().expect("Unable to parse topology vector!"))
                        .collect()

                },
                test_size: sub_matches.value_of("test_data_size").unwrap().parse().unwrap(),
                save_file: s_file.to_string(),
            }
        });

        // determine which dataset to use
        // TODO: unwraps
        let data = match matches.value_of("datatype").unwrap() {
            "flower" => input::parse_data::<Flower>(matches.value_of("data").unwrap()),
            "mnist" => input::parse_data::<Mnist>(matches.value_of("data").unwrap()),
            _ => unreachable!(),
        };

        GlobalConfig {
            verbosity: verbosity,
            save_file: s_file.to_string(),
            data: data,
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
    pub save_file: String,
}
