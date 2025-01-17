//luma_db/lumadb_parser/src/parser.rs
use chumsky::prelude::*;
use crate::select::select_parser;
use lumadb_core::token::Token;
//Decide which parser to use, combining functionality of all Select.rs-like files

fn decide_command(tokens: &Vec<Token>) -> Option<&Token> {
    //obtain initial keyword used to select parser
    tokens.first()
}

fn parse_input(tokens: &Vec<Token>) -> impl Parser<char, Expr, Error = Simple<char>>{
    //decide the command to parse
    let command = decide_command(tokens);

    match command { 
        Some(Token::Select) => {
            let parser = select_parser();
            parser
        }
        _ => Err("Unsupported command while parsing.".to_string()),
    }
    
}