use std::io::{Read, Write};
use std::io::stdin;
use std::io;
use std::net::{TcpListener, TcpStream};
use lumadb::config::DEFAULT_CONNECTION;
use lumadb_core::tokenizer::Tokenizer;
use lumadb_core::token::Token;
use lumadb_core::parser::Parser;

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

fn check_for_lost_connection(mut stream: &TcpStream, buffer: &mut [u8; 512]) -> bool{
    match stream.read(buffer) {
        Ok(bytes_read) => {
            if bytes_read == 0 {
                // No bytes read indicates the connection is closed
                false
            } else {
                // Connection is still active
                true
            }
        }
        Err(_) => {
            // An error occurred while reading from the stream
            false
        }
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

fn server_repl_loop(mut stream: &mut TcpStream){
    //Arnold Schwatznegger voice "grah, I am de tokenator"
    //that felt disgusting to type out
    
    loop {
        let select = Parser();
        let repl_input = pass_repl_input(stream).expect("could not recieve repl stuff");
        println!("{}", repl_input);
        let mut tokenator = Tokenizer::new(&repl_input);
        //loop for tokens until it is done, {:?} handles all errors
        let tokenated_line = tokenator.tokenize_all().expect("Could not tokenize line");
        println!("{:?}", tokenated_line);
        stream.write_all(format!("{:?}", tokenated_line).as_bytes())
                .map_err(|e| e.to_string())
                .expect("could not write");
        let result = select.parse(tokenated_line);
        match result {
            Ok(statement) => println!("Parsed successfully: {:?}", statement),
            Err(errors) => {
                for error in errors {
                    println!("Error: {}", error);
                }
            }
        }
        
    }
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
    //so it seems that the repl seems to freeze after the first input so fixed with function
    server_repl_loop(&mut stream);
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
