use sequence::PortSequenceDetector;
use server::Server;

mod config;
mod executor;
mod sequence;
mod server;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load the configuration
    let config = config::load_config("config.yaml")?;
    // Create the sequence detector
    let detector = PortSequenceDetector::new(config);

    let mut server = Server::new("enp3s0".to_string(), Box::new(detector));
    server.start();

    Ok(())
}
