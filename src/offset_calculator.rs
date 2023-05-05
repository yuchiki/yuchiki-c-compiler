use std::collections::{hash_map, HashMap};

use crate::{expr::Expr, statement::Statement};

pub fn calculate_offset(params: &[String], statements: &[Statement]) -> HashMap<String, usize> {
    let identifiers = [&collect_identifiers_in_statements(statements)[..], params].concat();

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
        Statement::Expr(expr) | Statement::Return(expr) => collect_identifiers_in_expr(expr),
        Statement::If(cond, then) => [
            &collect_identifiers_in_expr(cond)[..],
            &collect_identifiers_in_statement(then)[..],
        ]
        .concat(),
        Statement::IfElse(cond, then, els) => [
            &collect_identifiers_in_expr(cond)[..],
            &collect_identifiers_in_statement(then)[..],
            &collect_identifiers_in_statement(els)[..],
        ]
        .concat(),
        Statement::While(cond, body) => [
            &collect_identifiers_in_expr(cond)[..],
            &collect_identifiers_in_statement(body)[..],
        ]
        .concat(),
        Statement::For(init, cond, step, body) => [
            &collect_identifiers_in_expr(init)[..],
            &collect_identifiers_in_expr(cond)[..],
            &collect_identifiers_in_expr(step)[..],
            &collect_identifiers_in_statement(body)[..],
        ]
        .concat(),
        Statement::Block(statements) => collect_identifiers_in_statements(statements),
    }
}

fn collect_identifiers_in_expr(expr: &Expr) -> Vec<String> {
    match expr {
        Expr::Add(lhs, rhs)
        | Expr::Sub(lhs, rhs)
        | Expr::Mul(lhs, rhs)
        | Expr::Div(lhs, rhs)
        | Expr::Equal(lhs, rhs)
        | Expr::NotEqual(lhs, rhs)
        | Expr::LessThan(lhs, rhs)
        | Expr::LessEqual(lhs, rhs)
        | Expr::GreaterThan(lhs, rhs)
        | Expr::GreaterEqual(lhs, rhs)
        | Expr::Assign(lhs, rhs) => [
            &collect_identifiers_in_expr(lhs)[..],
            &collect_identifiers_in_expr(rhs)[..],
        ]
        .concat(),
        Expr::Num(_) => vec![],
        Expr::Variable(name) => vec![name.clone()],
        Expr::FunctionCall(_, args) => args
            .iter()
            .flat_map(collect_identifiers_in_expr)
            .collect::<Vec<String>>(),
    }
}

#[cfg(test)]
mod tests {
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
            Statement::Expr(Expr::Assign(
                Box::new(Expr::Variable("c".to_string())),
                Box::new(Expr::Num(3)),
            )),
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
            Statement::Expr(Expr::Assign(
                Box::new(Expr::Variable("b".to_string())),
                Box::new(Expr::Num(2)),
            )),
            Statement::Expr(Expr::Assign(
                Box::new(Expr::Variable("c".to_string())),
                Box::new(Expr::Num(3)),
            )),
        ];
        let identifiers = collect_identifiers_in_statements(&statements);
        assert_eq!(identifiers, vec!["a", "b", "c"]);
    }

    #[test]
    fn test_collect_identifiers_in_statement() {
        let statement = Statement::Expr(Expr::Assign(
            Box::new(Expr::Variable("a".to_string())),
            Box::new(Expr::Num(1)),
        ));
        let identifiers = collect_identifiers_in_statement(&statement);
        assert_eq!(identifiers, vec!["a"]);
    }

    #[test]
    fn test_collect_identifiers_in_expr() {
        let expr = Expr::FunctionCall(
            "f".to_string(),
            vec![
                Expr::Variable("a".to_string()),
                Expr::Mul(
                    Box::new(Expr::Variable("b".to_string())),
                    Box::new(Expr::Num(2)),
                ),
                Expr::Assign(
                    Box::new(Expr::Assign(
                        Box::new(Expr::Variable("c".to_string())),
                        Box::new(Expr::Num(3)),
                    )),
                    Box::new(Expr::Variable("d".to_string())),
                ),
            ],
        );
        let identifiers = collect_identifiers_in_expr(&expr);
        assert_eq!(identifiers, vec!["a", "b", "c", "d"]);
    }
}
