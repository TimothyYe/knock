use server::Server;

mod config;
mod sequence;
mod server;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server = Server::new("enp3s0".to_string());
    server.start();

    Ok(())
}
