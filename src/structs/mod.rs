pub mod flower;

use structs::flower::{Flower, FlowerName};
use na::DVector;
use std::io;

/// Struct for u8
#[derive(Clone)]
pub struct Data {
    /// class for the NN (Result)
    class: u8,
    /// Input Vector for the Inputlayer of the NN
    input: DVector<f32>,
    //TODO: Completly replace classifier by class vector
    class_vector: DVector<f32>,
}


impl Data {
    /// Generates a new Data struct with a Vector and an u8(class)
    pub fn new(vec: DVector<f32>, class: u8) -> Data {
        let mut class_v = DVector::from_element(3, 0.0f32);
        class_v[class as usize] = 1.0f32;
        Data {
            class: class,
            input: vec,
            class_vector: class_v,
        }
    }

    /// Generates a new Data struct from a Flower type
    pub fn from_flower(flower: Flower) -> Data {
        Data::new(DVector::from(flower), flower.name.classify())
    }

    /// getter for the Input
    pub fn get_input(&self) -> &DVector<f32> {
        &self.input
    }

    /// getter for the class
    pub fn get_class(&self) -> &u8 {
        &self.class
    }

    /// Get the class_vector
    pub fn get_class_vector(&self) -> &DVector<f32> {
        &self.class_vector
    }
}

/// trait that is used to classify or declassify
pub trait Classifier {
    fn classify(&self) -> u8;
    fn declassify(num: u8) -> Result<FlowerName, io::Error>;
}
