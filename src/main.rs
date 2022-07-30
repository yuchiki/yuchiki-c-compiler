mod lex;
use lex::{PositionedToken, SourcePosition, Token};

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

    let tokens = lex::tokenize(&input);
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

fn error(error_message: &str, pos: lex::SourcePosition, input: &str) -> ! {
    eprintln!("{input}\n{:width$}^{error_message}", "", width = pos.0);
    panic!("compile error")
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
