use crate::token::Token;

#[derive(Debug, Copy, Clone)]
pub struct SourcePosition(pub usize);

pub type PositionedToken = (Token, SourcePosition);

static TOKEN_MAP: [(&str, Token); 22] = [
    ("+", Token::Plus),
    ("-", Token::Minus),
    ("*", Token::Asterisk),
    ("/", Token::Slash),
    ("(", Token::LParen),
    (")", Token::RParen),
    ("{", Token::LBrace),
    ("}", Token::RBrace),
    (",", Token::Comma),
    ("==", Token::Equality),
    ("!=", Token::Inequality),
    ("<=", Token::LessThanOrEqual),
    ("<", Token::LessThan),
    (">=", Token::GreaterThanOrEqual),
    (">", Token::GreaterThan),
    (";", Token::Semicolon),
    ("=", Token::Assign),
    ("if", Token::If),
    ("else", Token::Else),
    ("while", Token::While),
    ("for", Token::For),
    ("return", Token::Return),
];

pub fn tokenize(input: &[char]) -> Vec<PositionedToken> {
    let mut ans: Vec<PositionedToken> = vec![];
    let mut pos = SourcePosition(0);

    while !input[pos.0..].is_empty() {
        if let Some((length, token)) = TOKEN_MAP
            .iter()
            .find(|(key, _)| input[pos.0..].starts_with(&key.chars().collect::<Vec<char>>()))
            .map(|(key, token)| (key.len(), token))
        {
            ans.push((token.clone(), pos));
            pos.0 += length;
        } else if input[pos.0].is_ascii_digit() {
            let (Some(num), length) = munch_int(&input[pos.0..]) else {
                panic!("invalid number: {}", input[0]);
            };

            ans.push((Token::Num(num), pos));
            pos.0 += length;
        } else if input[pos.0].is_ascii_alphabetic() {
            let (Some(identifier), length) = munch_identifier(&input[pos.0..]) else {
                panic!("invalid identifier: {}", input[0]);
            };

            ans.push((Token::Identifier(identifier), pos));
            pos.0 += length;
        } else if input[pos.0].is_ascii_whitespace() {
            pos.0 += 1;
        } else {
            panic!("invalid character: at {}", input[pos.0]);
        }
    }
    ans
}

const fn munch_int(mut input: &[char]) -> (Option<i32>, usize) {
    let mut char_count = 0;

    if let ['0'..='9', ..] = input {
        let mut ans = 0;
        while let [digit @ '0'..='9', rest @ ..] = input {
            ans = ans * 10 + (*digit as i32) - ('0' as i32);
            input = rest;
            char_count += 1;
        }

        (Some(ans), char_count)
    } else {
        (None, 0)
    }
}

fn munch_identifier(mut input: &[char]) -> (Option<String>, usize) {
    let mut char_count = 0;

    if let ['a'..='z', ..] = input {
        let mut ans = String::new();
        while let [alpha @ ('a'..='z' | '0'..='9' | '_'), rest @ ..] = input {
            ans.push(*alpha);
            input = rest;
            char_count += 1;
        }

        (Some(ans), char_count)
    } else {
        (None, 0)
    }
}
