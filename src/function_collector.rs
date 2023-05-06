use std::collections::HashMap;

use crate::{top_level::TopLevel, types::FunctionType};

pub fn collect_functions(program: &Vec<TopLevel>) -> HashMap<String, FunctionType> {
    let mut functions = HashMap::new();
    for top_level in program {
        match top_level {
            TopLevel::FunctionDefinition(name, args, return_type, _)
            | TopLevel::ExternalFunctionDeclaration(name, args, return_type) => {
                let mut arg_types = Vec::new();
                for (_, arg_type) in args {
                    arg_types.push(arg_type.clone());
                }
                functions.insert(name.clone(), (arg_types, Box::new(return_type.clone())));
            }
        }
    }
    functions
}
