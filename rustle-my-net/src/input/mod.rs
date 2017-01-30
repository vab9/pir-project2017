use structs::{Flower, FlowerName};
use std::io::{BufReader, BufRead, Result};
use std::fs::File;
use std::path::Path;
extern crate clap;
use self::clap::{Arg, App, SubCommand};

// reads content of given file and returns a result with
// either the Vector of Flowers or Err
pub fn read(filename: &Path) -> Result<Vec<Flower>> {
    let f = try!(File::open(filename));
    let reader = BufReader::new(&f);
    let mut flowers = Vec::new();
    for line in reader.lines() {
        let l = line.unwrap();
        let content = l.split(',').collect::<Vec<&str>>();
        // Expected Flower?
        match (match content[4] {
            "Iris-setosa"  => Ok(FlowerName::IrisSetosa),
            "Iris-versicolor" => Ok(FlowerName::IrisVersicolor),
            "Iris-virginica" => Ok(FlowerName::IrisVirginica),
            _ => Err("Invalid flower"),
        },
               content[0].parse::<f32>(),
               content[1].parse::<f32>(),
               content[2].parse::<f32>(),
               content[3].parse::<f32>()) {
            // push if valid Flower
            (Ok(name),
             Ok(sepal_length),
             Ok(sepal_width),
             Ok(petal_length),
             Ok(petal_width)) => flowers.push(Flower::new(
                 name,
                 sepal_length,
                 sepal_width,
                 petal_length,
                 petal_width,
             )),
            // print Err if invalid line
            _ =>  println!("this is not a valid flower {:?}", content),
        };




    }
    Ok(flowers)
}
pub fn commands() -> (String,String, String) {
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
      matches.subcommand_name().unwrap().to_string()
    )
}
