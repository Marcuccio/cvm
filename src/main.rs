use std::path::PathBuf;
use clap::Parser;
use log::{info, warn};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// The files to use
    #[clap(required = true)]
    paths: Vec<PathBuf>,

    /// Outputs in CSV format
    #[clap(short, long)]
    delta: Option<f64>,

    /// Outputs in JSON format
    #[clap(short, long)]
    epsilon: Option<f64>,
}

fn main() {
    let cli = Cli::parse();

    let paths: Vec<PathBuf> = cli.paths.iter().map(PathBuf::from).collect();
    let delta = cli.delta.unwrap_or(0.001);
    let epsilon = cli.delta.unwrap_or(0.1);

    match cvm::estimate_from_many(&paths, delta, epsilon) {
        Ok(estimate) => {
            warn!("Use with caution, as this is an estimation.");
            info!("{}", estimate)
        },
        Err(e) => eprintln!("Error: {}", e),
    }
}