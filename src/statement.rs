use crate::expr::Expr;
#[derive(Debug)]
pub enum Statement {
    Expr(Expr),
    Return(Expr),
    If(Box<Expr>, Box<Statement>),
    IfElse(Box<Expr>, Box<Statement>, Box<Statement>),
    While(Box<Expr>, Box<Statement>),
}
