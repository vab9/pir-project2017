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

/// Will load a neural network located at `save_file` and input `data` into the network.
/// The number of correctly classified items in `data` will be printed on the info log.
///
/// If no network is located at `save_file` or there is an error on initialising it from file
/// an error will be logged.
pub fn classify(save_file: &str, data: &Vec<Data>) {
    let nn = match nn::Network::from_file(save_file) {
        Err(msg) => {
            error!("Error when trying to open network file at given location: {}", msg);
            return;
        },
        Ok(nn) => nn,
    };
    nn::learning::evaluate_with_output(&nn, &data);
}
