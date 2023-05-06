use std::collections::{hash_map, HashMap};

use crate::{statement::Statement, types::Type};

pub fn calculate_offset(
    params: &[(String, Type)],
    statements: &[Statement],
) -> HashMap<String, (usize, Type)> {
    let identifiers = [params, &collect_identifiers_in_statements(statements)[..]].concat();

    let mut offset_map = HashMap::new();
    let mut offset = 8;
    for (variable, ty) in identifiers {
        if let hash_map::Entry::Vacant(e) = offset_map.entry(variable) {
            e.insert((offset, ty));
            offset += 8;
        }
    }
    offset_map
}

fn collect_identifiers_in_statements(statements: &[Statement]) -> Vec<(String, Type)> {
    let mut variable = vec![];
    for statement in statements {
        variable.append(&mut collect_identifiers_in_statement(statement));
    }
    variable
}

fn collect_identifiers_in_statement(statement: &Statement) -> Vec<(String, Type)> {
    match statement {
        Statement::Expr(_) | Statement::Return(_) => vec![],
        Statement::If(_, then) => [&collect_identifiers_in_statement(then)[..]].concat(),
        Statement::IfElse(_, then, els) => [
            &collect_identifiers_in_statement(then)[..],
            &collect_identifiers_in_statement(els)[..],
        ]
        .concat(),
        Statement::While(_, body) | Statement::For(_, _, _, body) => {
            [&collect_identifiers_in_statement(body)[..]].concat()
        }
        Statement::Block(statements) => collect_identifiers_in_statements(statements),
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
        let offset_map = calculate_offset(&params, &statements);
        assert_eq!(
            offset_map["a"],
            (8, Type::PointerType(Box::new(Type::IntType)))
        );
        assert_eq!(offset_map["b"], (16, Type::IntType));
        assert_eq!(offset_map["c"], (24, Type::IntType));
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
        let identifiers = collect_identifiers_in_statements(&statements);
        assert_eq!(identifiers, vec![("b".to_string(), Type::IntType)]);
    }
}
