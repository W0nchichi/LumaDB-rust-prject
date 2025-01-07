//src/lumabdb_client/client.rs

use std::io::prelude::*;
use std::net::TcpStream;
use std::io::{self, Write, BufRead};
//need to use luma_db instead of a crate because it isn't a crate, is a module, check Cargo.toml
use lumadb_client::repl::Repl;
use lumadb::config::DEFAULT_CONNECTION;



//this code is just rehashed from the REPL without the ';' rules
//also only need to take 1 line inputs
pub fn main_user_input_loop() -> String {
    //take the input
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    let mut handle = stdin.lock();

    stdout.flush().expect("Failed to flush stdout");

    let mut input = String::new();

    handle.read_line(&mut input).expect("Failed to read input");
    return input.trim().to_string();
}

pub fn main() -> std::io::Result<()> {
    println!("{}", DEFAULT_CONNECTION);
    let mut stream = TcpStream::connect(DEFAULT_CONNECTION)?;

    //read the incoming prompts
    let mut buffer = [0; 512];
    loop{
        let bytes_read = stream.read(&mut buffer)?;

        //>> to prompt user input same as repl
        print!(">>");

        if bytes_read == 0 {
            //authentication has let pass, and then pass to the REPL
            loop {
                let mut repl = Repl::new();
                let input = repl.main_loop();
                if input == "exit" {
                    break
                }
                //send message to server
            }
            break;
        }
        
        //god I love actually readable code with comments
        let server_message = String::from_utf8_lossy(&buffer[..bytes_read]).trim().to_string();
        println!("Server: {}", server_message);

        // Exit if the server sends a specific termination message
        if server_message.to_lowercase() == "User Authorized" {
            println!("Server requested to close the connection. Goodbye!");
            break;
        }

        // Get the user's input and send it to the server
        let user_input = main_user_input_loop();
        stream.write_all(user_input.as_bytes())?;
    }
    
    Ok(())
}
