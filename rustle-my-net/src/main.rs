mod input;
mod structs;

use input::{read, commands};
use std::env;
fn main() {

    let (data, config, subcom) = commands();

    let path = env::current_dir().unwrap();
    let filename = path.join(data);
    let input = match read(&filename) {
        Ok(s) => s,
        Err(e) => panic!("paniced at read input:   {:?}",e)
    };
    for i in input {
        println!("{}",i);
    }
    println!("{:?}", config);
}
