use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use lumadb::config::DEFAULT_CONNECTION;

fn authenticate(mut stream: &mut TcpStream) -> Result<bool, String> {
    let mut buffer = [0; 512];
    
    // Send username prompt
    //use map_err now to simplify catching the error shit
    stream.write_all(b"Please enter your username: ").map_err(|e| e.to_string())?;
    let bytes_read = stream.read(&mut buffer).map_err(|e| e.to_string())?;
    if bytes_read == 0 {
        return Err("Client disconnected during username input.".into());
    }
    let username = String::from_utf8_lossy(&buffer[..bytes_read]).trim().to_string();
    
    // Send password prompt
    buffer.fill(0);
    stream.write_all(b"Please enter your password: ").map_err(|e| e.to_string())?;
    let bytes_read = stream.read(&mut buffer).map_err(|e| e.to_string())?;
    if bytes_read == 0 {
        return Err("Client disconnected during password input.".into());
    }
    let password = String::from_utf8_lossy(&buffer[..bytes_read]).trim().to_string();

    // Simulate authentication (replace with actual logic)
    if username == "admin" && password == "password" {
        stream.write_all(b"User Authorized").map_err(|e| e.to_string())?;
        Ok(true)
    } else {
        stream.write_all(b"Authentication Failed").map_err(|e| e.to_string())?;
        Ok(false)
    }
}

//getting crazy with it
fn pass_repl_input(mut stream: &mut TcpStream) -> Result<String, String> {
    let mut buffer = [0; 512];

    // Read data from the stream
    let bytes_read = stream.read(&mut buffer).map_err(|e| e.to_string())?;
    
    if bytes_read == 0 {
        return Err("Client disconnected during REPL input.".into());
    }

    // Convert buffer to a trimmed UTF-8 string
    let input = String::from_utf8_lossy(&buffer[..bytes_read]).trim().to_string();
    Ok(input)
}

fn handle_connection(mut stream: TcpStream) {
    match authenticate(&mut stream) {
        Ok(true) => {
            println!("Client authenticated successfully.");
            // Additional REPL or command handling logic can go here.
        }
        Ok(false) => println!("Client failed authentication."),
        Err(e) => eprintln!("Error during authentication: {}", e),
    }
    let repl_input = pass_repl_input(&mut stream).expect("could not get repl");
    println!("{}", repl_input);

}

//use a borrowed pointer to make easier
pub fn create_listener(addr: &String) -> std::io::Result<()> {
    let listener = TcpListener::bind(addr)?;
    println!("Server is listening on {}", addr);
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection established: {:?}", stream.peer_addr());
                handle_connection(stream);
            }
            Err(e) => eprintln!("Connection error: {}", e),
        }
    }
    Ok(())
}

fn main() {
    if let Err(e) = create_listener(&DEFAULT_CONNECTION.to_string()) {
        eprintln!("Error starting server: {}", e);
    }
}
