mod input;
mod models;
use input::read;
use models::Flower;
use std::path::Path;
fn main() {
    let filename = Path::new("/home/tom/pir-project2017/rustle-my-net/data/iris_flowers.txt");
    let flowers = match read(&filename) {
        Ok(val)  => val,
        Err(e) => panic!("reading file {} failed. Error {:?}",filename.display(), e),
    };
    println!("flower: {}",flowers[150]);
}
