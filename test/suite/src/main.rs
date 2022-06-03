mod server;
mod process_wrapper;

fn main() {
    let server = server::Server::start();
    server.stop();
}
