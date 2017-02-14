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


const T_SIZE: usize = 30;


fn main() {

    // parses commands
    let (data, config, subcom, verbosity) = parse_commands();

    use rand::{self, Rng};

    // init logger
    if let Some(verbosity) = verbosity {
        logging::init_logger(verbosity);
    }

    info!("Starting_up");
    info!("Running with LogLevel: {:?}", verbosity);

    // tries to open the file
    let mut input = data.unwrap();


    info!("{:?} {:?}", config, subcom);


    let mut rng = rand::thread_rng();
    rng.shuffle(&mut input);


    let mut training_data: Vec<Data> = Vec::with_capacity(input.len() - 30);
    for i in 0..input.len() - T_SIZE {
        training_data.push(structs::Data::from_flower(input[i]));
    }

    let mut test_data: Vec<Data> = Vec::with_capacity(30);
    for i in input.len() - T_SIZE..input.len() {
        test_data.push(structs::Data::from_flower(input[i]));
    }

    // just dummy nn for no warnings

    info!("hi Network");
    let mut nn = nn::Network::new(vec![4, 60, 3]).unwrap();


    nn::learning::sgd(&mut nn, training_data, 30000, 70, 0.15, test_data);
    // nn.feedforward(na::DVector::from_element(nn.get_layers()[0] as usize, 0.0)));

    info!("ended");

}
