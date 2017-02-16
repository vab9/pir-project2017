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
    /// Holds level of verbosity for output (`Debug`, `Info`, `Error` or `Off`)
    pub verbosity: LogLevelFilter,
    /// Path to where saved network object is located,
    /// will currently default to "model_state.ser" if not specified
    pub save_file: String,
    /// The actual data that the network will use to learn / classify
    pub data: Result<Vec<Data>, io::Error>,
    /// Hyperparameters for network learning
    pub learn_config: Option<LearningConfig>,
}

impl GlobalConfig {
    /// Parse the given `ArgMatches` into a `GlobalConfig`
    pub fn from_arguments<'a>(matches: ArgMatches) -> Self {
        // get verbosity level or set highest in case of failure
        let verbosity = match matches.value_of("verbosity") {
            Some("debug") => LogLevelFilter::Debug,
            Some("info") => LogLevelFilter::Info,
            Some("error") => LogLevelFilter::Error,
            Some("off") => LogLevelFilter::Off,
            // if something went wrong during parsing, we use the most verbose level
            _ => LogLevelFilter::Trace,
        };

        let s_file = matches.value_of("save_file").unwrap();

        // create the learning configuration
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
        // if we add other datasets here, we also need to implement a Datatype for it
        // and add it to the possible values in clap (input::read_arguments())
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

/// The hyperparameters used for nn-learning.
///
/// These will be set manually and influence the behaviour, speed and success of the
/// learning progress. They depend on your training data set layout and the problem you are trying
/// to solve. Some settings might result in very good behaviour, some in the network getting more
/// stupid in each learning epoch. Have fun figuring them out :P
#[derive(Debug, Clone)]
pub struct LearningConfig {
    /// Influences speed and success of the learning progress. High values increase speed, too high
    /// values will result in the network not being able to learn. In most cases covered in this NN
    /// the value should be <1.
    pub learning_rate: f32,
    /// Number of learning epochs. Higher values yield better results by increasing
    /// total training time. Setting this too high might result in
    /// overfitting on your training data.
    pub epochs: u32,
    /// Size of minibatches that stochastic gradient descent will be applied on. Too big samples
    /// will result in slower networks, too small samples will result in bad gradients and thus
    /// badly influence learning success. In all examples covered by this net the size was ~10.
    pub batch_size: u32,
    /// Network topology: holds number of neurons in each layer (including in- and output layer).
    /// Example: [4, 5, 3] will result in a network with 4 input neurons, 5 neurons in a hidden
    /// layer, and three output neurons.
    pub init_vec: Vec<u32>,
    /// Number of samples to use for testing. Larger number of samples (especially when you don't
    /// have many samples in total) will decrease learning success.
    pub test_size: usize,
    /// Path to where the nn is to be loaded from
    pub save_file: String,
}
