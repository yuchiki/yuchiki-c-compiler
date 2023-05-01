#[derive(Debug, Copy, Clone)]
pub struct SourcePosition(pub usize);

pub type PositionedToken = (Token, SourcePosition);

#[derive(Debug)]
pub enum Token {
    Num(i32),
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
}

pub fn tokenize(mut input: &[char]) -> Vec<PositionedToken> {
    let mut ans = vec![];
    let mut pos = SourcePosition(0);

    while !input.is_empty() {
        match input {
            ['+', rest @ ..] => {
                ans.push((Token::Plus, pos));
                input = rest;
                pos.0 += 1;
            }
            ['-', rest @ ..] => {
                ans.push((Token::Minus, pos));
                input = rest;
                pos.0 += 1;
            }
            ['*', rest @ ..] => {
                ans.push((Token::Asterisk, pos));
                input = rest;
                pos.0 += 1;
            }
            ['/', rest @ ..] => {
                ans.push((Token::Slash, pos));
                input = rest;
                pos.0 += 1;
            }
            ['(', rest @ ..] => {
                ans.push((Token::LParen, pos));
                input = rest;
                pos.0 += 1;
            }
            [')', rest @ ..] => {
                ans.push((Token::RParen, pos));
                input = rest;
                pos.0 += 1;
            }
            ['=', '=', rest @ ..] => {
                ans.push((Token::Equality, pos));
                input = rest;
                pos.0 += 2;
            }
            ['!', '=', rest @ ..] => {
                ans.push((Token::Inequality, pos));
                input = rest;
                pos.0 += 2;
            }
            ['<', '=', rest @ ..] => {
                ans.push((Token::LessThanOrEqual, pos));
                input = rest;
                pos.0 += 2;
            }
            ['>', '=', rest @ ..] => {
                ans.push((Token::GreaterThanOrEqual, pos));
                input = rest;
                pos.0 += 2;
            }
            ['<', rest @ ..] => {
                ans.push((Token::LessThan, pos));
                input = rest;
                pos.0 += 1;
            }
            ['>', rest @ ..] => {
                ans.push((Token::GreaterThan, pos));
                input = rest;
                pos.0 += 1;
            }
            ['0'..='9', ..] => {
                if let (rest, Some(num), char_count) = munch_int(input) {
                    ans.push((Token::Num(num), pos));
                    input = rest;
                    pos.0 += char_count;
                }
            }
            ['=', rest @ ..] => {
                ans.push((Token::Assign, pos));
                input = rest;
                pos.0 += 1;
            }
            [';', rest @ ..] => {
                ans.push((Token::Semicolon, pos));
                input = rest;
                pos.0 += 1;
            }
            ['\t', rest @ ..] => {
                input = rest;
            }
            [' ', rest @ ..] => {
                input = rest;
                pos.0 += 1;
            }
            ['\n', rest @ ..] => {
                input = rest;
                pos.0 = 0;
            }
            _ => {
                panic!("tokenize error");
            }
        }
    }

    ans
}

fn munch_int(mut input: &[char]) -> (&[char], Option<i32>, usize) {
    let mut char_count = 0;

    if let ['0'..='9', ..] = input {
        let mut ans = 0;
        while let [digit @ '0'..='9', rest @ ..] = input {
            ans = ans * 10 + (*digit as i32) - ('0' as i32);
            input = rest;
            char_count += 1;
        }

        (input, Some(ans), char_count)
    } else {
        (input, None, 0)
    }
}
