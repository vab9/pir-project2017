use structs::Flower;
use std::io::{BufReader, BufRead, Result};
use std::fs::File;
use std::path::Path;
pub fn read(filename: &Path) -> Result<Vec<Flower>> {
    let f = try!(File::open(filename));
    let mut reader = BufReader::new(&f);
    let mut flowers = Vec::new();
    let mut len = 4;
    while len > 3 {
        let mut line = "".to_string();
        len = match reader.read_line(&mut line) {
            Ok(val) => val,
            Err(_) => return Ok(flowers),
        };
        if len < 4 {
            return Ok(flowers);
        }
        let split = line.split(',');
        let content = split.collect::<Vec<&str>>();
        let mut s = content[4].to_string();
        s.pop();
        s.pop();
        flowers.push(Flower::new(s,
                                 content[0].parse::<f32>().unwrap(),
                                 content[1].parse::<f32>().unwrap(),
                                 content[2].parse::<f32>().unwrap(),
                                 content[3].parse::<f32>().unwrap()));
    }
    Ok(flowers)
}
