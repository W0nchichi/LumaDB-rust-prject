use std::io::prelude::*;
use std::net::TcpStream;
use lumadb::config::DEFAULT_CONNECTION;

fn main() -> std::io::Result<()> {
    print!(DEFAULT_CONNECTION);
    let mut steam = TcpStream::connect(DEFAULT_CONNECTION);
    steam.write("haii");
}