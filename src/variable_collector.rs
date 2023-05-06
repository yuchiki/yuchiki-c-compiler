use std::collections::{hash_map, HashMap};

use crate::{statement::Statement, types::Type};

pub fn collect_variables(
    args: &[(String, Type)],
    statements: &[Statement],
) -> HashMap<String, Type> {
    let mut variables = args.to_vec();
    variables.append(&mut collect_variables_in_statements(statements));

    let mut variable_map = HashMap::new();
    for (variable, ty) in variables {
        if let hash_map::Entry::Vacant(e) = variable_map.entry(variable.clone()) {
            e.insert(ty);
        } else {
            panic!("Variable {variable} is already defined");
        }
    }
    variable_map
}

fn collect_variables_in_statements(statements: &[Statement]) -> Vec<(String, Type)> {
    let mut variable = vec![];
    for statement in statements {
        variable.append(&mut collect_variables_in_statement(statement));
    }
    variable
}

fn collect_variables_in_statement(statement: &Statement) -> Vec<(String, Type)> {
    match statement {
        Statement::Expr(_) | Statement::Return(_) => vec![],
        Statement::If(_, then) => [&collect_variables_in_statement(then)[..]].concat(),
        Statement::IfElse(_, then, els) => [
            &collect_variables_in_statement(then)[..],
            &collect_variables_in_statement(els)[..],
        ]
        .concat(),
        Statement::While(_, body) | Statement::For(_, _, _, body) => {
            [&collect_variables_in_statement(body)[..]].concat()
        }
        Statement::Block(statements) => collect_variables_in_statements(statements),
        Statement::VariableDeclaration(name, ty) => vec![(name.clone(), ty.clone())],
    }
}

#[cfg(test)]
mod tests {
    use crate::expr::Expr;

    use super::*;

    #[test]
    fn test_calculate_offset() {
        let params = vec![
            ("a".to_string(), Type::PointerType(Box::new(Type::IntType))),
            ("b".to_string(), Type::IntType),
        ];
        let statements = vec![
            Statement::Expr(Expr::Assign(
                Box::new(Expr::Variable("a".to_string())),
                Box::new(Expr::Num(1)),
            )),
            Statement::Expr(Expr::Assign(
                Box::new(Expr::Variable("b".to_string())),
                Box::new(Expr::Num(2)),
            )),
            Statement::VariableDeclaration("c".to_string(), Type::IntType),
        ];
        let offset_map = collect_variables(&params, &statements);
        assert_eq!(offset_map["a"], Type::PointerType(Box::new(Type::IntType)));
        assert_eq!(offset_map["b"], Type::IntType);
        assert_eq!(offset_map["c"], Type::IntType);
    }

    #[test]
    fn test_collect_identifiers_in_statements() {
        let statements = vec![
            Statement::Expr(Expr::Assign(
                Box::new(Expr::Variable("a".to_string())),
                Box::new(Expr::Num(1)),
            )),
            Statement::VariableDeclaration("b".to_string(), Type::IntType),
        ];
        let identifiers = collect_variables_in_statements(&statements);
        assert_eq!(identifiers, vec![("b".to_string(), Type::IntType)]);
    }
}
