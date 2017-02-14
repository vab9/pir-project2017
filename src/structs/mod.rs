pub mod flower;
pub mod serialnet;

use na::DVector;
use std::io;
use structs::flower::{Flower, FlowerName};

/// Struct for u8
pub struct Data {
    /// class for the NN (Result)
    class: u8,
    /// Input Vector for the Inputlayer of the NN
    input: DVector<f32>,
}


impl Data {
    /// Generates a new Data struct with a Vector and a u8(class)
    pub fn new(vec: DVector<f32>, class: u8) -> Data {
        Data {
            class: class,
            input: vec,
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
}


/// Trait used to classify or declassify
pub trait Classifier {
    fn classify(&self) -> u8;
    // TODO: mega unsinnig dieses Trait, solange der return type ein Result<Flower...> ist
    // perhaps use generics?!
    fn declassify(num: u8) -> Result<FlowerName, io::Error>;
}
