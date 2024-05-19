use std::{env, io};
use std::path::PathBuf;
use log::{info, warn};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <file_path1> <file_path2> ...", args[0]);
        std::process::exit(1);
    }

    let paths: Vec<PathBuf> = args[1..].iter().map(PathBuf::from).collect();
    let epsilon = 0.1;
    let delta = 0.01;

    match cvm::estimate_from_many(&paths, epsilon, delta) {
        Ok(estimate) => {
            warn!("Use with caution, as this is an estimation.");
            info!("{}", estimate)
        },
        Err(e) => eprintln!("Error: {}", e),
    }
}