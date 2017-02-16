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
mod model;

use input::config;
use input::util::generic_to_data;

fn main() {

    // read command line arguments
    let config: config::GlobalConfig = input::read_arguments();

    // initialize the global logger --> we can use info!(), debug!(), etc. from here on
    logging::init_logger(config.verbosity);

    info!("Starting_up...");
    info!("Running with Logging Level: {:?}", config.verbosity);

    // Program logic starts here
    let data = generic_to_data(config.data.unwrap());
    if let Some(learn_cfg) = config.learn_config {
        model::train(&learn_cfg, data);
    } else {
        model::classify(&config.save_file, &data);
    }
}
