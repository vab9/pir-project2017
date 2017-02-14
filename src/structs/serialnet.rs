use nn::Network;

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
