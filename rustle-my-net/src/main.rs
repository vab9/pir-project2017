mod input;
mod structs;

use input::{read, commands};
use std::env;


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

    // prints the inputfile
    for i in input {
        println!("{}", i);
    }

    println!("{:?} {:?}", config, subcom);
}
