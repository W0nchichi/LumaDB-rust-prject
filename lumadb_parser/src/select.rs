//luma_db/lumadb_parser/src/select.rs

use chumsky::prelude::*;
use crate::command::{command, WhereClause};

//to run function, use select_parser().parse(input)
//it's also so wild not using a parameter for this and i'm tweaking
pub fn select_parser() -> impl Parser<char, Expr, Error = Simple<char>> {
    // Parser for identifiers (column names, table names)
    let identifier = text::ident().padded();


}