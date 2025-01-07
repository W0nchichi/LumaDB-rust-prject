// src/ast.rs

#[derive(Debug, PartialEq)]
pub enum Statement {
    Select {
        columns: Vec<Column>,
        table: String,
        where_clause: Option<Condition>,
    },
    // Need to add more statements like Insert, Delete, etc.
}

#[derive(Debug, PartialEq)]
pub enum Column {
    All,
    Name(String),
}

#[derive(Debug, PartialEq)]
pub struct Condition {
    pub left: Box<Expression>,
    pub operator: Operator,
    pub right: Box<Expression>,
}

#[derive(Debug, PartialEq)]
pub enum Operator {
    GreaterEquals,
    LessEquals,
    GreaterThan,
    LessThan,
    Equals,
    NotEquals,
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Identifier(String),
    NumericLiteral(f64),
    StringLiteral(String),
}