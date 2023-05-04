use crate::statement::Statement;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TopLevel {
    FunctionDefinition(String, Vec<String>, Vec<Statement>),
}
