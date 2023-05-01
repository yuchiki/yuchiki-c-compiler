use std::collections::{hash_map, HashMap};

use crate::{expr::Expr, statement::Statement};

pub fn calculate_offset(statements: &Vec<Statement>) -> HashMap<String, usize> {
    let identifiers = collect_identifiers_in_statements(statements);

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

fn collect_identifiers_in_statements(statements: &Vec<Statement>) -> Vec<String> {
    let mut identifiers = vec![];
    for statement in statements {
        identifiers.append(&mut collect_identifiers_in_statement(statement));
    }
    identifiers
}

fn collect_identifiers_in_statement(statement: &Statement) -> Vec<String> {
    match statement {
        Statement::Expr(expr) | Statement::Return(expr) => collect_identifiers_in_expr(expr),
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
    }
}
