extern crate rand;
extern crate nalgebra;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

mod input;
mod structs;
mod nn;

use input::{read, commands};
use std::env;
extern crate nalgebra as na;
use self::na::DVector;

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

    let m = DVector::from(input[0]);
    println!("{:?}",m);

    // just dummy nn for no warnings
    let nn = nn::Network::new(vec![4, 20, 3]).unwrap();

    // dummy print for no warnings
    println!("{} {} {}",
             nn.get_layers().len(),
             nn.get_weights().len(),
             nn.get_biases().len());


    println!("FF: {:?}",
             nn.feedforward(nalgebra::DVector::from_element(nn.get_layers()[0] as usize, 0 as f32))
    );

    nn.serialize(&path.join(&config));
    let tt = match nn::Network::deserialize(&path.join(&config)) {
        Ok(val) => val,
        Err(e) => { println!("{:?}",e);
                    return;
        }
    };
    tt.get_layers();

}
