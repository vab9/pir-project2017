use std::env;
use std::path::PathBuf;
use structs::Data;

/// Get the root directory that the program runs in.
///
/// Will try to find the Cargo Root Directory, if that fails it will use the directory
/// from where the program was called.
pub fn get_root_dir() -> PathBuf {
    // TODO: remove expect here
    if let Ok(path) = env::var("CARGO_MANIFEST_DIR") {
        info!("Using $CARGO_MANIFEST_DIR as root directory.");
        PathBuf::from(path)
    } else {
        info!("Could not find $CARGO_MANIFEST_DIR, using current working directory instead.");
        env::current_dir().unwrap()
    }
}


/// Split the given data into a training and a test data set.
///
/// The output will be `(training_data, test_data)`. Before the data is split `input` will be
/// shuffled to ensure randomness in picking the test data. `test_data` will hold `test_data_size`,
/// `training_data` will contain all other elements from `input`.
pub fn split_data(mut input: &mut Vec<Data>, test_data_size: usize) -> (Vec<Data>, Vec<Data>) {
    use rand::{self, Rng};

    // shuffle data to make sure that not always the same data is picked as training and test data
    let mut rng = rand::thread_rng();
    rng.shuffle(&mut input);

    let mut training_data: Vec<Data> = Vec::with_capacity(input.len() - test_data_size);
    for i in 0..input.len() - test_data_size {
        training_data.push(input[i].clone());
    }

    let mut test_data: Vec<Data> = Vec::with_capacity(test_data_size);
    for i in input.len() - test_data_size..input.len() {
        test_data.push(input[i].clone());
    }
    (training_data, test_data)
}


/// Transform any given vector containing raw training data into a vector of `Data`.
///
/// `T` must support conversion into `Data` type. Use this directly after obtaining the
/// raw data since the network only works with the abstract `Data` and does not care what kind
/// of data set was originally input.
pub fn generic_to_data<T: Into<Data> + Clone>(input: Vec<T>) -> Vec<Data> {
    let mut input_data = Vec::with_capacity(input.len());
    for i in 0..input.len() {
        input_data.push(input[i].clone().into());
    }
    input_data
}
