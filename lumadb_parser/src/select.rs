//luma_db/lumadb_parser/src/select.rs

use chumsky::prelude::*;
use chumsky::Parser;
use crate::commands::{command, WhereClause};
use lumadb_core::token::Token;

//to run function, use select_parser().parse(input)
//it's also so wild not using a parameter for this and i'm tweaking
pub fn select_parser() -> impl Parser<Token, command, Error = Simple<Token>> {
    // Parser for identifiers (column names, table names)
    let identifier = text::ident().padded();

    //the <Select s> and <From f> parts are necessary so they can get put first, where is optional
    //as described in the enum
    // Parser for comma-separated column names or just a  '*'
    let columns = just(Token::Asterisk).map(|_| vec!["*".to_string()])
        .or(filter_map(|span, token| match token {
            Token::Identifier(name) => Ok(name),
            _ => Err(Simple::custom(span, "Expected column identifier")),
        })
        .separated_by(just(Token::Comma)))
        .map(|columns| command::Select { columns });

    let from_clause = just("FROM")
        //don't need the FROM part
        // identifier = Table name
        .ignore_then(identifier) 
        // check for the optional alias "table <t>"
        .then(identifier.chars().next().or_not()) 
        //map the result to the Enum of the From Command found in commands.rs
        //this mapping shit is wild to me
        .map(|(table, alias)| command::From { table, alias });
    
    //so, since the where part is optional, we'll set up the logic here in case, 
    //and actually handle the select itself after.
    let where_clause = just("WHERE")
        //this kind of works with the structure of the single vs. multiple condition
        //where clause stuff
        .ignore_then(where_clause_parser())
        .or_not();

    //The big boy now
    let select = just("SELECT")
        .ignore_then(columns)
        .then(from_clause)
        .then(where_clause.or_not())
        .map(|((columns, table), where_clause)| command::Select {
            columns,
            table,
            where_clause,
        });
}

//Need a where clause parser as well
pub fn where_parser() ->impl Parser<char, WhereClause, Error = Simple<char>> {
    // Parser for identifiers (variables, comparators, etc.)
    let identifier = text::ident().padded();
    //take all operators it can be
    let operator = just("=").or(just(">").or(just("<"))).padded();
    //check if there are any digits
    let value = text::ident().or(text::digits(10)).padded().or_not();

    let single_condition = identifier
        .then(operator)
        .then(value)
        .map(|((column, operator), value)| WhereClause::SingleCondition {
            column,
            operator,
            value,
        });

    let compound_condition = single_condition
        .separated_by(just("AND").or(just("OR")).padded())
        .map(|conditions| WhereClause::CompoundCondition {
            conditions,
            conjunction: "AND".to_string(), // Replace with actual conjunction
        });

    single_condition.or(compound_condition)
}