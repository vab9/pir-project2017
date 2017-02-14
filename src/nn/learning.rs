use structs::Data;
use nn::Network;
use na::{DVector, DMatrix, Iterable};
use std::cmp::Ordering;


pub fn sgd(mut nn: &mut Network,
           mut training_data: Vec<Data>,
           epochs: u8,
           mini_batch_size: u8,
           eta: f32,
           test_data: Vec<Data>) {
    use rand::{self, Rng};
    let mut rng = rand::thread_rng();

    for j in 0..epochs {
        rng.shuffle(&mut training_data);
        for mut mini_batch in training_data.chunks_mut(mini_batch_size as usize) {
            update_mini_batch(&mut nn, &mut mini_batch, eta);
        }
        if test_data.len() > 0 {
            println!("Epoch {}: {}/{}",
                     j,
                     evaluate(&nn, &test_data),
                     test_data.len());
        } else {
            println!("Epoch {} complete!", j);
        }
    }
}

fn update_mini_batch(mut nn: &mut Network, mini_batch: &mut [Data], eta: f32) {
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
        let (delta_nabla_b, delta_nabla_w) =
            backprop(&mut nn, data.get_input(), data.get_class_vector());
        for (mut nb, dnb) in nabla_b.iter_mut().zip(delta_nabla_b.iter()) {
            // TODO: This is really not good. Someone needs to fix this.
            *nb += dnb.clone();
        }
        for (mut nw, dnw) in nabla_w.iter_mut().zip(delta_nabla_w.iter()) {
            // TODO: Same as above.
            *nw += dnw.clone();
        }
    }

    for (mut w, nw) in nn.get_weights_mut().iter_mut().zip(nabla_w.iter()) {
        *w -= eta / (mini_batch_len as f32) * nw.clone();
    }


    for (mut b, nb) in nn.get_biases_mut().iter_mut().zip(nabla_b.iter()) {
        *b -= eta / (mini_batch_len as f32) * nb.clone();
    }
}


/// Gets the desired changes in weights and biases for one training example
fn backprop(nn: &mut Network,
            data: &DVector<f32>,
            desired_output: &DVector<f32>)
            -> (Vec<DVector<f32>>, Vec<DMatrix<f32>>) {
    use na::{Outer};
    use nn;

    // holds all biases of the network
    let mut nabla_b: Vec<DVector<f32>> = Vec::with_capacity(nn.get_biases().len());
    for biases in nn.get_biases() {
        nabla_b.push(DVector::new_zeros(biases.len()));
    }
    // holds all weights of the network
    let mut nabla_w: Vec<DMatrix<f32>> = Vec::with_capacity(nn.get_weights().len());
    for weights in nn.get_weights() {
        nabla_w.push(DMatrix::new_zeros(weights.nrows(), weights.ncols()));
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
        let z = weights * &activations[activations.len() - 1] + biases.clone();
        zs.push(z);
        activation = nn::sigmoid(&zs[zs.len() - 1]);
        activations.push(activation)
    }

    // backward pass
    let mut delta = cost_derivative(&activations[activations.len()-1], desired_output) *
                    sigmoid_prime(&zs[zs.len() - 1]);
    let nabla_b_len = nabla_b.len() - 1;
    let nabla_w_len = nabla_w.len() - 1;
    // TODO: Remove clone
    nabla_b[nabla_b_len] = delta.clone();
    nabla_w[nabla_w_len] = (&nabla_b[nabla_b_len]).outer(&activations[activations.len() - 2]);

    //TODO: Verify if we need to iterate only to ...len()-1 because of input layer
    for l in 2..nn.get_weights().len() {
        let z = &zs[zs.len() - l];
        let sp = sigmoid_prime(&z);
        //TODO: Verify that this line does what it's supposed to
        delta = &nn.get_weights()[nn.get_weights().len() - l + 1] * (&delta) * sp;
        nabla_b[nabla_b_len - l] = delta.clone();
        nabla_w[nabla_w_len - l] = (&delta).outer(&activations[activations.len() - l - 1]);
    }
    (nabla_b, nabla_w)
}

/// Derivative of the cost function
fn cost_derivative(output_activations: &DVector<f32>,
                   desired_output: &DVector<f32>)
                   -> DVector<f32> {
    // easy, derivative of quadratic cost function is:
    //TODO: Get rid of clone
    output_activations.clone() - desired_output.clone()
}


/// Derivative of the sigmoid function
fn sigmoid_prime(z: &DVector<f32>) -> DVector<f32> {
    use nn;
    // Derivative of sigmoid function, ask wolfram alpha if you don't believe me
    nn::sigmoid(z) * (1.0f32 - nn::sigmoid(z))
}

fn evaluate(nn: &Network, test_data: &Vec<Data>) -> u8 {
    let mut corr = 0;
    for (x, y) in test_data.iter().map(|x| x.get_input()).zip(test_data.iter().map(|x| x.get_class_vector())) {
        //TODO: Shitty performance yo
        if find_max(&nn.feedforward(x)) == find_max(&y) {
            corr+=1;
        }
    }
    corr
}



fn find_max(vec: &DVector<f32>) -> usize {
    //let mut res: Vec<usize> = Vec::with_capacity(vec.len());

    //for dvec in vec {
    vec.iter()
        .map(|x| NonNan::new((*x).clone()).unwrap())
        .enumerate()
        .max_by_key(|&(_, ref item)| item.clone())
        .unwrap()
        .0
    //res.push(max.unwrap().0);
    //}


}


#[derive(Clone, PartialEq,PartialOrd)]
struct NonNan(f32);

impl NonNan {
    fn new(val: f32) -> Option<NonNan> {
        if val.is_nan() {
            None
        } else {
            Some(NonNan(val))
        }
    }
}

impl Eq for NonNan {}

impl Ord for NonNan {
    fn cmp(&self, other: &NonNan) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}
