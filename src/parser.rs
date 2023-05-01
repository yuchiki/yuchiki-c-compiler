use crate::{
    expr::Expr,
    lex::{PositionedToken, SourcePosition},
    statement::Statement,
    token::Token,
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

    pub fn munch_statement(&mut self) -> Statement {
        match self.tokens {
            [(Token::Return, _), ..] => {
                self.advance(1);
                let statment = Statement::Return(self.munch_expr());
                match self.tokens {
                    [(Token::Semicolon, _), ..] => {
                        self.advance(1);
                        statment
                    }
                    _ => panic!("セミコロンがない"),
                }
            }
            [(Token::If, _), (Token::LParen, _), ..] => {
                self.advance(2);
                let cond = self.munch_expr();
                match self.tokens {
                    [(Token::RParen, _), ..] => {
                        self.advance(1);
                        let then = self.munch_statement();
                        match self.tokens {
                            [(Token::Else, _), ..] => {
                                self.advance(1);
                                let els = self.munch_statement();
                                Statement::IfElse(Box::new(cond), Box::new(then), Box::new(els))
                            }
                            _ => Statement::If(Box::new(cond), Box::new(then)),
                        }
                    }
                    _ => panic!("括弧が閉じられていない"),
                }
            }
            _ => {
                let expr = self.munch_expr();
                match self.tokens {
                    [(Token::Semicolon, _), ..] => {
                        self.advance(1);
                        Statement::Expr(expr)
                    }
                    _ => panic!("セミコロンがない"),
                }
            }
        }
    }

    pub fn munch_expr(&mut self) -> Expr {
        self.munch_assign()
    }

    pub fn munch_assign(&mut self) -> Expr {
        let mut expr = self.munch_equality();

        loop {
            match self.tokens {
                [(Token::Assign, _), ..] => {
                    self.advance(1);
                    let rhs = self.munch_assign();
                    expr = Expr::Assign(Box::new(expr), Box::new(rhs));
                }
                _ => return expr,
            }
        }
    }

    pub fn munch_equality(&mut self) -> Expr {
        let mut expr = self.munch_relational();

        loop {
            match self.tokens {
                [(Token::Equality, _), ..] => {
                    self.advance(1);
                    let rhs = self.munch_relational();
                    expr = Expr::Equal(Box::new(expr), Box::new(rhs));
                }
                [(Token::Inequality, _), ..] => {
                    self.advance(1);
                    let rhs = self.munch_relational();
                    expr = Expr::NotEqual(Box::new(expr), Box::new(rhs));
                }
                _ => return expr,
            }
        }
    }

    pub fn munch_relational(&mut self) -> Expr {
        let mut expr = self.munch_add();

        loop {
            match self.tokens {
                [(Token::LessThan, _), ..] => {
                    self.advance(1);
                    let rhs = self.munch_add();
                    expr = Expr::LessThan(Box::new(expr), Box::new(rhs));
                }
                [(Token::LessThanOrEqual, _), ..] => {
                    self.advance(1);
                    let rhs = self.munch_add();
                    expr = Expr::LessEqual(Box::new(expr), Box::new(rhs));
                }
                [(Token::GreaterThan, _), ..] => {
                    self.advance(1);
                    let rhs = self.munch_add();
                    expr = Expr::GreaterThan(Box::new(expr), Box::new(rhs));
                }
                [(Token::GreaterThanOrEqual, _), ..] => {
                    self.advance(1);
                    let rhs = self.munch_add();
                    expr = Expr::GreaterEqual(Box::new(expr), Box::new(rhs));
                }
                _ => return expr,
            }
        }
    }

    pub fn munch_add(&mut self) -> Expr {
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
        let mut expr = self.munch_unary();

        loop {
            match self.tokens {
                [(Token::Asterisk, _), ..] => {
                    self.advance(1);
                    let rhs = self.munch_unary();
                    expr = Expr::Mul(Box::new(expr), Box::new(rhs));
                }
                [(Token::Slash, _), ..] => {
                    self.advance(1);
                    let rhs = self.munch_unary();
                    expr = Expr::Div(Box::new(expr), Box::new(rhs));
                }
                _ => return expr,
            }
        }
    }

    fn munch_unary(&mut self) -> Expr {
        match self.tokens {
            [(Token::Plus, _), ..] => {
                self.advance(1);
                self.munch_primary()
            }
            [(Token::Minus, _), ..] => {
                self.advance(1);
                Expr::Sub(Box::new(Expr::Num(0)), Box::new(self.munch_primary()))
            }
            _ => self.munch_primary(),
        }
    }

    fn munch_primary(&mut self) -> Expr {
        match self.tokens {
            [(Token::Num(num), _), ..] => {
                self.advance(1);
                Expr::Num(*num)
            }
            [(Token::Identifier(name), _), ..] => {
                self.advance(1);
                Expr::Variable(name.clone())
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
