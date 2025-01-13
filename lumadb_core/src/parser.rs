use chumsky::prelude::*;

#[derive(Debug)]
pub enum Statement {
    Select {
        columns: Vec<String>,
        table: String,
    },
}

pub trait Parser {
    fn parse(&self, input: &str) -> Result<Statement, Error>;
}

pub fn parser() -> impl Parser<char, Statement, Error = Simple<char>> {
    // Parser for identifiers (columns or table names)
    let identifier = text::ident().padded();

    // Parser for SELECT keyword
    let select_keyword = just("SELECT").padded();

    // Parser for FROM keyword
    let from_keyword = just("FROM").padded();

    // Parser for a single column or multiple comma-separated columns
    let column_list = identifier
        .clone()
        .separated_by(just(",").padded())
        .delimited_by(select_keyword, from_keyword);

    // Parser for the table name
    let table_name = identifier;

    // Parser for the complete SELECT statement
    column_list
        .then(table_name)
        .then_ignore(just(";").padded())
        .map(|(columns, table)| Statement::Select { columns, table })
}
