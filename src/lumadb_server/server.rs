use std::io::prelude::*;
use std::io::Result;
use std::net::TcpListener;
use std::net::TcpStream;
use lumadb::config::DEFAULT_CONNECTION;

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    //this is where actual connections can be written to back and forth :3
    //first ask for a username, take an error if the connection is prematurely ended
    if let Err(e) = stream.write_all(b"Please enter your username: ") {
        eprintln!("Error sending username prompt: {}", e);
        return;
    }

    if let Ok(bytes_read) = stream.read(&mut buffer) {
        let username = String::from_utf8_lossy(&buffer[..bytes_read]).trim().to_string();
        //replace with actual authentication later
        println!("Received username: {}", username);
    } else {
        eprintln!("Failed to read username.");
        return;
    }

    // Clear the buffer for password
    buffer.fill(0);

    //ask for a password
    if let Err(e) = stream.write_all(b"Please enter your password: ") {
        eprintln!("Error sending password prompt: {}", e);
        return;
    }

    if let Ok(bytes_read) = stream.read(&mut buffer) {
        let password = String::from_utf8_lossy(&buffer[..bytes_read]).trim().to_string();
        println!("Received password: {}", password);
    } else {
        eprintln!("Failed to read password.");
        return;
    }

    if let Err(e) = stream.write_all(b"*hacker voide* I'm in.") {
        eprintln!("Error sending authorization {}", e);
    }

    // Clear the buffer for cleanliness
    buffer.fill(0);
}

pub fn create_listener(addr: String) -> Result<()> {
    let listener = TcpListener::bind(addr)?;
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
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
