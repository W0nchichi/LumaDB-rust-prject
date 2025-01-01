// src/main.rs
mod lumadb_server {pub mod server;}
use std::io::Result;
use lumadb::config::DEFAULT_CONNECTION;

fn main() {
    server::create_listener(String::from(DEFAULT_CONNECTION))
}


