use crate::{
    expr::{Expr, TypedExpr},
    types::Type,
};
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

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TypedStatement {
    Expr(TypedExpr),
    Return(TypedExpr),
    If(Box<TypedExpr>, Box<TypedStatement>),
    IfElse(Box<TypedExpr>, Box<TypedStatement>, Box<TypedStatement>),
    While(Box<TypedExpr>, Box<TypedStatement>),
    For(
        Box<TypedExpr>,
        Box<TypedExpr>,
        Box<TypedExpr>,
        Box<TypedStatement>,
    ),
    Block(Vec<TypedStatement>),
    VariableDeclaration(String, Type),
}
