use std::collections::{hash_map, HashMap};

use crate::statement::Statement;

pub fn calculate_offset(params: &[String], statements: &[Statement]) -> HashMap<String, usize> {
    let identifiers = [params, &collect_identifiers_in_statements(statements)[..]].concat();

    let mut offset_map = HashMap::new();
    let mut offset = 8;
    for variable in identifiers {
        if let hash_map::Entry::Vacant(e) = offset_map.entry(variable) {
            e.insert(offset);
            offset += 8;
        }
    }
    offset_map
}

fn collect_identifiers_in_statements(statements: &[Statement]) -> Vec<String> {
    let mut identifiers = vec![];
    for statement in statements {
        identifiers.append(&mut collect_identifiers_in_statement(statement));
    }
    identifiers
}

fn collect_identifiers_in_statement(statement: &Statement) -> Vec<String> {
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
        Statement::VariableDeclaration(name) => vec![name.clone()],
    }
}

#[cfg(test)]
mod tests {
    use crate::expr::Expr;

    use super::*;

    #[test]
    fn test_calculate_offset() {
        let params = vec!["a".to_string(), "b".to_string()];
        let statements = vec![
            Statement::Expr(Expr::Assign(
                Box::new(Expr::Variable("a".to_string())),
                Box::new(Expr::Num(1)),
            )),
            Statement::Expr(Expr::Assign(
                Box::new(Expr::Variable("b".to_string())),
                Box::new(Expr::Num(2)),
            )),
            Statement::VariableDeclaration("c".to_string()),
        ];
        let offset_map = calculate_offset(&params, &statements);
        assert_eq!(offset_map["a"], 8);
        assert_eq!(offset_map["b"], 16);
        assert_eq!(offset_map["c"], 24);
    }

    #[test]
    fn test_collect_identifiers_in_statements() {
        let statements = vec![
            Statement::Expr(Expr::Assign(
                Box::new(Expr::Variable("a".to_string())),
                Box::new(Expr::Num(1)),
            )),
            Statement::VariableDeclaration("b".to_string()),
        ];
        let identifiers = collect_identifiers_in_statements(&statements);
        assert_eq!(identifiers, vec!["b",]);
    }
}
