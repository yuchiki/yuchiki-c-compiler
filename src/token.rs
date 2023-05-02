#[derive(Debug)]
pub enum Token {
    Num(i32),
    Identifier(String),
    Plus,
    Minus,
    Asterisk,
    Slash,
    LParen,
    RParen,
    Equality,
    Inequality,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    Assign,
    Semicolon,
    Return,
    If,
    Else,
    While,
}
