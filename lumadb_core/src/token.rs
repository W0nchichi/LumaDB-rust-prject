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

impl Token {
    pub fn to_string(&self) -> String {
        match self {
            Token::Select => "SELECT".to_string(),
            Token::Insert => "INSERT".to_string(),
            Token::Into => "INTO".to_string(),
            Token::Delete => "DELETE".to_string(),
            Token::Where => "WHERE".to_string(),
            Token::From => "FROM".to_string(),
            Token::Not => "NOT".to_string(),
            Token::As => "AS".to_string(),
            Token::And => "AND".to_string(),
            Token::Or => "OR".to_string(),
            Token::GroupBy => "GROUP BY".to_string(),
            Token::Identifier(ident) => ident.clone(),
            Token::StringLiteral(lit) => format!("'{}'", lit),
            Token::NumericLiteral(num) => num.to_string(),
            Token::Equals => "=".to_string(),
            Token::NotEquals => "!=".to_string(),
            Token::GreaterThan => ">".to_string(),
            Token::LessThan => "<".to_string(),
            Token::GreaterThanOrEqual => ">=".to_string(),
            Token::LessThanOrEqual => "<=".to_string(),
            Token::Comma => ",".to_string(),
            Token::Semicolon => ";".to_string(),
            Token::LeftParen => "(".to_string(),
            Token::RightParen => ")".to_string(),
            Token::Asterisk => "*".to_string(),
            Token::EOF => "".to_string(),
            Token::Illegal(c) => c.to_string(),
        }
    }
}
