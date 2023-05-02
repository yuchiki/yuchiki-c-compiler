use std::collections::HashMap;

use crate::{expr::Expr, statement::Statement};

pub struct Generator {
    variable_offsets: HashMap<String, usize>,
    fresh_counter: usize,
}

impl Generator {
    pub fn new(variable_offsets: HashMap<String, usize>) -> Self {
        Self {
            variable_offsets,
            fresh_counter: 0,
        }
    }

    fn get_fresh_suffix(&mut self) -> String {
        self.fresh_counter += 1;
        format!("{}", self.fresh_counter)
    }

    pub fn gen(&mut self, statements: Vec<Statement>) {
        println!(".intel_syntax noprefix");
        println!(".globl main");
        println!("main:");

        println!("  push rbp");
        println!("  mov rbp, rsp");
        println!("  sub rsp, {}", self.variable_offsets.len() * 8);

        self.gen_statements(statements);

        println!("  mov rsp, rbp");
        println!("  pop rbp");
        println!("  ret");
    }

    fn gen_statements(&mut self, statements: Vec<Statement>) {
        for statement in statements {
            self.gen_statement(statement);
        }
    }

    fn gen_statement(&mut self, statement: Statement) {
        match statement {
            Statement::Expr(expr) => {
                self.gen_expr(expr);
                println!("  pop rax");
            }
            Statement::Return(expr) => {
                self.gen_expr(expr);
                println!("  pop rax");
                println!("  mov rsp, rbp");
                println!("  pop rbp");
                println!("  ret");
            }
            Statement::If(expr, then_statement) => {
                let suffix = self.get_fresh_suffix();

                self.gen_expr(*expr);
                println!("  pop rax");
                println!("  cmp rax, 0");
                println!("  je .Lend{}", suffix);

                self.gen_statement(*then_statement);

                println!(".Lend{}:", suffix);
            }
            Statement::IfElse(expr, then_statement, else_statement) => {
                let suffix = self.get_fresh_suffix();

                self.gen_expr(*expr);
                println!("  pop rax");
                println!("  cmp rax, 0");
                println!("  je .Lelse{}", suffix);

                self.gen_statement(*then_statement);

                println!("  jmp .Lend{}", suffix);
                println!(".Lelse{}:", suffix);

                self.gen_statement(*else_statement);

                println!(".Lend{}:", suffix);
            }
            Statement::While(expr, statement) => {
                let suffix = self.get_fresh_suffix();

                println!(".Lbegin{}:", suffix);

                self.gen_expr(*expr);
                println!("  pop rax");
                println!("  cmp rax, 0");
                println!("  je .Lend{}", suffix);

                self.gen_statement(*statement);

                println!("  jmp .Lbegin{}", suffix);
                println!(".Lend{}:", suffix);
            }
            Statement::For(init, cond, update, body) => {
                let suffix = self.get_fresh_suffix();

                self.gen_expr(*init);

                println!(".Lbegin{}:", suffix);

                self.gen_expr(*cond);
                println!("  pop rax");
                println!("  cmp rax, 0");
                println!("  je .Lend{}", suffix);

                self.gen_statement(*body);

                self.gen_expr(*update);

                println!("  jmp .Lbegin{}", suffix);
                println!(".Lend{}:", suffix);
            }
            Statement::Block(statements) => {
                self.gen_statements(statements);
            }
        }
    }

    fn gen_expr(&self, expr: Expr) {
        match expr {
            Expr::Num(n) => {
                println!("  push {n}");
            }
            Expr::Add(lhs, rhs) => {
                self.gen_expr(*lhs);
                self.gen_expr(*rhs);

                println!("  pop rdi");
                println!("  pop rax");

                println!("  add rax, rdi");
                println!("  push rax");
            }

            Expr::Sub(lhs, rhs) => {
                self.gen_expr(*lhs);
                self.gen_expr(*rhs);

                println!("  pop rdi");
                println!("  pop rax");

                println!("  sub rax, rdi");
                println!("  push rax");
            }
            Expr::Mul(lhs, rhs) => {
                self.gen_expr(*lhs);
                self.gen_expr(*rhs);

                println!("  pop rdi");
                println!("  pop rax");

                println!("  imul rax, rdi");

                println!("  push rax");
            }

            Expr::Div(lhs, rhs) => {
                self.gen_expr(*lhs);
                self.gen_expr(*rhs);

                println!("  pop rdi");
                println!("  pop rax");

                println!("  cqo");
                println!("  idiv rdi");

                println!("  push rax");
            }
            Expr::LessThan(lhs, rhs) => {
                self.gen_expr(*lhs);
                self.gen_expr(*rhs);

                println!("  pop rdi");
                println!("  pop rax");

                println!("  cmp rax, rdi");
                println!("  setl al");
                println!("  movzb rax, al");
                println!("  push rax");
            }
            Expr::LessEqual(lhs, rhs) => {
                self.gen_expr(*lhs);
                self.gen_expr(*rhs);

                println!("  pop rdi");
                println!("  pop rax");

                println!("  cmp rax, rdi");
                println!("  setle al");
                println!("  movzb rax, al");
                println!("  push rax");
            }
            Expr::Equal(lhs, rhs) => {
                self.gen_expr(*lhs);
                self.gen_expr(*rhs);

                println!("  pop rdi");
                println!("  pop rax");

                println!("  cmp rax, rdi");
                println!("  sete al");
                println!("  movzb rax, al");
                println!("  push rax");
            }
            Expr::NotEqual(lhs, rhs) => {
                self.gen_expr(*lhs);
                self.gen_expr(*rhs);

                println!("  pop rdi");
                println!("  pop rax");

                println!("  cmp rax, rdi");
                println!("  setne al");
                println!("  movzb rax, al");
                println!("  push rax");
            }
            Expr::GreaterThan(lhs, rhs) => {
                self.gen_expr(*lhs);
                self.gen_expr(*rhs);

                println!("  pop rdi");
                println!("  pop rax");

                println!("  cmp rax, rdi");
                println!("  setg al");
                println!("  movzb rax, al");
                println!("  push rax");
            }
            Expr::GreaterEqual(lhs, rhs) => {
                self.gen_expr(*lhs);
                self.gen_expr(*rhs);

                println!("  pop rdi");
                println!("  pop rax");

                println!("  cmp rax, rdi");
                println!("  setge al");
                println!("  movzb rax, al");
                println!("  push rax");
            }
            Expr::Assign(lhs, rhs) => {
                self.gen_lvalue(*lhs);
                self.gen_expr(*rhs);

                println!("  pop rdi");
                println!("  pop rax");
                println!("  mov [rax], rdi");
                println!("  push rdi");
            }
            Expr::Variable(_) => {
                self.gen_lvalue(expr);
                println!("  pop rax");
                println!("  mov rax, [rax]");
                println!("  push rax");
            }
        }
    }

    fn gen_lvalue(&self, expr: Expr) {
        match expr {
            Expr::Variable(name) => {
                let offset = self
                    .variable_offsets
                    .get(&name)
                    .expect("variable not found");

                println!("  mov rax, rbp");
                println!("  sub rax, {}", offset);
                println!("  push rax");
            }
            _ => todo!(),
        }
    }
}
