extern crate rand;
#[macro_use]
extern crate log;
extern crate nalgebra as na;

mod input;
mod structs;
mod nn;
mod logging;

use input::parse_commands;
use std::env;
use structs::Data;

// number of data sets used for evaluation purposes only
const TEST_DATA_SIZE: usize = 30;


fn main() {

    // parses commands
    let (data, config, subcom, verbosity) = parse_commands();

    use rand::{self, Rng};

    // init logger
    if let Some(verbosity) = verbosity {
        logging::init_logger(verbosity);
    }

    info!("Starting_up...");
    info!("Running with Loging Level: {:?}", verbosity);

    // we got the input data from the parse_commands() invocation above
    let mut input = data.unwrap();

    info!("config: {:?}", config);
    info!("subcommand: {:?}", subcom);

    // init RNG
    let mut rng = rand::thread_rng();
    rng.shuffle(&mut input);

    // split into training and test data
    let mut training_data: Vec<Data> = Vec::with_capacity(input.len() - 30);
    for i in 0..input.len() - TEST_DATA_SIZE {
        training_data.push(structs::Data::from_flower(input[i]));
    }

    let mut test_data: Vec<Data> = Vec::with_capacity(30);
    for i in input.len() - TEST_DATA_SIZE..input.len() {
        test_data.push(structs::Data::from_flower(input[i]));
    }

    info!("Initialising network");

    // create the network
    let mut nn = nn::Network::new(vec![4, 30, 3]).unwrap();
    // learn!
    nn::learning::sgd(&mut nn, training_data, 30000, 70, 0.15, test_data);

    info!("...terminated!");

}
