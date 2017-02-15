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
use structs::Data;

// number of data sets used for evaluation purposes only
const TEST_DATA_SIZE: usize = 20;



fn main() {

    // read command line arguments
    let config: config::GlobalConfig = input::read_arguments();

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
        learn(&learn_cfg,
              input::util::generic_to_data(config.data.unwrap()));

    } else {
        // classify
        // TODO: implement classify
        unimplemented!();
    }
}

fn learn(learn_cfg: &config::LearningConfig, mut data: Vec<Data>) {

    // split data into training and test data
    let (training_data, test_data) = input::util::split_data(&mut data, TEST_DATA_SIZE);

    info!("Initialising network...");

    // create the network
    let mut nn = nn::Network::new(&learn_cfg.init_vec).unwrap();
    // learn!
    nn::learning::sgd(&mut nn,
                      training_data,
                      learn_cfg.epochs,
                      learn_cfg.batch_size,
                      learn_cfg.learning_rate,
                      test_data);
}
