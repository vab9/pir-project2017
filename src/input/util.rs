use std::env;
use std::path::PathBuf;
use structs::Data;

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

pub fn split_data(mut input: &mut Vec<Data>, test_data_size: usize) -> (Vec<Data>, Vec<Data>) {
    use rand;
    use rand::Rng;

    // split into training and test data
    // init RNG
    let mut rng = rand::thread_rng();
    rng.shuffle(&mut input);

    let mut training_data: Vec<Data> = Vec::with_capacity(input.len() - test_data_size);
    for i in 0..input.len() - test_data_size {
        training_data.push(input[i].clone());
    }

    let mut test_data: Vec<Data> = Vec::with_capacity(test_data_size);
    for i in input.len()-test_data_size..input.len() {
        test_data.push(input[i].clone());
    }
    (training_data, test_data)
}
