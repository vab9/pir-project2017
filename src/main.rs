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


use input::parse_commands;
use nn::Network;
use rand::Rng;
use std::env;
use structs::Data;

// number of data sets used for evaluation purposes only
const TEST_DATA_SIZE: usize = 20;


fn main() {

    // parses commands
    let (data, _, subcom, verbosity) = parse_commands();

    if let Some(verbosity) = verbosity {
        logging::init_logger(verbosity);
    }

    info!("Starting_up...");
    info!("Running with Logging Level: {:?}", verbosity);

    // we got the input data from the parse_commands() invocation above
    let mut input = data.unwrap();

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

    nn::learning::sanitise(&mut training_data, &mut test_data);
    nn::learning::check_san(&training_data, &test_data);

    info!("Initialising network...");

    // create the network
    let mut nn = nn::Network::new(vec![4, 5, 3]).unwrap();
    // learn!
    // params needed: epochs, mini_batch_size, learning_rate eta
    nn::learning::sgd(&mut nn, training_data, 30000, 32, 0.05, test_data);

    // ========================================================
    // CODE SHOWING HOW SERIALIZATION WORKS
    // ========================================================
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
