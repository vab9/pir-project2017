extern crate rand;
extern crate blas;

mod nn;

fn main() {
    let nn = nn::Network::new(vec![4, 20, 3]);
    println!("Hello, world!");
}
