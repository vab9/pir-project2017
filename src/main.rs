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

use input::{read, parse_commands};
use nn::Network;
use std::env;
use structs::Classifier;
use structs::flower::FlowerName;



fn main() {

    // parses commands
    let (data, config, subcom, verbosity) = parse_commands();
    // init logger
    if let Some(verbosity) = verbosity {
        logging::init_logger(verbosity);
    }

    info!("Starting_up");
    info!("Running with LogLevel: {:?}", verbosity);

    // gets path for data
    let path = env::current_dir().unwrap();
    let filename = path.join(data);

    // tries to open the file
    let input = match read(&filename) {
        Ok(s) => s,
        Err(e) => panic!("paniced at read input: {:?}", e),
    };

    info!("{:?} {:?}", config, subcom);


    let m = structs::Data::from_flower(input[0]);

    info!("{:?} {:?}",
          m.get_input(),
          FlowerName::declassify(*m.get_class()).unwrap());

    // just dummy nn for no warnings
    let nn = nn::Network::new(vec![4, 7, 3]).unwrap();

    // dummy print for no warnings
    info!("{} {} {}",
          nn.get_layers().len(),
          nn.get_weights().len(),
          nn.get_biases().len());

    // ========================================================
    // CODE SHOWING THAT SERIALIZATION WORKS
    // ========================================================
    nn.save_to_file("state1.json").unwrap();
    let loaded_nn = Network::from_file("state1.json");
    info!("{:?}", loaded_nn);

}
