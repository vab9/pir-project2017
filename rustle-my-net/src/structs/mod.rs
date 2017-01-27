use std::fmt;
#[derive(Debug)]
pub struct Flower {
    name: String,
    sepal_length: f32,
    sepal_width: f32,
    petal_length: f32,
    petal_width: f32,
}

impl Flower {
    pub fn new(n: String, sl: f32, sw: f32, pl: f32, pw: f32) -> Flower {
        Flower { name: n,
                 sepal_length: sl,
                 sepal_width: sw,
                 petal_length: pl,
                 petal_width: pw,
        }
    }
}
impl fmt::Display for Flower {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {}, {}, {})", self.name, self.sepal_length,
               self.sepal_width, self.petal_length, self.petal_width)
    }
}
