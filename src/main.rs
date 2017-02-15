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
use nn::{learning, Network};
use std::convert;
use std::env;
use std::io;
use std::iter::FromIterator;
use std::str::FromStr;
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

    // Program logic starts here
    if let Some(learn_cfg) = config.learn_config {
        // learn
        // TODO: sort out Type parameter
        // Lukas, we should make this work together:
        //
        // learning::learn(&learn_cfg, config.data);
        // unimplemented!();

    } else {
        // classify
        // TODO: implement classify
        unimplemented!();
    }
}

fn learn(learn_cfg: &config::LearningConfig, data: Result<Vec<Data>, io::Error>) {

    // we got the input data from the parse_commands() invocation above
    let mut input_data = data.unwrap();

    // split data into training and test data
    let (training_data, test_data) = input::util::split_data(&mut input_data, TEST_DATA_SIZE);

    info!("Initialising network...");

    // create the network
    let mut nn = nn::Network::new(vec![4, 5, 3]).unwrap();
    // learn!
    nn::learning::sgd(&mut nn, training_data, 3000, 32, 0.05, test_data);
}
