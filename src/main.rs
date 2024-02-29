mod server;
use server::Server;

fn main() {
    let server = Server::new("enp3s0".to_string());
    server.start();
}
