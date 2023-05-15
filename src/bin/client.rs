use std::path::PathBuf;
use clap::{Parser, Subcommand};
use sp18_lab06::client::Client;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    port: Option<u16>,
    message: Option<String>
}


fn main() {
    let cli = Cli::parse();

    let p = match cli.port {
        Some(port) => port,
        _ => 12345,
    };

    let m = match cli.message {
        Some(msg) => msg,
        _ => "PING".to_string()
    };

    dbg!(hostname::get().unwrap());

    let binding = hostname::get().unwrap();
    let host = binding.to_str().unwrap();
    let mut sock = Client::new(host, p, "tcp");
    sock.push(&m);
    sock.get_msg();
    sock.close();
}