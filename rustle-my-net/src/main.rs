mod input;
mod structs;
extern crate clap;
use clap::{Arg, App, SubCommand};
use input::read;
use std::env;
fn main() {
    let matches = App::new("rustle my net")
        .subcommand(SubCommand::with_name("learn"))
        .subcommand(SubCommand::with_name("classify"))
        .arg(Arg::with_name("data")
             .long("data")
             .short("d")
             .takes_value(true)
             .default_value("data/iris_flowers.txt"))
        .arg(Arg::with_name("config")
             .long("config")
             .short("c")
             .takes_value(true)
             .default_value("data/iris_flowers.txt"))
        .get_matches();

    let data: String = matches.value_of("data").unwrap().parse().unwrap();
    let config: String = matches.value_of("config").unwrap().parse().unwrap();
    if matches.subcommand_name().is_none() {
        println!("Provide at least one subcommand: learn or classify");
        return;
    }
    let path = env::current_dir().unwrap();
    let filename = path.join(data);
}
