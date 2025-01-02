use std::io::prelude::*;
use std::io::Result;
use std::net::TcpListener;
use std::net::TcpStream;
use lumadb::config::DEFAULT_CONNECTION;

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    match stream.read(&mut buffer) {
        Ok(bytes_read) if bytes_read > 0 => {
            println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
        }
        Ok(_) => {
            println!("Client sent an empty response");
        }
        Err(e) => {
            eprintln!("Failed to read from stream: {}", e);
        }
    }
}

pub fn create_listener(addr: String) -> Result<()> {
    let listener = TcpListener::bind(addr)?;
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection established: {:?}", stream.peer_addr());
                handle_connection(stream);
            }
            Err(e) => eprintln!("Failed to establish connection: {}", e),
        }
    }
    Ok(())
}

fn main() {
    create_listener(DEFAULT_CONNECTION.to_string()).expect("Could not create connection");
}
