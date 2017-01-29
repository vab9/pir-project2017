use structs::{Flower, FlowerName};
use std::io::{BufReader, BufRead, Result};
use std::fs::File;
use std::path::Path;
pub fn read(filename: &Path) -> Result<Vec<Flower>> {
    let f = try!(File::open(filename));
    let reader = BufReader::new(&f);
    let mut flowers = Vec::new();
    for line in reader.lines() {
        let l = line.unwrap();
        let content = l.split(',').collect::<Vec<&str>>();
        flowers.push(Flower::new(
            match content[4] {
                "Iris-setosa"  => FlowerName::IrisSetosa,
                "Iris-versicolor" => FlowerName::IrisVersicolor,
                "Iris-virginica" => FlowerName::IrisVirginica,
                _ => panic!("unknown flowertype!!!")
            },
            content[0].parse::<f32>().unwrap(),
            content[1].parse::<f32>().unwrap(),
            content[2].parse::<f32>().unwrap(),
            content[3].parse::<f32>().unwrap(),
        ));
    }
    Ok(flowers)
}
