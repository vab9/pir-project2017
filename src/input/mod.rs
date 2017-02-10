extern crate clap;

use structs::{Flower, FlowerName};
use std::io::{BufReader, BufRead, Result};
use std::fs::{File, remove_file};
use std::path::Path;
use self::clap::{Arg, App, SubCommand};
use std::io::{Error, ErrorKind};
use std::fs;
use std::env;




/// reads content of given file and returns a result with
/// either the Vector of Flowers or Err
pub fn read(filename: &Path) -> Result<Vec<Flower>> {

    // Tries to open a file and reads it line for line
    let f = try!(File::open(filename));
    let reader = BufReader::new(&f);
    let mut flowers = Vec::new();

    // goes through every line and parses into a flower if valid
    for line in reader.lines() {
        let l = line.unwrap();
        let content = l.split(',').collect::<Vec<&str>>();

        // matches if its a valid flower
        match is_flower(&content[..]) {
            Ok(res) => flowers.push(res),
            Err(e) => println!("{}", e),
        }
    }
    Ok(flowers)
}

/// parses the String array into the flower type and returns it as
/// as a Result
fn is_flower(flower: &[&str]) -> Result<Flower> {

    // just returns for every value of the flower
    // or an std::io::Error
    let name = match flower[4] {
        "Iris-setosa" => FlowerName::IrisSetosa,
        "Iris-versicolor" => FlowerName::IrisVersicolor,
        "Iris-virginica" => FlowerName::IrisVirginica,
        _ => return Err(Error::new(ErrorKind::Other, "Not a flower")),
    };

    let sepal_length = match flower[0].parse::<f32>() {
        Ok(x) => x,
        _ => return Err(Error::new(ErrorKind::Other, "Not a f32")),
    };

    let sepal_width = match flower[1].parse::<f32>() {
        Ok(x) => x,
        _ => return Err(Error::new(ErrorKind::Other, "Not a f32")),
    };

    let petal_length = match flower[2].parse::<f32>() {
        Ok(x) => x,
        _ => return Err(Error::new(ErrorKind::Other, "Not a f32")),
    };

    let petal_width = match flower[3].parse::<f32>() {
        Ok(x) => x,
        _ => return Err(Error::new(ErrorKind::Other, "Not a f32")),
    };

    Ok(Flower::new(name, sepal_length, sepal_width, petal_length, petal_width))
}


/// parses commands for the programm and returns a tuple of strings
pub fn commands() -> (String, String, String) {
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
    if matches.subcommand_name().is_none() {
        panic!("Provide at least one subcommand: learn or classify");
    }
    (matches.value_of("data").unwrap().parse().unwrap(),
     matches.value_of("config").unwrap().parse().unwrap(),
     matches.subcommand_name().unwrap().to_string())
}

/// checks logfile count and removes the oldest accessed files if the count
/// is higher than max_files
/// creation time isnt supported on debian
pub fn check_logs(max_files: u8) {

    // logdir
    let path = env::current_dir().unwrap().join("logs");

    // reads directory
    let paths = fs::read_dir(&Path::new(&path)).unwrap();

    // maps log names into a Vec
    let mut names = paths.filter_map(|entry| {
            entry.ok().and_then(|e| {
                e.path()
                    .file_name()
                    .and_then(|n| n.to_str().map(|s| String::from(s)))
            })
        })
        .collect::<Vec<String>>();

    // removes oldest accessed files as long as there are to many logfiles
    while names.len() > max_files as usize {

        let mut index = 0;

        // get accessed time
        let mut oldest = match fs::metadata(path.join(&names[0])).unwrap().accessed() {
            Ok(x) => x,
            Err(e) => {
                println!("{}", e);
                println!("You have to delete logfiles yourself");
                break;
            }
        };

        // gets oldest accessed file
        for i in 0..names.len() {

            // does work because the code above validate that it is available
            let current = fs::metadata(path.join(&names[i])).unwrap().accessed().unwrap();

            // set new oldest
            if oldest > current {
                oldest = current;
                index = i;
            }

        }

        /// file exist for sure
        match remove_file(path.join(&names[index])) {
            Ok(_) => println!("log removed"),
            Err(_) => println!("something went wrong"),
        }

        /// removes from Vector
        names.remove(index as usize);
    }
}
