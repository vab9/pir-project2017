pub mod flower;

use na::DVector;
use nn::Network;
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

/// Struct used as a container for serializing a network state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableNet {
    /// a Vec hat contains the networks layers
    pub layers: Vec<u8>,
    /// a Vec that contains the weights of the respective layer
    /// (nrows, ncols, column-major-vector)
    pub weights: Vec<(usize, usize, Vec<f32>)>,
    /// a Vec cointaining the biases of the respective layer
    pub biases: Vec<Vec<f32>>,
}

impl From<Network> for SerializableNet {
    fn from(network: Network) -> Self {

        let mut weights: Vec<(usize, usize, Vec<f32>)> = Vec::new();
        for matrix in network.get_weights() {
            // TODO: clone?!
            weights.push((matrix.nrows(), matrix.ncols(), matrix.clone().into_vector()))
        }

        let mut biases: Vec<Vec<f32>> = Vec::new();
        for vec in network.get_biases() {
            // TODO: clone?!
            biases.push(vec.clone().at);
        }
        SerializableNet {
            layers: network.get_layers().to_vec(),
            weights: weights,
            biases: biases,
        }
    }
}

/// Trait used to classify or declassify
pub trait Classifier {
    fn classify(&self) -> u8;
    // TODO: mega unsinnig dieses Trait, solange der return type ein Result<Flower...> ist
    // perhaps use generics?!
    fn declassify(num: u8) -> Result<FlowerName, io::Error>;
}
