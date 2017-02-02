use rand::distributions::normal::Normal;
use rand;
use rand::distributions::IndependentSample;
use nalgebra::{DMatrix, DVector};


/// Network type which shows how the Network looks like
pub struct Network {
    /// a Vec giving the sizes of the layers
    /// the first entry corresponds to the inputlayer
    /// and the last entry corresponds to the outputlayer
    layers: Vec<u8>,
    /// a Vec that contains the weights of the respective layer
    weights: Vec<DMatrix<f32>>,
    ///
    biases: Vec<DVector<f32>>,
}


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
        let mut biases: Vec<DVector<f32>> = Vec::with_capacity(sizes.len() - 1);

        let mut rng = rand::thread_rng();
        let normal = Normal::new(0.0, 1.0);

        for i in 1..sizes.len() {

            unsafe {
                biases.push(DVector::new_uninitialized(sizes[i] as usize));
            }

            for j in 0..sizes[i] {
                biases[i - 1][j as usize] = normal.ind_sample(&mut rng) as f32;
            }
            weights.push(DMatrix::new_random_normal(sizes[i - 1] as usize, sizes[i] as usize));
        }


        Ok(Network {
            layers: sizes,
            weights: weights,
            biases: biases,
        })
    }

    /// returns the layers
    pub fn get_layers(&self) -> &Vec<u8> {
        &self.layers
    }

    /// returns the weights
    pub fn get_weights(&self) -> &Vec<DMatrix<f32>> {
        &self.weights
    }

    /// returns the biases
    pub fn get_biases(&self) -> &Vec<DVector<f32>> {
        &self.biases
    }
}

/// trait for implementing random values from 0 to 1
trait Random {
    fn new_random_normal(rows: usize, columns: usize) -> DMatrix<f32>;
}


impl Random for DMatrix<f32> {
    /// Implements random number initializer for DMatrix
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
