use crate::{statement::Statement, types::Type};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TopLevel {
    FunctionDefinition(String, Vec<(String, Type)>, Type, Vec<Statement>),
}
