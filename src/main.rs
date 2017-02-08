extern crate rand;
extern crate nalgebra;

mod input;
mod structs;
mod nn;

use input::{read, commands};
use std::env;
extern crate nalgebra as na;

fn main() {

    // parses commands
    let (data, config, subcom) = commands();

    // gets path for data
    let path = env::current_dir().unwrap();
    let filename = path.join(data);

    // tries to open the file
    let input = match read(&filename) {
        Ok(s) => s,
        Err(e) => panic!("paniced at read input:   {:?}", e),
    };

    println!("{:?} {:?}", config, subcom);

    let m = structs::Data::from_flower(input[0]);
    println!("{:?}", m.get_input());

    // just dummy nn for no warnings
    let nn = nn::Network::new(vec![4, 20, 3]).unwrap();

    // dummy print for no warnings
    println!("{} {} {}",
             nn.get_layers().len(),
             nn.get_weights().len(),
             nn.get_biases().len());


    println!("FF: {:?}",
             nn.feedforward(nalgebra::DVector::from_element(nn.get_layers()[0] as usize,
                                                            0 as f32)));
}
