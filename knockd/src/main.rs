use clap::Parser;
use log::LevelFilter;
use pretty_env_logger::env_logger::Builder;
use sequence::PortSequenceDetector;
use server::Server;

mod config;
mod executor;
mod sequence;
mod server;

#[derive(Parser, Debug)]
#[command(version = env!("VERSION"), about, long_about = "A port knocking server written in Rust")]
struct Args {
    /// Path to the configuration file
    #[arg(short, long, default_value = "config.yaml")]
    config: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Initialize the logger
    Builder::new()
        .filter_level(LevelFilter::Info) // Set default log level to Info
        .init();

    // Load the configuration
    let config = config::load_config(&args.config)?;
    // Create the sequence detector
    let detector = PortSequenceDetector::new(config);

    let mut server = Server::new("enp3s0".to_string(), Box::new(detector));
    server.start();

    Ok(())
}
