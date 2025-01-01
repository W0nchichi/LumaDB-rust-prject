// src/main.rs

mod dbms_rust_project; // Declare the module (matches directory name)

use crate::dbms_rust_project::Repl;
use crate::dbms_rust_project::Tokenizer;
use crate::dbms_rust_project::Token;

fn main() {
    let mut repl = Repl::new(); // Instantiate the Repl object
    let   String = repl.main_loop(); // Start the REPL and get the input
    let mut tokenizer: Tokenizer<'_> = Tokenizer::new(&s);
    println!("");
    match tokenizer.tokenize_all() {
        Ok(tokens) => {
            for token in tokens {
                println!("{:?}", token);
            }
        }
        Err(e) => {
            eprintln!("Lexer error: {:?}", e);
        }
    }
}


