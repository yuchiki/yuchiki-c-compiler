use std::collections::HashMap;

use crate::{
    statement::{Statement, TypedStatement},
    types::Type,
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TopLevel {
    FunctionDefinition(String, Vec<(String, Type)>, Type, Vec<Statement>),
    ExternalFunctionDeclaration(String, Vec<(String, Type)>, Type),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TypedTopLevel {
    FunctionDefinition(
        String,
        Vec<(String, Type)>,
        Type,
        Vec<TypedStatement>,
        HashMap<String, Type>,
    ),
}
