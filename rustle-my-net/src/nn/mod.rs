mod matrix;

pub struct Network {
    layers: Vec<u8>,
    weights: Vec<Matrix>,
    biases: Vec<Vector>,
}

impl Network {
    pub fn new(sizes: Vec<u8>) {
        use rand::distributions::{Normal, IndependentSample};

        // Store the weights and biases in lists
        let weights: Vec<Matrix> = Vec::with_capacity(sizes.len()-1);
        let biases: Vec<Vector> = Vec::with_capacity(sizes.len()-1);


    }
}
