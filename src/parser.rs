use crate::{
    expr::Expr,
    lex::{PositionedToken, SourcePosition},
    statement::Statement,
    token::Token,
    top_level::TopLevel,
};

pub struct Parser<'a> {
    tokens: &'a [PositionedToken],
    raw_input: &'a str,
}

impl<'a> Parser<'a> {
    pub const fn new(tokens: &'a [PositionedToken], raw_input: &'a str) -> Self {
        Self { tokens, raw_input }
    }

    pub const fn fully_parsed(&self) -> bool {
        self.tokens.is_empty()
    }

    fn advance(&mut self, offset: usize) {
        self.tokens = &self.tokens[offset..];
    }

    pub fn munch_program(&mut self) -> Vec<TopLevel> {
        let mut ans = vec![];
        while !self.fully_parsed() {
            ans.push(self.munch_top_level());
        }
        ans
    }

    pub fn munch_top_level(&mut self) -> TopLevel {
        match self.tokens {
            [(Token::Identifier(name), _), (Token::LParen, _), ..] => {
                self.advance(2);
                let mut args = vec![];
                while self.tokens[0].0 != Token::RParen {
                    if let Token::Identifier(arg) = &self.tokens[0].0 {
                        self.advance(1);
                        args.push(arg.clone());

                        if self.tokens[0].0 == Token::RParen {
                            break;
                        } else if self.tokens[0].0 == Token::Comma {
                            self.advance(1);
                        } else {
                            panic!("parse error at {:#?}", self.tokens)
                        }
                    } else {
                        panic!("parse error at {:#?}", self.tokens)
                    }
                }

                self.advance(1);

                assert!(!(self.tokens[0].0 != Token::LBrace), "parse error");

                self.advance(1);
                let mut statements = vec![];
                while self.tokens[0].0 != Token::RBrace {
                    statements.push(self.munch_statement());
                }
                self.advance(1);
                TopLevel::FunctionDefinition(name.clone(), args, statements)
            }
            _ => panic!("parse error"),
        }
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
            [(Token::While, _), (Token::LParen, _), ..] => {
                self.advance(2);
                let cond = self.munch_expr();
                match self.tokens {
                    [(Token::RParen, _), ..] => {
                        self.advance(1);
                        let body = self.munch_statement();
                        Statement::While(Box::new(cond), Box::new(body))
                    }
                    _ => panic!("括弧が閉じられていない"),
                }
            }
            [(Token::For, _), (Token::LParen, _), ..] => {
                self.advance(2);
                let init = self.munch_expr();
                if let [(Token::Semicolon, _), ..] = self.tokens {
                    self.advance(1);
                } else {
                    panic!("セミコロンがない");
                }

                let cond = self.munch_expr();
                if let [(Token::Semicolon, _), ..] = self.tokens {
                    self.advance(1);
                } else {
                    panic!("セミコロンがない");
                }

                let update = self.munch_expr();
                if let [(Token::RParen, _), ..] = self.tokens {
                    self.advance(1);
                } else {
                    panic!("括弧が閉じられていない");
                }

                let body = self.munch_statement();

                Statement::For(
                    Box::new(init),
                    Box::new(cond),
                    Box::new(update),
                    Box::new(body),
                )
            }
            [(Token::LBrace, _), ..] => {
                self.advance(1);
                let mut statements = Vec::new();
                loop {
                    match self.tokens {
                        [(Token::RBrace, _), ..] => {
                            self.advance(1);
                            break;
                        }
                        _ => statements.push(self.munch_statement()),
                    }
                }

                Statement::Block(statements)
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
            [(Token::Ampersand, _), ..] => {
                self.advance(1);
                Expr::Address(Box::new(self.munch_primary()))
            }
            [(Token::Asterisk, _), ..] => {
                self.advance(1);
                Expr::Dereference(Box::new(self.munch_primary()))
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
            [(Token::Identifier(name), _), (Token::LParen, _), ..] => {
                self.advance(2);
                let mut args = Vec::new();
                loop {
                    if let [(Token::RParen, _), ..] = self.tokens {
                        self.advance(1);
                        break;
                    }

                    args.push(self.munch_expr());
                    match self.tokens {
                        [(Token::Comma, _), ..] => {
                            self.advance(1);
                        }
                        [(Token::RParen, _), ..] => {
                            self.advance(1);
                            break;
                        }
                        _ => panic!("引数の区切りが不正"),
                    }
                }

                Expr::FunctionCall(name.clone(), args)
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
                        self.error(&format!("expected ')', but got {token:?}"), *pos)
                    }
                    _ => panic!("かっこが閉じられていない"),
                }
            }
            [(token, pos), ..] => {
                self.error(&format!("expected primary but got {token:?}"), *pos);
            }
            [] => panic!("tokens are empty."),
        }
    }

    fn error(&self, error_message: &str, pos: SourcePosition) -> ! {
        eprint!(
            "{input}\n{:width$}^{error_message}",
            "",
            width = pos.0,
            input = self.raw_input
        );
        panic!("compile error")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_munch_expr() {
        let tokens = vec![
            (Token::Identifier("b".to_string()), SourcePosition(45)),
            (Token::Assign, SourcePosition(47)),
            (Token::Plus, SourcePosition(0)),
            (Token::Num(1), SourcePosition(0)),
            (Token::Plus, SourcePosition(2)),
            (Token::Num(2), SourcePosition(4)),
            (Token::Asterisk, SourcePosition(6)),
            (Token::Num(3), SourcePosition(8)),
            (Token::Slash, SourcePosition(10)),
            (Token::LParen, SourcePosition(12)),
            (Token::Minus, SourcePosition(13)),
            (Token::Num(4), SourcePosition(13)),
            (Token::Minus, SourcePosition(15)),
            (Token::Num(5), SourcePosition(16)),
            (Token::RParen, SourcePosition(17)),
            (Token::LessThan, SourcePosition(19)),
            (Token::Identifier("a".to_string()), SourcePosition(21)),
            (Token::GreaterThan, SourcePosition(23)),
            (Token::Num(7), SourcePosition(25)),
            (Token::GreaterThanOrEqual, SourcePosition(27)),
            (Token::Ampersand, SourcePosition(0)),
            (Token::Identifier("a".to_string()), SourcePosition(0)),
            (Token::LessThanOrEqual, SourcePosition(32)),
            (Token::Num(9), SourcePosition(35)),
            (Token::Equality, SourcePosition(37)),
            (Token::Num(10), SourcePosition(40)),
            (Token::Inequality, SourcePosition(42)),
            (Token::Asterisk, SourcePosition(45)),
            (Token::Num(11), SourcePosition(45)),
        ];

        let mut parser = Parser::new(
            &tokens,
            "b = +1 + 2 * 3 / (4-5) < a > 7 >= &a <= 9 == 10 != 11;",
        );
        let expr = parser.munch_expr();
        assert_eq!(
            expr,
            Expr::Assign(
                Box::new(Expr::Variable("b".to_string())),
                Box::new(Expr::NotEqual(
                    Box::new(Expr::Equal(
                        Box::new(Expr::LessEqual(
                            Box::new(Expr::GreaterEqual(
                                Box::new(Expr::GreaterThan(
                                    Box::new(Expr::LessThan(
                                        Box::new(Expr::Add(
                                            Box::new(Expr::Num(1)),
                                            Box::new(Expr::Div(
                                                Box::new(Expr::Mul(
                                                    Box::new(Expr::Num(2)),
                                                    Box::new(Expr::Num(3))
                                                )),
                                                Box::new(Expr::Sub(
                                                    Box::new(Expr::Sub(
                                                        Box::new(Expr::Num(0)),
                                                        Box::new(Expr::Num(4))
                                                    )),
                                                    Box::new(Expr::Num(5))
                                                ))
                                            ))
                                        )),
                                        Box::new(Expr::Variable("a".to_string()))
                                    )),
                                    Box::new(Expr::Num(7))
                                )),
                                Box::new(Expr::Address(Box::new(Expr::Variable("a".to_string()))))
                            )),
                            Box::new(Expr::Num(9)),
                        )),
                        Box::new(Expr::Num(10)),
                    )),
                    Box::new(Expr::Dereference(Box::new(Expr::Num(11))))
                ))
            )
        );
    }

    #[test]
    fn test_munch_statement_with_expr() {
        let tokens = vec![
            (Token::Identifier("a".to_string()), SourcePosition(0)),
            (Token::Assign, SourcePosition(2)),
            (Token::Num(1), SourcePosition(4)),
            (Token::Semicolon, SourcePosition(5)),
        ];

        let mut parser = Parser::new(&tokens, "a = 1;");
        let statement = parser.munch_statement();

        assert_eq!(
            statement,
            Statement::Expr(Expr::Assign(
                Box::new(Expr::Variable("a".to_string())),
                Box::new(Expr::Num(1))
            ))
        );
    }

    #[test]
    fn test_munch_statement_with_if() {
        let tokens = vec![
            (Token::If, SourcePosition(0)),
            (Token::LParen, SourcePosition(2)),
            (Token::Num(1), SourcePosition(3)),
            (Token::RParen, SourcePosition(4)),
            (Token::LBrace, SourcePosition(6)),
            (Token::Num(1), SourcePosition(0)),
            (Token::Semicolon, SourcePosition(1)),
            (Token::Num(2), SourcePosition(2)),
            (Token::Semicolon, SourcePosition(3)),
            (Token::RBrace, SourcePosition(7)),
        ];

        let mut parser = Parser::new(&tokens, "if (1) {1; 2;}");
        let statement = parser.munch_statement();

        assert_eq!(
            statement,
            Statement::If(
                Box::new(Expr::Num(1)),
                Box::new(Statement::Block(vec![
                    Statement::Expr(Expr::Num(1)),
                    Statement::Expr(Expr::Num(2))
                ]))
            )
        );
    }

    #[test]
    fn test_munch_statement_with_for() {
        let tokens = vec![
            (Token::For, SourcePosition(0)),
            (Token::LParen, SourcePosition(3)),
            (Token::Identifier("i".to_string()), SourcePosition(4)),
            (Token::Assign, SourcePosition(6)),
            (Token::Num(0), SourcePosition(8)),
            (Token::Semicolon, SourcePosition(9)),
            (Token::Identifier("i".to_string()), SourcePosition(11)),
            (Token::LessThan, SourcePosition(13)),
            (Token::Num(10), SourcePosition(15)),
            (Token::Semicolon, SourcePosition(17)),
            (Token::Identifier("i".to_string()), SourcePosition(19)),
            (Token::Assign, SourcePosition(21)),
            (Token::Identifier("i".to_string()), SourcePosition(23)),
            (Token::Plus, SourcePosition(25)),
            (Token::Num(1), SourcePosition(27)),
            (Token::RParen, SourcePosition(28)),
            (Token::LBrace, SourcePosition(30)),
            (Token::Num(1), SourcePosition(0)),
            (Token::Semicolon, SourcePosition(1)),
            (Token::Num(2), SourcePosition(2)),
            (Token::Semicolon, SourcePosition(3)),
            (Token::RBrace, SourcePosition(31)),
        ];

        let mut parser = Parser::new(&tokens, "for (i = 0; i < 10; i = i + 1) {1; 2;}");
        let statement = parser.munch_statement();

        assert_eq!(
            statement,
            Statement::For(
                Box::new(Expr::Assign(
                    Box::new(Expr::Variable("i".to_string())),
                    Box::new(Expr::Num(0))
                )),
                Box::new(Expr::LessThan(
                    Box::new(Expr::Variable("i".to_string())),
                    Box::new(Expr::Num(10))
                )),
                Box::new(Expr::Assign(
                    Box::new(Expr::Variable("i".to_string())),
                    Box::new(Expr::Add(
                        Box::new(Expr::Variable("i".to_string())),
                        Box::new(Expr::Num(1))
                    ))
                )),
                Box::new(Statement::Block(vec![
                    Statement::Expr(Expr::Num(1)),
                    Statement::Expr(Expr::Num(2))
                ]))
            )
        );
    }

    #[test]
    fn test_munch_statement_with_while() {
        let tokens = vec![
            (Token::While, SourcePosition(0)),
            (Token::LParen, SourcePosition(5)),
            (Token::Num(1), SourcePosition(6)),
            (Token::RParen, SourcePosition(7)),
            (Token::LBrace, SourcePosition(9)),
            (Token::Num(1), SourcePosition(0)),
            (Token::Semicolon, SourcePosition(1)),
            (Token::Num(2), SourcePosition(2)),
            (Token::Semicolon, SourcePosition(3)),
            (Token::RBrace, SourcePosition(10)),
        ];

        let mut parser = Parser::new(&tokens, "while (1){1;2;}");
        let statement = parser.munch_statement();

        assert_eq!(
            statement,
            Statement::While(
                Box::new(Expr::Num(1)),
                Box::new(Statement::Block(vec![
                    Statement::Expr(Expr::Num(1)),
                    Statement::Expr(Expr::Num(2))
                ]))
            )
        );
    }

    #[test]
    fn test_munch_statement_with_block() {
        let tokens = vec![
            (Token::LBrace, SourcePosition(0)),
            (Token::Num(1), SourcePosition(0)),
            (Token::Semicolon, SourcePosition(0)),
            (Token::Num(2), SourcePosition(0)),
            (Token::Semicolon, SourcePosition(0)),
            (Token::RBrace, SourcePosition(0)),
        ];

        let mut parser = Parser::new(&tokens, "{}");
        let statement = parser.munch_statement();

        assert_eq!(
            statement,
            Statement::Block(vec![
                Statement::Expr(Expr::Num(1)),
                Statement::Expr(Expr::Num(2))
            ])
        );
    }

    #[test]
    fn test_munch_top_level() {
        let tokens = vec![
            (Token::Identifier("f".to_string()), SourcePosition(0)),
            (Token::LParen, SourcePosition(0)),
            (Token::Identifier("a".to_string()), SourcePosition(0)),
            (Token::Comma, SourcePosition(0)),
            (Token::Identifier("b".to_string()), SourcePosition(0)),
            (Token::RParen, SourcePosition(0)),
            (Token::LBrace, SourcePosition(0)),
            (Token::Num(1), SourcePosition(0)),
            (Token::Semicolon, SourcePosition(0)),
            (Token::Num(2), SourcePosition(0)),
            (Token::Semicolon, SourcePosition(0)),
            (Token::RBrace, SourcePosition(0)),
        ];

        let mut parser = Parser::new(&tokens, "f(a, b) {}");
        let top_level = parser.munch_top_level();

        assert_eq!(
            top_level,
            TopLevel::FunctionDefinition(
                "f".to_string(),
                vec!["a".to_string(), "b".to_string()],
                vec![Statement::Expr(Expr::Num(1)), Statement::Expr(Expr::Num(2))]
            )
        );
    }
}
