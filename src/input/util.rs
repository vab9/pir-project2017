use std::env;
use std::path::PathBuf;

pub fn get_root_dir() -> PathBuf {
    // TODO: remove expect here
    PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("Could not find the CARGO_MANIFEST_DIR"))
}
