#[derive(Debug, Copy, Clone)]
struct SourcePosition(pub usize);

type PositionedToken = (Token, SourcePosition);

#[derive(Debug)]
enum Token {
    Num(i32),
    Plus,
    Minus,
    Asterisk,
}

#[derive(Debug)]
enum Expr {
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Num(i32),
}

fn main() {
    let raw_input = std::env::args().nth(1).expect("no arguments");
    let input = raw_input.chars().collect::<Vec<_>>();

    let tokens = tokenize(&input);
    let tokens = &tokens[..];

    let (expr, tokens) = munch_expr(tokens);
    if !tokens.is_empty() {
        panic!("parseした後にtokensがあまっている");
    }

    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");

    gen_expr(expr);

    println!("  pop rax");
    println!("  ret");
}

fn error(error_message: &str, pos: SourcePosition, input: &str) -> ! {
    eprintln!("{input}\n{:width$}^{error_message}", "", width = pos.0);
    panic!("compile error")
}

fn tokenize(mut input: &[char]) -> Vec<PositionedToken> {
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
            ['0'..='9', ..] => {
                if let (rest, Some(num), char_count) = munch_int(input) {
                    ans.push((Token::Num(num), pos));
                    input = rest;
                    pos.0 += char_count;
                }
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

fn munch_expr(tokens: &[PositionedToken]) -> (Expr, &[PositionedToken]) {
    let (mut expr, mut tokens) = munch_mul(tokens);

    loop {
        match tokens {
            [(Token::Plus, _), rest @ ..] => {
                let (rhs, new_tokens) = munch_mul(rest);
                expr = Expr::Add(Box::new(expr), Box::new(rhs));
                tokens = new_tokens;
            }
            [(Token::Minus, _), rest @ ..] => {
                let (rhs, new_tokens) = munch_mul(rest);
                expr = Expr::Sub(Box::new(expr), Box::new(rhs));
                tokens = new_tokens;
            }
            _ => return (expr, tokens),
        }
    }
}

fn munch_mul(tokens: &[PositionedToken]) -> (Expr, &[PositionedToken]) {
    let (mut expr, mut tokens) = munch_primary(tokens);

    loop {
        match tokens {
            [(Token::Asterisk, _), rest @ ..] => {
                let (rhs, new_tokens) = munch_primary(rest);
                expr = Expr::Mul(Box::new(expr), Box::new(rhs));
                tokens = new_tokens;
            }
            _ => return (expr, tokens),
        }
    }
}

fn munch_primary(tokens: &[PositionedToken]) -> (Expr, &[PositionedToken]) {
    match tokens {
        [(Token::Num(num), _), rest @ ..] => (Expr::Num(*num), rest),
        [(_, pos), ..] => {
            panic!("parse error at {:?}", pos);
        }
        [] => panic!("tokens are empty."),
    }
}

fn gen_expr(expr: Expr) {
    match expr {
        Expr::Num(n) => {
            println!("  push {n}");
        }
        Expr::Add(lhs, rhs) => {
            gen_expr(*lhs);
            gen_expr(*rhs);

            println!("  pop rdi");
            println!("  pop rax");

            println!("  add rax, rdi");
            println!("  push rax");
        }

        Expr::Sub(lhs, rhs) => {
            gen_expr(*lhs);
            gen_expr(*rhs);

            println!("  pop rdi");
            println!("  pop rax");

            println!("  sub rax, rdi");
            println!("  push rax");
        }
        Expr::Mul(lhs, rhs) => {
            gen_expr(*lhs);
            gen_expr(*rhs);

            println!("  pop rdi");
            println!("  pop rax");

            println!("  imul rax, rdi");
            println!("  push rax");
        }
    }
}
