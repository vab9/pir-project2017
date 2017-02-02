mod matrix;

use nn::matrix::Matrix;
//use rand::distributions::normal::Normal;
use rand;
//use rand::distributions::IndependentSample;


pub struct Network {
    layers: Vec<u8>,
    weights: Vec<Matrix>,
    biases: Vec<Vec<f32>>,
}

impl Network {
    pub fn new(sizes: Vec<u8>) -> Result<Network, &'static str> {
        use rand::distributions::{Normal, IndependentSample};

        if sizes.len() < 3 {
            return Err("Not enough layers");
        }

        // Store the weights and biases in lists
        // We will not need weights or biases for input layer, so ignore that (hence -1)
        let mut weights: Vec<Matrix> = Vec::with_capacity(sizes.len()-1);
        let mut biases: Vec<Vec<f32>> = Vec::with_capacity(sizes.len()-1);

        let mut rng = rand::thread_rng();
        let normal = Normal::new(0.0, 1.0);

        for i in 1..sizes.len() {
            biases[i] = Vec::with_capacity(sizes[i] as usize);
            for j in 0..sizes[i] {
                biases[i].push(normal.ind_sample(&mut rng) as f32);
            }
            weights[i] = Matrix::random_new(sizes[i-1] as u16, sizes[i] as u16);
        }

        Ok(Network {
            layers: sizes,
            weights: weights,
            biases: biases
        })
    }
}
