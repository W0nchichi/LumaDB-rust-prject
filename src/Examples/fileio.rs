use std::fs;
use std::path::Path;
use crate::dbms_rust_project::config::DEFAULT_DIR;

fn main() {
    let data = "Some data!";
    //write the data
    fs::write(DEFAULT_DIR, data).expect("Unable to write file");
    //now read it
    fs::read(DEFAULT_DIR).expect("Unable to read it :3");
}