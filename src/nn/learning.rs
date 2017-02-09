use structs::Data;
use nn::Network;
use nalgebra::{DVector, DMatrix};


pub fn sgd(mut nn: &mut Network,
           mut training_data: Vec<Data>,
           epochs: u8, mini_batch_size: u8,
           eta: f32) {
    use rand;
    use rand::Rng;
    let mut rng = rand::thread_rng();

    for j in 0..epochs {
        rng.shuffle(&mut training_data);
        for mut mini_batch in training_data.chunks_mut(mini_batch_size as usize) {
            update_mini_batch(&mut nn, &mut mini_batch, eta);
        }
        println!("Epoch {} complete!", j);
    }

}

fn update_mini_batch(mut nn: &mut Network, mut mini_batch: &mut [Data], eta: f32) {

    // holds all biases of the network
    let mut nabla_b: Vec<DVector<f32>> = Vec::with_capacity(nn.get_biases().len());
    for biases in nn.get_biases() {
        nabla_b.push(DVector::from_element(biases.len(), 0.0f32));
    }
    // holds all weights of the network
    let mut nabla_w: Vec<DMatrix<f32>> = Vec::with_capacity(nn.get_weights().len());
    for weights in nn.get_weights() {
        nabla_w.push(DMatrix::from_element(weights.nrows(), weights.ncols(), 0.0f32));
    }

    let mini_batch_len = mini_batch.len();

    // for each dataset in mini_batch: calculate gradients, add to nablas
    for data in mini_batch {
        let (delta_nabla_b, delta_nabla_w) = backprop(&mut nn,
                                                      data.get_input(),
                                                      data.get_classifier());
        for (mut nb, dnb) in nabla_b.iter_mut().zip(delta_nabla_b.iter()) {
            // TODO: This is really not good. Someone needs to fix this.
            *nb += dnb.clone();
        }
        for (mut nw, dnw) in nabla_w.iter_mut().zip(delta_nabla_w.iter()) {
            // TODO: Same as above.
            *nw += dnw.clone();
        }
    }

    for (mut w, nw) in nn.get_weights_mut().iter_mut().zip( nabla_w.iter() ) {
        *w -= eta / (mini_batch_len as f32) * nw.clone();
    }


    for (mut b, nb) in nn.get_biases_mut().iter_mut().zip( nabla_b.iter() ) {
        *b -= eta / (mini_batch_len as f32) * nb.clone();
    }


    unimplemented!();
}
/*
    def update_mini_batch(self, mini_batch, eta):
        """Update the network's weights and biases by applying
        gradient descent using backpropagation to a single mini batch.
        The "mini_batch" is a list of tuples "(x, y)", and "eta"
        is the learning rate."""
        nabla_b = [np.zeros(b.shape) for b in self.biases]
        nabla_w = [np.zeros(w.shape) for w in self.weights]
        for x, y in mini_batch:
            delta_nabla_b, delta_nabla_w = self.backprop(x, y)
            nabla_b = [nb+dnb for nb, dnb in zip(nabla_b, delta_nabla_b)]
            nabla_w = [nw+dnw for nw, dnw in zip(nabla_w, delta_nabla_w)]
        self.weights = [w-(eta/len(mini_batch))*nw
                        for w, nw in zip(self.weights, nabla_w)]
        self.biases = [b-(eta/len(mini_batch))*nb
                       for b, nb in zip(self.biases, nabla_b)]
*/

fn backprop(nn: &mut Network, data: &DVector<f32>, class: &u8) -> (Vec<DVector<f32>>,
                                                                   Vec<DMatrix<f32>>) {
    unimplemented!();
}
