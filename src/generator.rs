use crate::{expr::Expr, statement::Statement};

pub fn gen(statements: Vec<Statement>) {
    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");

    gen_statements(statements);

    println!("  ret");
}

fn gen_statements(statements: Vec<Statement>) {
    for statement in statements {
        gen_statement(statement);
    }
}

fn gen_statement(statement: Statement) {
    match statement {
        Statement::Expr(expr) => {
            gen_expr(expr);
            println!("  pop rax");
        }
    }
}

fn gen_expr(expr: Expr) {
    match expr {
        Expr::Num(n) => {
            println!("  push {n}");
        }
        Expr::Add(lhs, rhs) => {
            gen_expr(*lhs);
            gen_expr(*rhs);

            println!("  pop rdi");
            println!("  pop rax");

            println!("  add rax, rdi");
            println!("  push rax");
        }

        Expr::Sub(lhs, rhs) => {
            gen_expr(*lhs);
            gen_expr(*rhs);

            println!("  pop rdi");
            println!("  pop rax");

            println!("  sub rax, rdi");
            println!("  push rax");
        }
        Expr::Mul(lhs, rhs) => {
            gen_expr(*lhs);
            gen_expr(*rhs);

            println!("  pop rdi");
            println!("  pop rax");

            println!("  imul rax, rdi");

            println!("  push rax");
        }

        Expr::Div(lhs, rhs) => {
            gen_expr(*lhs);
            gen_expr(*rhs);

            println!("  pop rdi");
            println!("  pop rax");

            println!("  cqo");
            println!("  idiv rdi");

            println!("  push rax");
        }
        Expr::LessThan(lhs, rhs) => {
            gen_expr(*lhs);
            gen_expr(*rhs);

            println!("  pop rdi");
            println!("  pop rax");

            println!("  cmp rax, rdi");
            println!("  setl al");
            println!("  movzb rax, al");
            println!("  push rax");
        }
        Expr::LessEqual(lhs, rhs) => {
            gen_expr(*lhs);
            gen_expr(*rhs);

            println!("  pop rdi");
            println!("  pop rax");

            println!("  cmp rax, rdi");
            println!("  setle al");
            println!("  movzb rax, al");
            println!("  push rax");
        }
        Expr::Equal(lhs, rhs) => {
            gen_expr(*lhs);
            gen_expr(*rhs);

            println!("  pop rdi");
            println!("  pop rax");

            println!("  cmp rax, rdi");
            println!("  sete al");
            println!("  movzb rax, al");
            println!("  push rax");
        }
        Expr::NotEqual(lhs, rhs) => {
            gen_expr(*lhs);
            gen_expr(*rhs);

            println!("  pop rdi");
            println!("  pop rax");

            println!("  cmp rax, rdi");
            println!("  setne al");
            println!("  movzb rax, al");
            println!("  push rax");
        }
        Expr::GreaterThan(lhs, rhs) => {
            gen_expr(*lhs);
            gen_expr(*rhs);

            println!("  pop rdi");
            println!("  pop rax");

            println!("  cmp rax, rdi");
            println!("  setg al");
            println!("  movzb rax, al");
            println!("  push rax");
        }
        Expr::GreaterEqual(lhs, rhs) => {
            gen_expr(*lhs);
            gen_expr(*rhs);

            println!("  pop rdi");
            println!("  pop rax");

            println!("  cmp rax, rdi");
            println!("  setge al");
            println!("  movzb rax, al");
            println!("  push rax");
        }
        Expr::Assign(_, _) => todo!(),
    }
}
