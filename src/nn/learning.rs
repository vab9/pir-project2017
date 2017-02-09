use structs::Data;
use nn::Network;

pub fn sgd(net: Network, mut training_data: Vec<Data>, epochs: u8, mini_batch_size: u8, eta: f32) {
    use rand;
    use rand::Rng;
    let mut rng = rand::thread_rng();

    for j in 0..epochs {
        rng.shuffle(&mut training_data);
        for mini_batch in training_data.chunks_mut(mini_batch_size as usize) {
            update_mini_batch(mini_batch, eta);
        }
        println!("Epoch {} complete!", j);
    }

}

fn update_mini_batch(mini_batch: &[Data], eta: f32) {
    unimplemented!();
}



/*
def SGD(self, training_data, epochs, mini_batch_size, eta,
            test_data=None):
        """Train the neural network using mini-batch stochastic
        gradient descent.  The "training_data" is a list of tuples
        "(x, y)" representing the training inputs and the desired
        outputs.  The other non-optional parameters are
        self-explanatory.  If "test_data" is provided then the
        network will be evaluated against the test data after each
        epoch, and partial progress printed out.  This is useful for
        tracking progress, but slows things down substantially."""
        if test_data: n_test = len(test_data)
        n = len(training_data)
        for j in xrange(epochs):
            random.shuffle(training_data)
            mini_batches = [
                training_data[k:k+mini_batch_size]
                for k in xrange(0, n, mini_batch_size)]
            for mini_batch in mini_batches:
                self.update_mini_batch(mini_batch, eta)
            if test_data:
                print "Epoch {0}: {1} / {2}".format(
                    j, self.evaluate(test_data), n_test)
            else:
                print "Epoch {0} complete".format(j)*/
