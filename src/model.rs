use input::config;
use input::util;
use nn;
use structs::Data;

/// Train a neural network model
pub fn train(learn_cfg: &config::LearningConfig, mut data: Vec<Data>) {

    // split data into training and test data
    let (training_data, test_data) = util::split_data(&mut data, learn_cfg.test_size);

    info!("Initialising network...");

    // create the network
    let mut nn = nn::Network::new(&learn_cfg.init_vec).unwrap();
    // learn!
    nn::learning::sgd(&mut nn,
                      training_data,
                      learn_cfg.epochs,
                      learn_cfg.batch_size,
                      learn_cfg.learning_rate,
                      test_data);

    nn.save_to_file(&learn_cfg.save_file).unwrap_or_else(|e| {
        error!("Could not save network state to file: {}", e);
    });

    info!("...terminated!");
}

// TODO: implement this!
#[allow(dead_code, unused_variables, unused_mut)]
pub fn classify(save_file: &str, mut data: Vec<Data>) {
    let nn = nn::Network::from_file(save_file);
    unimplemented!();
}
