pub mod flower;
pub mod serialnet;

use na::DVector;
use std::io;
use structs::flower::{Flower, FlowerName};

/// Struct for u8
#[derive(Clone)]
pub struct Data {
    /// Input Vector for the input layer of the NN
    input: DVector<f32>,
    /// actual class vector of the for the NN (Result)
    class_vector: DVector<f32>,
}


impl Data {
    /// Generates a new Data struct with a Vector and an u8(class)
    pub fn new(vec: DVector<f32>, class: u8) -> Data {
        let mut class_v = DVector::from_element(3, 0.0f32);
        class_v[class as usize] = 1.0f32;
        Data {
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

    /// Get the class_vector
    pub fn get_class_vector(&self) -> &DVector<f32> {
        &self.class_vector
    }
}


/// Trait used to classify or declassify
pub trait Classifier {
    fn classify(&self) -> u8;
    // TODO: mega unsinnig dieses Trait, solange der return type ein Result<Flower...> ist
    // perhaps use generics?!
    fn declassify(num: u8) -> Result<FlowerName, io::Error>;
}
