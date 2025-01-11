    //src/lumabdb_client/client.rs

    use std::io::prelude::*;
    use std::net::TcpStream;
    use std::io::{self, Write, BufRead};
    //need to use luma_db instead of a crate because it isn't a crate, is a module, check Cargo.toml
    use lumadb_client::repl::Repl;
    use lumadb::config::DEFAULT_CONNECTION;



    //this code is just rehashed from the REPL without the ';' rules
    //also only need to take 1 line inputs
    //got too repetitive to do this manually lol
fn get_user_input(server_prompt: &str) -> String {
    print!("{}", server_prompt);
    //still drk what flushing is
    io::stdout().flush().expect("failed to flush");
    let mut input = String::new();
    //get input
    io::stdin().lock().read_line(&mut input).expect("failed to read input");
    return input.trim().to_string();
}


//changed to a reference of a stream that instantiated in the main
fn handle_authentication(mut stream: &TcpStream) -> io::Result<bool> {
    let mut buffer = [0; 512];
    //this is where actual connections can be written to back and forth :3
    //first ask for a username, take an error if the connection is prematurely ended
    let bytes_read = stream.read(&mut buffer)?;
    println!("{}", String::from_utf8_lossy(&buffer[..bytes_read]));
    let username = get_user_input("> ");
    stream.write_all(username.as_bytes())?;

    //password same deal
    let bytes_read = stream.read(&mut buffer)?;
    println!("{}", String::from_utf8_lossy(&buffer[..bytes_read]));
    let password = get_user_input("> ");
    stream.write_all(password.as_bytes())?;

    // Receive authentication response hopefully
    let bytes_read = stream.read(&mut buffer)?;
    let response = String::from_utf8_lossy(&buffer[..bytes_read]).trim().to_string();
    println!("Server: {}", response);
    //needed to add 'return' as compiler wasn't recognizing this was an implicit return
    return Ok(response == "User Authorized");
}

pub fn repl_loop(mut stream: TcpStream) -> io::Result<()> {
    loop {
        //instaitiate rpel afterwards otherwise will keep extending inputs
        let mut repl = Repl::new();
        let input = repl.main_loop();
        if input.trim() == "exit;" {
            break;
        }
        stream.write_all(input.as_bytes())?;
        
        let mut buffer = [0; 512];
        let bytes_read = stream.read(&mut buffer)?;
        if bytes_read > 0 {
            println!("Server: {}", String::from_utf8_lossy(&buffer[..bytes_read]).trim());
        }
    }
    Ok(())
}

pub fn main() -> io::Result<()> {
    let mut stream = TcpStream::connect(DEFAULT_CONNECTION)?;
    if handle_authentication(&stream)? {
        println!("Authentication successful. Entering REPL...");
        repl_loop(stream)?;
    } else {
        println!("Authentication failed. Exiting.");
    }
    Ok(())
}
