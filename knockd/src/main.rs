use clap::Parser;
use sequence::PortSequenceDetector;
use server::Server;

mod config;
mod executor;
mod sequence;
mod server;

#[derive(Parser, Debug)]
#[command(version, about, long_about = "A port knocking server written in Rust")]
struct Args {
    /// Path to the configuration file
    #[arg(short, long, default_value = "config.yaml")]
    config: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Load the configuration
    let config = config::load_config(&args.config)?;
    // Create the sequence detector
    let detector = PortSequenceDetector::new(config);

    let mut server = Server::new("enp3s0".to_string(), Box::new(detector));
    server.start();

    Ok(())
}
