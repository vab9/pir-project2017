use std::env;
use std::path::PathBuf;

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
