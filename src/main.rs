extern crate rand;
extern crate nalgebra as na;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate log;
mod input;
mod structs;
mod nn;
mod logging;

use input::{read, parse_commands};
use std::env;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use structs::{Classifier, SerializableNet};
use structs::flower::FlowerName;



fn main() {

    // parses commands
    let (data, config, subcom, verbosity) = parse_commands();
    // init logger
    if let Some(verbosity) = verbosity {
        logging::init_logger(verbosity);
    }

    info!("Starting_up");
    info!("Running with LogLevel: {:?}", verbosity);

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
          FlowerName::declassify(*m.get_class()).unwrap());

    // just dummy nn for no warnings
    let nn = nn::Network::new(vec![4, 7, 3]).unwrap();

    // dummy print for no warnings
    info!("{} {} {}",
          nn.get_layers().len(),
          nn.get_weights().len(),
          nn.get_biases().len());

    // ========================================================
    // CODE SHOWING THAT SERIALIZATION WORKS
    // ========================================================

    // wrap it in a SerializableNet
    let serializable_net: SerializableNet = nn.into();
    // serialize it
    let f = File::create(&path.join(&config)).unwrap();
    // new scope here b/c writer needs to be dropped before we reopen the file
    {
        let mut writer = BufWriter::new(f);
        serde_json::to_writer(&mut writer, &serializable_net).unwrap();
    }

    let f = File::open(&path.join(&config)).unwrap();
    let reader = BufReader::new(f);
    let my_net: SerializableNet = serde_json::from_reader(reader).unwrap();
    let new_net: nn::Network = my_net.into();
    info!("{:?}", new_net);

}
