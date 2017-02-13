use rand;
use rand::Rng;
use rand::distributions::normal::StandardNormal;
use na::{DMatrix, DVector, IterableMut};

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
    pub fn new(sizes: Vec<u8>) -> Result<Network, &'static str> {
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
            layers: sizes,
            weights: weights,
            biases: biases,
        })
    }

    /// Feed input through network, return output layer activation level
    pub fn feedforward(&self, mut a: DVector<f32>) -> DVector<f32> {
        for (weight, bias) in self.weights.iter().zip(self.biases.clone().into_iter()) {
            a = sigmoid(weight * a + bias);
        }
        a
    }

    /// return the layers used to initialize the ANN
    pub fn get_layers(&self) -> &[u8] {
        &self.layers
    }

    /// return a vector of the weight matrices of the ANN
    pub fn get_weights(&self) -> &[DMatrix<f32>] {
        &self.weights
    }

    /// return a vector of the bias matrices of the ANN
    pub fn get_biases(&self) -> &[DVector<f32>] {
        &self.biases
    }
}

// calculate elementwise sigmoid function
fn sigmoid(arr: DVector<f32>) -> DVector<f32> {
    let mut sig = arr.clone();
    for elem in sig.iter_mut() {
        *elem = 1.0 / (1.0 + (elem).exp());
    }
    sig
}


#[test]
fn test_sigmoid() {
    let mut arr_orig = DVector::from_element(3, 1.0f32);
    arr_orig[2] = 2.4137;
    let arr = sigmoid(arr_orig);
    assert_eq!(arr[0], arr[1]);
    assert_eq!(arr[0], 0.26894142137f32);
    assert_eq!(0.082133951f32, arr[2]);
}
