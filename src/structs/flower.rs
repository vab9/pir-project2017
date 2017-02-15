use structs::Classifier;
use std::fmt;
use na::DVector;
use std::io;
use std::str::FromStr;

/// flowertype that contains the 4 inputs and the Flowername
#[derive(Debug, Clone, Copy)]
pub struct Flower {
    pub name: FlowerName,
    pub sepal_length: f32,
    pub sepal_width: f32,
    pub petal_length: f32,
    pub petal_width: f32,
}


impl FromStr for Flower {
    type Err = io::Error;

    /// Parse a Flower from a string.
    ///
    /// Returns a `Result<Flower, io::Error>`, in case the incoming string cannot
    /// be parsed into a valid flower.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v: Vec<&str> = s.split(',').collect();

        let sepal_length = v[0].parse()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
        let sepal_width = v[1].parse()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
        let petal_length = v[2].parse()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
        let petal_width = v[3].parse()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;

        let name = match v[4] {
            "Iris-setosa" => FlowerName::IrisSetosa,
            "Iris-versicolor" => FlowerName::IrisVersicolor,
            "Iris-virginica" => FlowerName::IrisVirginica,
            _ => {
                return Err(io::Error::new(io::ErrorKind::InvalidInput,
                                          "Flower name did not match."))
            }
        };

        Ok(Flower {
            name: name,
            sepal_length: sepal_length,
            sepal_width: sepal_width,
            petal_length: petal_length,
            petal_width: petal_width,
        })

    }
}

impl fmt::Display for Flower {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "({:?}, {}, {}, {}, {})",
               self.name,
               self.sepal_length,
               self.sepal_width,
               self.petal_length,
               self.petal_width)
    }
}

impl From<Flower> for DVector<f32> {
    fn from(fl: Flower) -> DVector<f32> {

        DVector::from_slice(4,
                            &[fl.sepal_length, fl.sepal_width, fl.petal_length, fl.petal_width])
    }
}

/// enum flower names
#[derive(Debug, Clone, Copy)]
pub enum FlowerName {
    IrisSetosa,
    IrisVersicolor,
    IrisVirginica,
}

impl Classifier for FlowerName {
    /// classify FlowerName into an u8
    fn classify(&self) -> u8 {
        match *self {
            FlowerName::IrisSetosa => 0,
            FlowerName::IrisVersicolor => 1,
            FlowerName::IrisVirginica => 2,
        }
    }

    /// declassify an u8 into an FlowerName
    fn declassify(num: u8) -> Result<FlowerName, io::Error> {
        match num {
            0 => Ok(FlowerName::IrisSetosa),
            1 => Ok(FlowerName::IrisVersicolor),
            2 => Ok(FlowerName::IrisVirginica),
            _ => Err(io::Error::new(io::ErrorKind::InvalidInput, "Not a valid flower class")),
        }
    }
}
