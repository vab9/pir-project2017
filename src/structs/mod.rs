use std::fmt;
use na::DVector;
use std::io::{Error, ErrorKind};


/// Struct for u8
pub struct Data {
    /// classifier for the NN (Result)
    classifier: u8,
    /// Input Vector for the Inputlayer of the NN
    input: DVector<f32>,
    //TODO: Completly replace classifier by class vector
    class_vector: DVector<f32>,
}


impl Data {
    /// Generates a new Data struct with a Vector and an u8(classifier)
    pub fn new(vec: DVector<f32>, clas: u8) -> Data {
        let mut class_v = DVector::from_element(3, 0.0f32);
        class_v[clas as usize] = 1.0f32;
        Data {
            classifier: clas,
            input: vec,
            class_vector: class_v,
        }
    }

    /// Generates a new Data struct from a Flower type
    pub fn from_flower(flower: Flower) -> Data {
        Data::new(DVector::from(flower), flower.get_flower_name().classify())
    }

    /// getter for the Input
    pub fn get_input(&self) -> &DVector<f32> {
        &self.input
    }

    /// getter for the classifier
    pub fn get_classifier(&self) -> &u8 {
        &self.classifier
    }

    pub fn get_class_vector(&self) -> &DVector<f32> {
        &self.class_vector
    }
}


/// trait that is used to classify or declassify
pub trait Classifier {
    fn classify(&self) -> u8;
    fn declassify(num: u8) -> Result<FlowerName, Error>;
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
    fn declassify(num: u8) -> Result<FlowerName, Error> {
        match num {
            0 => Ok(FlowerName::IrisSetosa),
            1 => Ok(FlowerName::IrisVersicolor),
            2 => Ok(FlowerName::IrisVirginica),
            _ => Err(Error::new(ErrorKind::Other, "Not a f32")),
        }
    }
}

/// flowertype that contains the 4 inputs and the Flowername
#[derive(Debug, Clone, Copy)]
pub struct Flower {
    name: FlowerName,
    sepal_length: f32,
    sepal_width: f32,
    petal_length: f32,
    petal_width: f32,
}

/// constuctor for the Flowertype
impl Flower {
    pub fn new(n: FlowerName, sl: f32, sw: f32, pl: f32, pw: f32) -> Flower {
        Flower {
            name: n,
            sepal_length: sl,
            sepal_width: sw,
            petal_length: pl,
            petal_width: pw,
        }
    }

    pub fn get_flower_name(&self) -> FlowerName {
        self.name
    }
}

/// easy printing of the flower type
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
