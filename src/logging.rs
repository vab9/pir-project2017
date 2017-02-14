extern crate fern;
extern crate time;

use input::util;
use log;
use std::ffi::OsStr;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

const MAX_LOG_FILES: usize = 10;

/// Initializes a global logger that logs to its own file for each run
///
/// It can be used with the macros in the log crate. The resulting log files can
/// be found in the logs subdirectory.
pub fn init_logger(verbosity: log::LogLevelFilter) {

    // remove old logs if the amount of logs in log directory exceeds MAX_LOG_FILES
    if let Err(e) = clean_logs() {
        panic!("Error trying to clean logs: {} ... exiting!", e);
    }

    // get the log directory path
    let logs_dir_path = get_log_dir();

    // check if the log directory exists, create it if not
    if fs::metadata(&logs_dir_path).is_err() {
        fs::create_dir(Path::new("logs/")).unwrap();
    }

    // the logfiles are named with a timestamp
    let logfile_path =
        logs_dir_path.join(Path::new(format!("{}{}", time::now().rfc3339(), ".log").as_str()));


    // initialize the actual logger
    // output is in the following format: [timestamp] [loglevel] message
    let logger_config = fern::DispatchConfig {
        format: Box::new(|msg: &str, level: &log::LogLevel, _location: &log::LogLocation| {
            format!("[{}] [{}] {}",
                    time::now().strftime("%H:%M:%S").unwrap(),
                    level,
                    msg)
        }),
        // set the output file
        output: vec![fern::OutputConfig::stdout(), fern::OutputConfig::file(&logfile_path)],
        // set the max log level (see below)
        level: log::LogLevelFilter::Trace,
    };

    // try to initialize the logger, setting the actual log level given during command parsing
    if let Err(e) = fern::init_global_logger(logger_config, verbosity) {
        panic!("Failed to initialize global logger: {}", e);
    }
}

/// Deletes logfiles if there are more than MAX_LOG_FILES in the log/ directory
///
/// it returns the amount of files deleted, or an io::Error if the operation fails
fn clean_logs() -> Result<usize, io::Error> {

    // get the log dir path
    let path = get_log_dir();

    // if reading the directory fails, n_logs stays 0 and nothing is deleted
    let mut n_logs = 0;
    if let Ok(iter) = fs::read_dir(&path) {
        n_logs = iter.filter_map(|f| f.ok())
            .filter(|f| f.path().extension() == Some(OsStr::new("log")))
            .count();
    }

    // calculate how many logfiles too many there are
    // - 2 because the current execution also generates a logfile
    // and we want to have at most MAX_LOG_FILES files.
    let excess_files = n_logs.saturating_sub(MAX_LOG_FILES - 2);

    // collect paths to the old files in a vector
    let mut delete_vec: Vec<PathBuf> = Vec::new();
    if let Ok(iter) = fs::read_dir(&path) {
        delete_vec = iter.take(excess_files)
            .filter_map(|f| f.ok())
            .filter(|f| f.path().extension() == Some(OsStr::new("log")))
            .map(|f| f.path())
            .collect();
    }

    // delete the selected files
    for f in delete_vec {
        fs::remove_file(&path.join(f))?;
    }
    Ok(excess_files)
}

fn get_log_dir() -> PathBuf {
    let mut p = util::get_root_dir();
    p.push(Path::new("logs/"));
    p
}
