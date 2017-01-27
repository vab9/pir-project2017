mod input;
mod structs;
use input::read;
use std::env;
fn main() {
    let path = env::current_dir().unwrap();
    let filename = path.join("data/iris_flowers.txt");
}
