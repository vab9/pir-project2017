extern crate serde_json;

pub mod learning;

use input::util;
use na::{DMatrix, DVector, IterableMut};
use rand;
use rand::distributions::normal::StandardNormal;
use rand::Rng;
use std::fs::File;
use std::io::{self, BufReader, BufWriter};
use std::str;
use structs::serialnet::SerializableNet;


/// Artificial Neural Network
///
/// This struct represents a simple Artificial Neural Network (ANN) using
/// feedforward and backpropagation. It uses Stochastic Gradient Descent(SGD)
/// and a sigmoid activation function.
///
/// [Source](http://neuralnetworksanddeeplearning.com/chap1.html)
///
/// # Example
///
/// ```rust
/// // This will create a new ANN with the following topology:
/// // 3 "neurons" in the input layer
/// // 5 "neurons" in the first hidden layer
/// // 3 "neurons" in the second hidden layer
/// // 2 "neurons" in the output layer
/// let nnet = Network::new(vec![3, 5, 2]);
/// ```
#[derive(Debug, Clone)]
pub struct Network {
    /// a Vec outlining the topology of the ANN
    /// the first entry corresponds to the inputlayer,
    /// the intermediate entries correspond to the hidden layers
    /// and the last entry corresponds to the outputlayer.
    layers: Vec<u8>,
    /// a Vec that contains the weights of the respective layer
    weights: Vec<DMatrix<f32>>,
    /// a Vec cointaining the biases of the respective layer
    biases: Vec<DVector<f32>>,
}


impl Network {
    /// build a new Network with a topology Vector
    pub fn new(sizes: &[u8]) -> Result<Network, &'static str> {
        assert!(sizes.len() >= 3, "at least three layers required");

        // Store the weights and biases in lists
        // We will not need weights or biases for input layer, so ignore that (hence -1)
        let mut weights = Vec::with_capacity(sizes.len() - 1);
        let mut biases = Vec::with_capacity(sizes.len() - 1);

        let mut rng = rand::thread_rng();

        // we use the standard normal distribution to initialize weights and biases - why?
        for (i, layer) in sizes.iter().enumerate().skip(1) {
            // initialize weight matrices
            weights.push(DMatrix::from_fn(*layer as usize, sizes[i - 1] as usize, |_, _| {
                let StandardNormal(x) = rng.gen();
                x as f32
            }));

            // initialize biases
            biases.push(DVector::from_fn(*layer as usize, |_| {
                let StandardNormal(x) = rng.gen();
                x as f32
            }));
        }

        Ok(Network {
            layers: sizes.to_vec(),
            weights: weights,
            biases: biases,
        })
    }

    /// Feed input through network, return output layer activation level
    pub fn feedforward(&self, a: &DVector<f32>) -> DVector<f32> {
        let mut act = a.clone();
        for (weight, bias) in self.weights.iter().zip(self.biases.clone().into_iter()) {
            act = sigmoid(&(weight * act + bias));
        }
        act
    }

    /// return the layers used to initialize the ANN
    pub fn get_layers(&self) -> &[u8] {
        &self.layers
    }

    /// return a vector of the weight matrices of the ANN
    pub fn get_weights(&self) -> &[DMatrix<f32>] {
        &self.weights
    }

    /// return a mutable vector of the weight matrices of the ANN
    pub fn get_weights_mut(&mut self) -> &mut Vec<DMatrix<f32>> {
        &mut self.weights
    }

    /// return a vector of the bias matrices of the ANN
    pub fn get_biases(&self) -> &[DVector<f32>] {
        &self.biases
    }

    // TODO: remove allow dead_code

    /// Saves a network state to the given filename and returns a result
    ///
    /// # Examples
    ///
    /// ```
    /// let mut nn = nn::Network::new(vec![4, 5, 3]).unwrap();
    /// let state_file_name = "state1.json";
    /// nn.save_to_file(state_file_name).unwrap();
    /// ```
    #[allow(dead_code)]
    pub fn save_to_file(self, filename: &str) -> Result<(), serde_json::Error> {
        // wrap it in a SerializableNet
        let serializable_net: SerializableNet = self.into();
        // create the file
        let f = File::create(util::get_root_dir().join("data/").join(filename)).unwrap();
        // create a writer
        let mut writer = BufWriter::new(f);
        // serialize the network and return the result
        serde_json::to_writer(&mut writer, &serializable_net)
    }


    /// Loads a network state from the given file
    ///
    /// Returns a result with the file or an io::Error if the specified file could
    /// not be opened
    ///
    /// # Examples
    ///
    /// ```
    /// let loaded_nn = Network::from_file("state1.json");
    /// ```
    pub fn from_file(filename: &str) -> Result<Self, io::Error> {
        // attempt to open the file
        let f = File::open(util::get_root_dir().join("data/").join(filename))?;
        let reader = BufReader::new(f);
        // read the SerializableNet from the file
        // we use unwrap here b/c if the file exists then we want the program to panic
        // if we cannot read from it
        let my_net: SerializableNet = serde_json::from_reader(reader)
            .expect("Could not parse Network from File");
        // convert into a Network and return it
        Ok(my_net.into())
    }

    /// return a vector of the bias matrices of the ANN
    pub fn get_biases_mut(&mut self) -> &mut Vec<DVector<f32>> {
        &mut self.biases
    }
}

/// calculate elementwise sigmoid function of the `input` vector.
pub fn sigmoid(input: &DVector<f32>) -> DVector<f32> {
    let mut sig = input.clone();
    for elem in sig.iter_mut() {
        *elem = 1.0 / (1.0 + (-1.0f32 * *elem).exp());
    }
    sig
}

impl From<SerializableNet> for Network {
    fn from(ser_net: SerializableNet) -> Self {

        let mut weights: Vec<DMatrix<f32>> = Vec::new();
        for v in ser_net.weights {
            let (nrows, ncols) = (v.0, v.1);
            weights.push(DMatrix::from_column_vector(nrows, ncols, &v.2))
        }

        let mut biases: Vec<DVector<f32>> = Vec::new();
        for v in ser_net.biases {
            biases.push(DVector::from_slice(v.len(), &v))
        }

        Network {
            layers: ser_net.layers,
            weights: weights,
            biases: biases,
        }
    }
}


#[test]
fn test_sigmoid() {
    let mut arr_orig = DVector::from_element(3, 1.0f32);
    arr_orig[2] = 2.4137;
    let arr = sigmoid(&arr_orig);
    assert_eq!(arr[0], arr[1]);
    assert_eq!(arr[0], 0.73105857863f32);
    assert_eq!(arr[2], 0.91786604895f32);
}
