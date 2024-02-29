mod server;
mod config;

use server::Server;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = config::load_config()?;
    println!("{:?}", config);

    let server = Server::new("enp3s0".to_string());
    server.start();

    Ok(())
}
