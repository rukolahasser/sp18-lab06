use clap::{Parser};
use sp18_lab06::server::Server;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Optional name to operate on
    port: Option<u16>,
}

fn main() {
    let cli = Cli::parse();

    let p = match cli.port {
        Some(port) => port,
        _ => 12345,
    };

    let binding = hostname::get().unwrap();
    let host = binding.to_str().unwrap();
    let mut sock = Server::new(&host, p, "tcp");
    sock.listen(5);
    sock.close();
}
