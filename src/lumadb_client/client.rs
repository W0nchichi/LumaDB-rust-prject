use std::io::prelude::*;
use std::net::TcpStream;
use lumadb::config::DEFAULT_CONNECTION;
use std::io::{self, Write, BufRead};

//this code is just rehashed from the REPL without the ';' rules
//also only need to take 1 line inputs
fn main_user_input_loop() -> String {
    //take the input
    let stdin = io::stdin();

    let mut handle = stdin.lock();

    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();

    handle.read_line(&mut input).expect("Failed to read input");
    return input.trim_end_matches(&['\n', '\r'][..]).to_string();
}
fn main() -> std::io::Result<()> {
    print!("{}", DEFAULT_CONNECTION);
    let mut stream = TcpStream::connect(DEFAULT_CONNECTION)?;

    let response: String = main_user_input_loop();

    stream.write_all(response.as_bytes())?;
    Ok(())
}
