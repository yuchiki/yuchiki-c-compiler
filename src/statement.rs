use crate::expr::Expr;
pub enum Statement {
    Expr(Expr),
    Return(Expr),
}
