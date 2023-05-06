use crate::{expr::Expr, types::Type};
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Statement {
    Expr(Expr),
    Return(Expr),
    If(Box<Expr>, Box<Statement>),
    IfElse(Box<Expr>, Box<Statement>, Box<Statement>),
    While(Box<Expr>, Box<Statement>),
    For(Box<Expr>, Box<Expr>, Box<Expr>, Box<Statement>),
    Block(Vec<Statement>),
    VariableDeclaration(String, Type),
}
