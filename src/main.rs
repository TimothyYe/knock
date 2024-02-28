mod server;
use server::Server;

fn main() {
    let server = Server::new("en0".to_string());
    server.start();
}
