extern crate rand;
#[macro_use]
extern crate log;
extern crate nalgebra as na;

mod input;
mod structs;
mod nn;
mod logging;

use input::{read, commands};
use std::env;
use structs::{Data};



fn main() {
    // init logger
    logging::init_logger();

    info!("starting_up");

    // parses commands
    let (data, config, subcom) = commands();

    // gets path for data
    let path = env::current_dir().unwrap();
    let filename = path.join(data);

    // tries to open the file
    let input = match read(&filename) {
        Ok(s) => s,
        Err(e) => panic!("paniced at read input: {:?}", e),
    };

    info!("{:?} {:?}", config, subcom);

    let mut training_data: Vec<Data> = Vec::with_capacity(input.len()-30);
    for i in 0..input.len()-30 {
        training_data.push(structs::Data::from_flower(input[i]));
    }

    let mut test_data: Vec<Data> = Vec::with_capacity(30);;
    for i in input.len()-30..input.len() {
        test_data.push(structs::Data::from_flower(input[i]));
    }

//    info!("{:?} {:?}",
//          m.get_input(),
//          FlowerName::declassify(*m.get_class()).unwrap());

    // just dummy nn for no warnings
    let mut nn = nn::Network::new(vec![4, 40, 3]).unwrap();


    nn::learning::sgd(&mut nn, training_data, 50, 10, 0.15, test_data);

/*
    // dummy print for no warnings
    info!("{} {} {}",
          nn.get_layers().len(),
          nn.get_weights().len(),
          nn.get_biases().len());


    info!("FF: {:?}",
          nn.feedforward(na::DVector::from_element(nn.get_layers()[0] as usize, 0 as f32)));*/
    info!("ended");

}
