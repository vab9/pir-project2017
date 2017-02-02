mod input;
mod structs;
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
}
