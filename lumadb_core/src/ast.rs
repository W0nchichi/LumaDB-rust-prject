// src/ast.rs

#[derive(Debug, PartialEq, Eq)]
pub enum Statement {
    Insert {
        table: String,
        values: Vec<i32>,
    },
    // Extend this with more SQL statements as needed
    Select {
        table: String,
        columns: Vec<String>,
        conditions: Option<String>, // Example for a simple WHERE clause
    },
    Update {
        table: String,
        values: Vec<(String, String)>, // (column, new_value)
        conditions: Option<String>,   // Example for a simple WHERE clause
    },
    Delete {
        table: String,
        conditions: Option<String>,   // Example for a simple WHERE clause
    },
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