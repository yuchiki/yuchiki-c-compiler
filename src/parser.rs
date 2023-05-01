use crate::{
    expr::Expr,
    lex::{PositionedToken, SourcePosition, Token},
};

pub struct ParserState<'a> {
    tokens: &'a [PositionedToken],
    raw_input: &'a str,
}

impl<'a> ParserState<'a> {
    pub fn new(tokens: &'a [PositionedToken], raw_input: &'a str) -> Self {
        Self { tokens, raw_input }
    }

    pub fn fully_parsed(&self) -> bool {
        self.tokens.is_empty()
    }

    fn advance(&mut self, offset: usize) {
        self.tokens = &self.tokens[offset..]
    }

    pub fn munch_expr(&mut self) -> Expr {
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
                [(Token::Slash, _), ..] => {
                    self.advance(1);
                    let rhs = self.munch_primary();
                    expr = Expr::Div(Box::new(expr), Box::new(rhs));
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
            [(Token::LParen, _), ..] => {
                self.advance(1);
                let expr = self.munch_expr();
                match self.tokens {
                    [(Token::RParen, _), ..] => {
                        self.advance(1);
                        expr
                    }
                    [(token, pos), ..] => {
                        self.error(&format!("expected ')', but got {:?}", token), *pos)
                    }
                    _ => panic!("かっこが閉じられていない"),
                }
            }
            [(token, pos), ..] => {
                self.error(&format!("expected primary but got {:?}", token), *pos);
            }
            [] => panic!("tokens are empty."),
        }
    }

    fn error(&self, error_message: &str, pos: SourcePosition) -> ! {
        eprintln!(
            "{input}\n{:width$}^{error_message}",
            "",
            width = pos.0,
            input = self.raw_input
        );
        panic!("compile error")
    }
}
