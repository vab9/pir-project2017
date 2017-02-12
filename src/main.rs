extern crate rand;
extern crate nalgebra as na;

#[macro_use]
extern crate log;

mod input;
mod structs;
mod nn;
mod logging;


use input::{read, commands};
use std::env;
use structs::{FlowerName, Classifier};


fn main() {

    // does travis really need output?
    println!("hey travis");

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


    let m = structs::Data::from_flower(input[0]);

    info!("{:?} {:?}",
          m.get_input(),
          FlowerName::declassify(*m.get_classifier()).unwrap());

    // just dummy nn for no warnings
    let nn = nn::Network::new(vec![4, 20, 3]).unwrap();

    // dummy print for no warnings
    info!("{} {} {}",
          nn.get_layers().len(),
          nn.get_weights().len(),
          nn.get_biases().len());


    info!("FF: {:?}",
          nn.feedforward(na::DVector::from_element(nn.get_layers()[0] as usize, 0 as f32)));
    info!("ended");
}
