// Copyright (c) 2017 rustle-my-net developers
// Licensed under the MIT license
//! #Rustle-My-Net
//!
//! An implementation of a simple Artificial Neural Network (ANN) in Rust.
//!
//! ##About
//! Rustle-My-Net is the basic implementation of a very simple ANN in Rust. The code was designed
//! using [this book](http://neuralnetworksanddeeplearning.com/) written by Michael Nielsen. While
//! this is a layman implementation it might help you understand how a neural network works and
//! how to implement one in rust.
//!
//! The Network uses Sigmoid Neurons and Stochastic Gradient Descent for learning. This version of
//! the network uses the quadratic cost function which is not ideal in terms of learning but easier
//! to implement.
//!
//! The results of the learning progress are stored in the log folder’s log files along with all
//! the other outputs that occur during the invocation of the learning progress.

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
