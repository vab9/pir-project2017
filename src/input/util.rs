use std::env;
use std::ffi::OsStr;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

fn get_root_dir() -> PathBuf {
    // TODO: try to get the right directory with cargo MANIFEST environment variable
    let mut p = env::current_dir().unwrap();
    p.push(Path::new("logs/"));
    p
}
