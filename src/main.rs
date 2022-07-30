mod lex;
use lex::{PositionedToken, SourcePosition, Token};

struct ParserState<'a> {
    tokens: &'a [PositionedToken],
}

impl<'a> ParserState<'a> {
    fn advance(&mut self, offset: usize) {
        self.tokens = &self.tokens[offset..]
    }

    fn munch_expr(&mut self) -> Expr {
        let mut expr = self.munch_mul();

        loop {
            match self.tokens {
                [(Token::Plus, _), ..] => {
                    self.advance(1);
                    let rhs = self.munch_mul();
                    expr = Expr::Add(Box::new(expr), Box::new(rhs));
                }
                [(Token::Minus, _), ..] => {
                    self.advance(1);
                    let rhs = self.munch_mul();
                    expr = Expr::Sub(Box::new(expr), Box::new(rhs));
                }
                _ => return expr,
            }
        }
    }

    fn munch_mul(&mut self) -> Expr {
        let mut expr = self.munch_primary();

        loop {
            match self.tokens {
                [(Token::Asterisk, _), ..] => {
                    self.advance(1);
                    let rhs = self.munch_primary();
                    expr = Expr::Mul(Box::new(expr), Box::new(rhs));
                }
                _ => return expr,
            }
        }
    }

    fn munch_primary(&mut self) -> Expr {
        match self.tokens {
            [(Token::Num(num), _), ..] => {
                self.advance(1);
                Expr::Num(*num)
            }
            [(_, pos), ..] => {
                panic!("parse error at {:?}", pos);
            }
            [] => panic!("tokens are empty."),
        }
    }
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

    let tokens = lex::tokenize(&input);
    let tokens = &tokens[..];

    let mut parser_state = ParserState { tokens };

    let expr = parser_state.munch_expr();
    if !parser_state.tokens.is_empty() {
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
