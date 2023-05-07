#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Expr {
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Num(i32),
    LessThan(Box<Expr>, Box<Expr>),
    LessEqual(Box<Expr>, Box<Expr>),
    Equal(Box<Expr>, Box<Expr>),
    NotEqual(Box<Expr>, Box<Expr>),
    GreaterThan(Box<Expr>, Box<Expr>),
    GreaterEqual(Box<Expr>, Box<Expr>),
    Assign(Box<Expr>, Box<Expr>),
    Variable(String),
    FunctionCall(String, Vec<Expr>),
    Address(Box<Expr>),
    Dereference(Box<Expr>),
    Sizeof(Box<Expr>),
}

use crate::types::Type;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TypedExpr {
    Add(Type, Box<TypedExpr>, Box<TypedExpr>),
    Sub(Type, Box<TypedExpr>, Box<TypedExpr>),
    Mul(Type, Box<TypedExpr>, Box<TypedExpr>),
    Div(Type, Box<TypedExpr>, Box<TypedExpr>),
    IntNum(i32),
    LessThan(Box<TypedExpr>, Box<TypedExpr>),
    LessEqual(Box<TypedExpr>, Box<TypedExpr>),
    Equal(Box<TypedExpr>, Box<TypedExpr>),
    NotEqual(Box<TypedExpr>, Box<TypedExpr>),
    GreaterThan(Box<TypedExpr>, Box<TypedExpr>),
    GreaterEqual(Box<TypedExpr>, Box<TypedExpr>),
    Assign(Type, Box<TypedExpr>, Box<TypedExpr>),
    Variable(Type, String),
    FunctionCall(Type, String, Vec<TypedExpr>),
    Address(Type, Box<TypedExpr>),
    Dereference(Type, Box<TypedExpr>),
    Sizeof(Box<TypedExpr>),
}

impl TypedExpr {
    pub fn get_type(&self) -> Type {
        match self {
            Self::Add(t, _, _)
            | Self::Sub(t, _, _)
            | Self::Mul(t, _, _)
            | Self::Div(t, _, _)
            | Self::Assign(t, _, _)
            | Self::Variable(t, _)
            | Self::FunctionCall(t, _, _)
            | Self::Address(t, _)
            | Self::Dereference(t, _) => t.clone(),
            Self::IntNum(_)
            | Self::LessThan(_, _)
            | Self::LessEqual(_, _)
            | Self::Equal(_, _)
            | Self::NotEqual(_, _)
            | Self::GreaterThan(_, _)
            | Self::GreaterEqual(_, _)
            | Self::Sizeof(_) => Type::IntTyp,
        }
    }
}
