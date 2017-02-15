use structs::Data;
use nn::Network;
use na::{DVector, DMatrix, Iterable, Transpose};

/// Execute Stochastic Gradient Descent on the `Network`.
///
/// `training_data` is the data actually used for learning and should be disjoint from the
/// `test_data`. Epochs is the  Number of learning cycles in each of which the whole `training_data`
/// will be cycled through in mini batches of `mini_batch_size` size. `Eta` is the learning rate.
/// `test_data` can be empty and if it is there will be no validation of the network.
///
/// The weights and biases of the network will be changed according to the gradient on the Error
/// over the mini_batch. Note that this means that the SGD does not actually calculate the gradient
/// over the whole training data set in each cycle, instead it calculates the gradient over the mini
/// batches and then sums those up (hence Stochastic Gradient Descent).
pub fn sgd(mut nn: &mut Network,
           mut training_data: Vec<Data>,
           epochs: u32,
           mini_batch_size: u8,
           eta: f32,
           test_data: Vec<Data>) {
    use rand::{self, Rng};

    // Used to shuffle data
    let mut rng = rand::thread_rng();

    // In each learning epoche: Shuffle the training data so that the mini batches always contain
    // different data sets from different flowers. Then update the mini batches using SGD.
    for j in 0..epochs {
        rng.shuffle(&mut training_data);
        for mut mini_batch in training_data.chunks_mut(mini_batch_size as usize) {
            // all the actual learning happens there:
            update_mini_batch(&mut nn, &mut mini_batch, eta);
        }
        if test_data.len() > 0 {
            debug!("Epoch {}: {}/{}",
                   j + 1,
                   evaluate(&nn, &test_data),
                   test_data.len());
        } else {
            debug!("Epoch {} complete!", j + 1);
        }
    }
}


// Applies Stochastic Gradient Descent over the mini batch.
fn update_mini_batch(mut nn: &mut Network, mini_batch: &mut [Data], eta: f32) {
    // nabla_b holds changes for biases in the network. Initialise with zeros because
    // the changes will later on be summed up in this vector
    let mut nabla_b: Vec<DVector<f32>> = Vec::with_capacity(nn.get_biases().len());
    for biases in nn.get_biases() {
        nabla_b.push(DVector::new_zeros(biases.len()));
    }
    // holds changes for weights in the network, similar to biases
    let mut nabla_w: Vec<DMatrix<f32>> = Vec::with_capacity(nn.get_weights().len());
    for weights in nn.get_weights() {
        // verify: rows, columns
        nabla_w.push(DMatrix::new_zeros(weights.nrows(), weights.ncols()));
    }

    // necessary because we can't access mini_batch_len later on
    let mini_batch_len = mini_batch.len();

    // for each dataset in mini_batch: calculate gradients, add to nablas
    for data in mini_batch {
        let (delta_nabla_b, delta_nabla_w) =
            backprop(&mut nn, data.get_input(), data.get_class_vector());
        for (mut nb, dnb) in nabla_b.iter_mut().zip(delta_nabla_b.iter()) {
            // TODO: Remove clone (seriously)
            *nb += dnb.clone();
        }
        for (mut nw, dnw) in nabla_w.iter_mut().zip(delta_nabla_w.iter()) {
            // TODO: Remove clone
            *nw += dnw.clone();
        }
    }

    // Update the actual weights and biases
    for (mut w, nw) in nn.get_weights_mut().iter_mut().zip(nabla_w.iter()) {
        *w -= (eta / (mini_batch_len as f32)) * nw.clone();
    }

    for (mut b, nb) in nn.get_biases_mut().iter_mut().zip(nabla_b.iter()) {
        *b -= eta / (mini_batch_len as f32) * nb.clone();
    }
}


// Gets the desired changes in weights and biases for one training example
fn backprop(nn: &mut Network,
            data: &DVector<f32>,
            desired_output: &DVector<f32>)
            -> (Vec<DVector<f32>>, Vec<DMatrix<f32>>) {
    use na::Outer;
    use nn;

    // Hold the changes calculated for this training data
    let mut nabla_b: Vec<DVector<f32>> = Vec::with_capacity(nn.get_biases().len());
    for biases in nn.get_biases() {
        nabla_b.push(DVector::new_zeros(biases.len()));
    }
    let mut nabla_w: Vec<DMatrix<f32>> = Vec::with_capacity(nn.get_weights().len());
    for weights in nn.get_weights() {
        nabla_w.push(DMatrix::new_zeros(weights.nrows(), weights.ncols()));
    }

    // feedforward

    // holds activation levels all for layers (including in- and output)
    let mut activations: Vec<DVector<f32>> = Vec::with_capacity(nn.get_layers().len());
    // note that this pushes the input activations
    activations.push(data.clone());

    // hold z for each layer where z is the input vector of the sigmoid function
    let mut zs: Vec<DVector<f32>> = Vec::with_capacity(nn.get_layers().len());

    // execute feedforward
    for (biases, weights) in nn.get_biases().iter().zip(nn.get_weights().iter()) {
        // TODO: Remove Clone
        zs.push(weights * &activations[activations.len() - 1] + biases.clone());
        activations.push(nn::sigmoid(&zs[zs.len() - 1]))
    }


    // backward pass

    // calculate values for output layer
    // delta is a measurement for the error of the last layer's output
    // compared to the desired output, we will derive the nabla values from this
    let mut delta = cost_derivative(&activations[activations.len() - 1], desired_output) *
                    sigmoid_prime(&zs[zs.len() - 1]);
    // need to store these because ownership issues
    let nabla_b_len = nabla_b.len();
    let nabla_w_len = nabla_w.len();
    // TODO: Remove clone
    nabla_b[nabla_b_len - 1] = delta.clone();
    nabla_w[nabla_w_len - 1] = (&delta).outer(&activations[activations.len() - 2]);

    // now calculate the values for all previous layers going from second to last to first layer
    // note: get_weiths is used for measurement of number of layers with biases and weights to
    // make sure the input layer is ignored
    for l in 2..nn.get_weights().len() + 1 {
        let z = &zs[zs.len() - l];
        let sp = sigmoid_prime(&z);
        delta = (&nn.get_weights()[nn.get_weights().len() - l + 1].transpose() * &delta) * sp;
        nabla_b[nabla_b_len - l] = delta.clone();
        nabla_w[nabla_w_len - l] = (&delta).outer(&activations[activations.len() - l - 1]);
    }
    (nabla_b, nabla_w)
}

// Derivative of the cost function
fn cost_derivative(output_activations: &DVector<f32>,
                   desired_output: &DVector<f32>)
                   -> DVector<f32> {
    // easy, derivative of quadratic cost function is:
    //TODO: Get rid of clone
    output_activations.clone() - desired_output.clone()
}


// Derivative of the sigmoid function
fn sigmoid_prime(z: &DVector<f32>) -> DVector<f32> {
    use nn;
    // Derivative of sigmoid function, ask wolfram alpha if you don't believe me
    nn::sigmoid(z) * (1.0f32 - nn::sigmoid(z))
}

// compares the output of the Network with the test_data
// returns the number of correct results
fn evaluate(nn: &Network, test_data: &Vec<Data>) -> u8 {
    // corr holds number of correctly recognised training data sets
    let mut corr = 0;
    // iterate over test data input vectors and test data class vectors
    for (x, y) in test_data.iter()
        .map(|x| x.get_input())
        .zip(test_data.iter()
            .map(|x| x.get_class_vector())) {
        //TODO: Shitty performance yo
        if find_max(&nn.feedforward(x)) == find_max(&y) {
            corr += 1;
        }
    }
    corr
}


// returns the index of the highest value in the vector
fn find_max(vec: &DVector<f32>) -> usize {
    vec.iter()
        .enumerate()
        .max_by(|tuple1, tuple2| tuple1.1.partial_cmp(tuple2.1).unwrap())
        .unwrap()
        .0
}
