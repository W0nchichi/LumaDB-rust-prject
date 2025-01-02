use std::io::prelude::*;
use std::net::TcpStream;
use lumadb::config::DEFAULT_CONNECTION;

fn main() -> std::io::Result<()> {
    print!("{}", DEFAULT_CONNECTION);
    let mut stream = TcpStream::connect(DEFAULT_CONNECTION)?;
    stream.write_all(b"haii")?;
    Ok(())
}
