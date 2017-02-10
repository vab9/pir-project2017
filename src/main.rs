extern crate rand;
extern crate nalgebra;

#[macro_use]
extern crate log;
extern crate fern;
extern crate time;

mod input;
mod structs;
mod nn;


use input::{read, commands, check_logs};
use std::env;
use structs::{FlowerName, Classifier};


extern crate nalgebra as na;

fn main() {

    // init logger
    init_logger();

    info!("starting_up");

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

    info!("{:?} {:?}", config, subcom);


    let m = structs::Data::from_flower(input[0]);

    info!("{:?} {:?}",
          m.get_input(),
          FlowerName::declassify(*m.get_classifier()).unwrap());

    // just dummy nn for no warnings
    let nn = nn::Network::new(vec![4, 20, 3]).unwrap();

    // dummy print for no warnings
    info!("{} {} {}",
          nn.get_layers().len(),
          nn.get_weights().len(),
          nn.get_biases().len());


    info!("FF: {:?}",
          nn.feedforward(nalgebra::DVector::from_element(nn.get_layers()[0] as usize, 0 as f32)));
    info!("ended");
}


/// does init a logger that writes into a logfile
fn init_logger() {

    // OutputFilePath
    let str = format!("{}{}{}",
                      "logs/",
                      time::now().strftime("%Y-%m-%d_%H:%M:%S").unwrap(),
                      ".log");
    // init logger
    // write into file: [timestamp] [loglevel] message
    let logger_config = fern::DispatchConfig {
        format: Box::new(|msg: &str, level: &log::LogLevel, _location: &log::LogLocation| {
            format!("[{}][{}] {}",
                    time::now().strftime("%Y-%m-%d][%H:%M:%S").unwrap(),
                    level,
                    msg)
        }),
        // sets output file
        output: vec![fern::OutputConfig::file(str.as_str())],
        level: log::LogLevelFilter::Trace,
    };

    // tries to init logger
    if let Err(e) = fern::init_global_logger(logger_config, log::LogLevelFilter::Trace) {
        panic!("Failed to initialize global logger: {}", e);
    }

    // removes logs if therer are more than 20
    check_logs(20);
}
