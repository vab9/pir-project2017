use rand::distributions::normal::Normal;
use rand;
use rand::distributions::IndependentSample;
use nalgebra::DMatrix;


/// Network type which shows how the Network looks like
pub struct Network {
    layers: Vec<u8>,
    weights: Vec<DMatrix<f32>>,
    biases: Vec<Vec<f32>>,
}

/// Implements Constructor and some getter functions
impl Network {
    /// build a new Network by given parameters
    pub fn new(sizes: Vec<u8>) -> Result<Network, &'static str> {
        use rand::distributions::{Normal, IndependentSample};

        if sizes.len() < 3 {
            return Err("Not enough layers");
        }

        // Store the weights and biases in lists
        // We will not need weights or biases for input layer, so ignore that (hence -1)
        let mut weights: Vec<DMatrix<f32>> = Vec::with_capacity(sizes.len() - 1);
        let mut biases: Vec<Vec<f32>> = Vec::with_capacity(sizes.len() - 1);

        let mut rng = rand::thread_rng();
        let normal = Normal::new(0.0, 1.0);

        for i in 1..sizes.len() {
            biases.push(Vec::with_capacity(sizes[i] as usize));
            for _ in 0..sizes[i] {
                biases[i - 1].push(normal.ind_sample(&mut rng) as f32);
            }
            weights.push(DMatrix::new_random_normal(sizes[i - 1] as usize, sizes[i] as usize));
        }


        Ok(Network {
            layers: sizes,
            weights: weights,
            biases: biases,
        })
    }

    pub fn get_layers(&self) -> &Vec<u8> {
        &self.layers
    }

    pub fn get_weights(&self) -> &Vec<DMatrix<f32>> {
        &self.weights
    }

    pub fn get_biases(&self) -> &Vec<Vec<f32>> {
        &self.biases
    }
}

/// trait for implementing random values from 0 to 1
trait Random {
    fn new_random_normal(rows: usize, columns: usize) -> DMatrix<f32>;
}

/// Implements random number initializer for DMatrix
impl Random for DMatrix<f32> {
    fn new_random_normal(rows: usize, columns: usize) -> DMatrix<f32> {

        // Init the DMatrix with rows and columns
        let mut res;
        unsafe {
            res = DMatrix::new_uninitialized(rows, columns);
        }

        // random number between 0 and 1
        let mut rng = rand::thread_rng();
        let normal = Normal::new(0.0, 1.0);

        // fills matrix with random numbers from 0 and 1
        for i in 0..rows {
            for j in 0..columns {
                res[(i, j)] = normal.ind_sample(&mut rng) as f32;
            }
        }

        res
    }
}
