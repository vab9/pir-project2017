use structs::Data;
use nn::Network;
use nalgebra::{DVector, DMatrix};


pub fn sgd(mut nn: &mut Network,
           mut training_data: Vec<Data>,
           epochs: u8, mini_batch_size: u8,
           eta: f32) {
    use rand::{ self, Rng};
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
                                                      data.get_class_vector());
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
}


fn backprop(nn: &mut Network, data: &DVector<f32>, desired_output: &DVector<f32>)
            -> (Vec<DVector<f32>>, Vec<DMatrix<f32>>) {
    use nalgebra::{self, Dot};
    //use nalgebra::Dot;
    use nn;

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

    // feedforward
    // current activation layer, at the beginning this is the input
    // Note: Clone here because in later iterations activation will actually hold the
    // ownership of the Vector (or rather the activations vector will). However this might
    // still be a performance issue since we call this once per training data.
    let mut activation = data.clone();

    // hold all activation layers (including output)
    let mut activations: Vec<DVector<f32>> = Vec::with_capacity(nn.get_layers().len());
    // hold z where z is the input of the sigmoid function for each layer
    let mut zs: Vec<DVector<f32>> = Vec::with_capacity(nn.get_layers().len());

    activations.push(activation);
    // execute feedforward
    for (biases, weights) in nn.get_biases().iter().zip(nn.get_weights().iter()) {
        // TODO: Remove Clone
        let z = weights * &activations[activations.len()-1] + biases.clone();
        zs.push(z);
        activation = nn::sigmoid(&zs[zs.len()-1]);
        activations.push(activation)
    }

    // backward pass
    let delta = cost_derivative(&activations[activations.len()], desired_output)
        * sigmoid_prime(&zs[zs.len()-1]);
    let nabla_b_len = nabla_b.len() -1;
    let nabla_w_len = nabla_w.len() -1;
    nabla_b[nabla_b_len] = delta;
    //let mut act_transp = DMatrix::from_column_iter(
    nabla_w[nabla_w_len] = (&nabla_b[nabla_b_len]).clone().dot(&activations[activations.len()-2]).clone();

    unimplemented!();
}
/*

        # backward pass
        delta = self.cost_derivative(activations[-1], y) * \
            sigmoid_prime(zs[-1])
        nabla_b[-1] = delta
        nabla_w[-1] = np.dot(delta, activations[-2].transpose())
        # Note that the variable l in the loop below is used a little
        # differently to the notation in Chapter 2 of the book.  Here,
        # l = 1 means the last layer of neurons, l = 2 is the
        # second-last layer, and so on.  It's a renumbering of the
        # scheme in the book, used here to take advantage of the fact
        # that Python can use negative indices in lists.
        for l in xrange(2, self.num_layers):
            z = zs[-l]
            sp = sigmoid_prime(z)
            delta = np.dot(self.weights[-l+1].transpose(), delta) * sp
            nabla_b[-l] = delta
            nabla_w[-l] = np.dot(delta, activations[-l-1].transpose())
        return (nabla_b, nabla_w)



   def backprop(self, x, y):
        """Return a tuple "(nabla_b, nabla_w)" representing the
        gradient for the cost function C_x.  "nabla_b" and
        "nabla_w" are layer-by-layer lists of numpy arrays, similar
        to "self.biases" and "self.weights"."""
        nabla_b = [np.zeros(b.shape) for b in self.biases]
        nabla_w = [np.zeros(w.shape) for w in self.weights]
        # feedforward
        activation = x
        activations = [x] # list to store all the activations, layer by layer
        zs = [] # list to store all the z vectors, layer by layer
        for b, w in zip(self.biases, self.weights):
            z = np.dot(w, activation)+b
            zs.append(z)
            activation = sigmoid(z)
            activations.append(activation)
*/

fn transpose_vec(vec: &Vec<DVector<f32>>) -> DMatrix<&f32> {
    // this eventually has to go. If we need this for any vector of DVectors we should probably
    // think of storing it all in a DMatrix instead.
    use nalgebra::Iterable;
    let iter = vec.iter().flat_map(|x| x.iter());

    let mut m = DMatrix::from_column_iter(vec[0].len(), vec.len(), iter);
    m = m.transpose()
}



/// Derivative of the cost function
fn cost_derivative(output_activations: &DVector<f32>, desired_output: &DVector<f32>)
                   -> DVector<f32> {
    // easy, derivative of quadratic cost function is:
    //TODO: Get rid of clone
    output_activations.clone()-desired_output.clone()
}


/// Derivative of the sigmoid function
fn sigmoid_prime(z: &DVector<f32>) -> DVector<f32>{
    use nn;
    // Derivative of sigmoid function, ask wolfram alpha if you don't believe me
    nn::sigmoid(z) * (1.0f32 - nn::sigmoid(z))
}
