pub mod flower;
pub mod serialnet;
pub mod mnist;

use na::DVector;
use std::io;
use structs::flower::{Flower, FlowerName};
use structs::mnist::Mnist;

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
    pub fn new(vec: DVector<f32>, class: u8, output_neurons: usize) -> Data {
        let mut class_v = DVector::from_element(output_neurons, 0.0f32);
        class_v[class as usize] = 1.0f32;
        Data {
            input: vec,
            class_vector: class_v,
        }
    }

    /// Generates a new Data struct from a Flower type
    pub fn from_flower(flower: Flower) -> Data {
        Data::new(DVector::from(flower), flower.name.classify(), 3)
    }

    pub fn from_mnist(mnist: Mnist) -> Data {
        Data::new(DVector::from_slice(784, mnist.get_slice()),
                  mnist.get_class(),
                  10)
    }

    /// getter for the Input
    pub fn get_input(&self) -> &DVector<f32> {
        &self.input
    }

    /*pub fn get_input_mut(&mut self) -> &mut DVector<f32> {
        &mut self.input
    }*/

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
