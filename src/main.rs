extern crate rand;
extern crate nalgebra as na;
#[macro_use]
extern crate serde_derive;
extern crate serde;
#[macro_use]
extern crate log;
mod input;
mod structs;
mod nn;
mod logging;

use input::config;
use nn::Network;
use std::env;
use std::io;
use structs::Data;
use structs::flower::Flower;


// number of data sets used for evaluation purposes only
const TEST_DATA_SIZE: usize = 20;


fn main() {

    // read command line arguments
    let config: config::GlobalConfig<Flower> = input::read_arguments();

    // initialize the global logger --> we can use info!(), debug!(), etc. from here on
    logging::init_logger(config.verbosity);

    info!("Starting_up...");
    info!("Running with Logging Level: {:?}", config.verbosity);

    // ========================================================
    // LUKAS THINGS
    // ========================================================

    // we got the input data from the parse_commands() invocation above
    let mut input_data = input::into_data_vec(config.data.unwrap());

    // split data into training and test data
    let (training_data, test_data) = input::util::split_data(&mut input_data, TEST_DATA_SIZE);

    info!("Initialising network...");

    // create the network
    let mut nn = nn::Network::new(vec![4, 5, 3]).unwrap();
    // learn!
    nn::learning::sgd(&mut nn, training_data, 3000, 32, 0.05, test_data);

    // ========================================================
    // CODE SHOWING HOW SERIALIZATION WORKS
    // ========================================================
    let mut nn = nn::Network::new(vec![4, 5, 3]).unwrap();
    let state_file_name = "state1.json";
    nn.save_to_file(state_file_name).unwrap();
    info!("Saved a neural network state to file: {}", state_file_name);
    info!("=====================================");

    let loaded_nn = Network::from_file("state1.json");
    info!("Loaded Neural Network from file: {}", state_file_name);
    info!("{:?}", loaded_nn);
    info!("=====================================");
    info!("...terminated!");
}

fn learn<T>(learn_cfg: &config::LearningConfig, data: Result<Vec<T>, io::Error>) {
    unimplemented!();
}
