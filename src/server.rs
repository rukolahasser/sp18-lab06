use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream, SocketAddr};
use std::process;
use std::collections::HashMap;
use crate::color_utils::{bprint, gprint};


pub struct Server {
    clients: HashMap<SocketAddr, String>,
    order: Vec<SocketAddr>,
    files: Vec<String>,
    online: bool,
    protocol: String,
    listener: TcpListener,
}

impl Server {
    pub fn new(host: &str, port: u16, protocol: &str) -> Server {
        let addr = format!("{}:{}", host, port);
        let listener = match TcpListener::bind(addr) {
            Ok(listener) => listener,
            Err(_) => {
                eprintln!("Connection refused...");
                process::exit(1);
            }
        };
        Server {
            clients: HashMap::new(),
            order: Vec::new(),
            files: Vec::new(),
            online: true,
            protocol: String::from(protocol),
            listener,
        }
    }

    pub fn listen(&mut self, limit: u32) {
        assert!(self.online);
        println!("Listening...");
        let r = self.listener.try_clone().unwrap();
        for result in r.incoming().take(limit as usize) {
            match result {
                Ok(stream) => {
                    let addr = stream.peer_addr().unwrap();
                    gprint("Got connection from {}", &addr.to_string());
                    let mut client = stream;
                    let message = self.ret_msg(&mut client);
                    let response = self.process(addr, &message);
                    self.respond(&mut client, &response);
                }
                Err(e) => {
                    eprintln!("Failed to establish a connection: {}", e);
                    break;
                }
            }
        }
        println!("Socket closed.");
    }

    fn process(&mut self, addr: SocketAddr, m: &str) -> String {
        self.clients.insert(addr, m.to_string());
        self.order.push(addr);

        if m == "PING" {
            "PONG".to_string()
        } else if m.starts_with("QUERY") {
            let file = m.split_whitespace().nth(1);
            if let Some(file) = file {
                if self.files.contains(&file.to_string()) {
                    "QUERYHIT".to_string()
                } else {
                    "NOT FOUND".to_string()
                }
            } else {
                "hello, World!".to_string()
            }
        } else {
            "hello, World!".to_string()
        }
    }

    fn respond(&self, stream: &mut TcpStream, m: &str) {
        bprint("Sending message: {}", m);
        stream.write_all(m.as_bytes()).unwrap();
    }

    fn ret_msg(&self, stream: &mut TcpStream) -> String {
        assert!(self.online);
        let mut buffer = String::new();
        stream.read_to_string(&mut buffer).unwrap();
        gprint("Received message: {}", &buffer);
        buffer
    }

    pub fn close(&mut self) {
        assert!(self.online);
        self.online = false;
        drop(&self.listener);
    }
}
