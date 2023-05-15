use std::io::{Read, Write};
use std::net::{TcpStream, SocketAddr};
use std::process;
use crate::color_utils::{bprint, gprint};

pub struct Client {
    peers: Vec<SocketAddr>,
    online: bool,
    protocol: String,
    sock: TcpStream,
}

impl Client {
    pub fn new(host: &str, port: u16, protocol: &str) -> Client {
        let addr = format!("{}:{}", host, port);
        let sock = match TcpStream::connect(addr) {
            Ok(stream) => stream,
            Err(_) => {
                eprintln!("Connection refused...");
                process::exit(1);
            }
        };
        Client {
            peers: Vec::new(),
            online: true,
            protocol: String::from(protocol),
            sock,
        }
    }

    pub fn push(&mut self, m: &str) {
        bprint("Sending message: {}", m);
        self.sock.write_all(m.as_bytes()).unwrap();
    }

    pub fn get_msg(&mut self) -> String {
        assert!(self.online);
        let mut buffer = String::new();
        self.sock.read_to_string(&mut buffer).unwrap();
        gprint("Received message: {}", &buffer);
        buffer
    }

    pub fn close(&mut self) {
        assert!(self.online);
        self.sock.shutdown(std::net::Shutdown::Both).unwrap();
        self.online = false;
    }
}