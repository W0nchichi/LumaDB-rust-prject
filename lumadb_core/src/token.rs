// luma_db/lumadb_core/src/token.rs

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // Keywords
    Select,
    Insert,
    Into,
    Delete,
    Where,
    From,
    Not,
    As,
    And,
    Or,
    GroupBy,
    
    // Identifiers
    Identifier(String),
    
    // Literals
    StringLiteral(String),
    NumericLiteral(f64),
    
    // Operators
    Equals,
    NotEquals,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
    
    // Punctuation
    Comma,
    Semicolon,
    LeftParen,
    RightParen,
    Asterisk,
    
    // End of Input
    EOF,
    
    // Errors
    Illegal(char),
}
