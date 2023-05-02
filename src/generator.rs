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

    pub fn gen(&mut self, statements: &Vec<Statement>) {
        println!(".intel_syntax noprefix");
        println!(".globl main");
        println!("main:");

        println!("  push rbp");
        println!("  mov rbp, rsp");
        println!("  sub rsp, {}", self.variable_offsets.len() * 8);

        self.gen_statements(statements, (1 + self.variable_offsets.len()) * 8);

        println!("  mov rsp, rbp");
        println!("  pop rbp");
        println!("  ret");
    }

    fn gen_statements(&mut self, statements: &Vec<Statement>, rsp_offset: usize) {
        for statement in statements {
            self.gen_statement(statement, rsp_offset);
        }
    }

    fn gen_statement(&mut self, statement: &Statement, rsp_offset: usize) {
        match statement {
            Statement::Expr(expr) => {
                self.gen_expr(expr, rsp_offset);
                println!("  pop rax");
            }
            Statement::Return(expr) => {
                self.gen_expr(expr, rsp_offset);
                println!("  pop rax");
                println!("  mov rsp, rbp");
                println!("  pop rbp");
                println!("  ret");
            }
            Statement::If(expr, then_statement) => {
                let suffix = self.get_fresh_suffix();

                self.gen_expr(expr, rsp_offset);
                println!("  pop rax");
                println!("  cmp rax, 0");
                println!("  je .Lend{}", suffix);

                self.gen_statement(then_statement, rsp_offset);

                println!(".Lend{}:", suffix);
            }
            Statement::IfElse(expr, then_statement, else_statement) => {
                let suffix = self.get_fresh_suffix();

                self.gen_expr(expr, rsp_offset);
                println!("  pop rax");
                println!("  cmp rax, 0");
                println!("  je .Lelse{}", suffix);

                self.gen_statement(then_statement, rsp_offset);

                println!("  jmp .Lend{}", suffix);
                println!(".Lelse{}:", suffix);

                self.gen_statement(else_statement, rsp_offset);

                println!(".Lend{}:", suffix);
            }
            Statement::While(expr, statement) => {
                let suffix = self.get_fresh_suffix();

                println!(".Lbegin{}:", suffix);

                self.gen_expr(expr, rsp_offset);
                println!("  pop rax");
                println!("  cmp rax, 0");
                println!("  je .Lend{}", suffix);

                self.gen_statement(statement, rsp_offset);

                println!("  jmp .Lbegin{}", suffix);
                println!(".Lend{}:", suffix);
            }
            Statement::For(init, cond, update, body) => {
                let suffix = self.get_fresh_suffix();

                self.gen_expr(init, rsp_offset);
                println!("  pop rax");

                println!(".Lbegin{}:", suffix);

                self.gen_expr(cond, rsp_offset);
                println!("  pop rax");
                println!("  cmp rax, 0");
                println!("  je .Lend{}", suffix);

                self.gen_statement(body, rsp_offset);

                self.gen_expr(update, rsp_offset);
                println!("  pop rax");

                println!("  jmp .Lbegin{}", suffix);
                println!(".Lend{}:", suffix);
            }
            Statement::Block(statements) => {
                self.gen_statements(statements, rsp_offset);
            }
        }
    }

    // gen_expr 一回の呼び出しで rsp_offsetは 8 増える
    fn gen_expr(&self, expr: &Expr, rsp_offset: usize) {
        match expr {
            Expr::Num(n) => {
                println!("  push {n}");
            }
            Expr::Add(lhs, rhs) => {
                self.gen_expr(lhs, rsp_offset);
                self.gen_expr(rhs, rsp_offset + 8);

                println!("  pop rdi");
                println!("  pop rax");

                println!("  add rax, rdi");
                println!("  push rax");
            }

            Expr::Sub(lhs, rhs) => {
                self.gen_expr(lhs, rsp_offset);
                self.gen_expr(rhs, rsp_offset + 8);

                println!("  pop rdi");
                println!("  pop rax");

                println!("  sub rax, rdi");
                println!("  push rax");
            }
            Expr::Mul(lhs, rhs) => {
                self.gen_expr(lhs, rsp_offset);
                self.gen_expr(rhs, rsp_offset + 8);

                println!("  pop rdi");
                println!("  pop rax");

                println!("  imul rax, rdi");

                println!("  push rax");
            }

            Expr::Div(lhs, rhs) => {
                self.gen_expr(lhs, rsp_offset);
                self.gen_expr(rhs, rsp_offset + 8);

                println!("  pop rdi");
                println!("  pop rax");

                println!("  cqo");
                println!("  idiv rdi");

                println!("  push rax");
            }
            Expr::LessThan(lhs, rhs) => {
                self.gen_expr(lhs, rsp_offset);
                self.gen_expr(rhs, rsp_offset + 8);

                println!("  pop rdi");
                println!("  pop rax");

                println!("  cmp rax, rdi");
                println!("  setl al");
                println!("  movzb rax, al");
                println!("  push rax");
            }
            Expr::LessEqual(lhs, rhs) => {
                self.gen_expr(lhs, rsp_offset);
                self.gen_expr(rhs, rsp_offset + 8);

                println!("  pop rdi");
                println!("  pop rax");

                println!("  cmp rax, rdi");
                println!("  setle al");
                println!("  movzb rax, al");
                println!("  push rax");
            }
            Expr::Equal(lhs, rhs) => {
                self.gen_expr(lhs, rsp_offset);
                self.gen_expr(rhs, rsp_offset + 8);

                println!("  pop rdi");
                println!("  pop rax");

                println!("  cmp rax, rdi");
                println!("  sete al");
                println!("  movzb rax, al");
                println!("  push rax");
            }
            Expr::NotEqual(lhs, rhs) => {
                self.gen_expr(lhs, rsp_offset);
                self.gen_expr(rhs, rsp_offset + 8);

                println!("  pop rdi");
                println!("  pop rax");

                println!("  cmp rax, rdi");
                println!("  setne al");
                println!("  movzb rax, al");
                println!("  push rax");
            }
            Expr::GreaterThan(lhs, rhs) => {
                self.gen_expr(lhs, rsp_offset);
                self.gen_expr(rhs, rsp_offset + 8);

                println!("  pop rdi");
                println!("  pop rax");

                println!("  cmp rax, rdi");
                println!("  setg al");
                println!("  movzb rax, al");
                println!("  push rax");
            }
            Expr::GreaterEqual(lhs, rhs) => {
                self.gen_expr(lhs, rsp_offset);
                self.gen_expr(rhs, rsp_offset + 8);

                println!("  pop rdi");
                println!("  pop rax");

                println!("  cmp rax, rdi");
                println!("  setge al");
                println!("  movzb rax, al");
                println!("  push rax");
            }
            Expr::Assign(lhs, rhs) => {
                self.gen_lvalue(lhs);
                self.gen_expr(rhs, rsp_offset + 8);

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
            Expr::FunctionCall(name, args) => {
                let system_v_caller_save_registers = ["rdi", "rsi", "rdx", "rcx", "r8", "r9"];

                for (i, arg) in args.iter().enumerate() {
                    self.gen_expr(arg, rsp_offset + i * 8);
                }

                for i in (0..args.len()).rev() {
                    println!("  pop {}", system_v_caller_save_registers[i]);
                }

                if (rsp_offset % 16) != 0 {
                    println!("  sub rsp, 8");
                }

                println!("  call {}", name);
                println!("  push rax");

                if (rsp_offset % 16) != 0 {
                    println!("  add rsp, 8");
                }
            }
        }
    }

    fn gen_lvalue(&self, expr: &Expr) {
        match expr {
            Expr::Variable(name) => {
                let offset = self.variable_offsets.get(name).expect("variable not found");

                println!("  mov rax, rbp");
                println!("  sub rax, {}", offset);
                println!("  push rax");
            }
            _ => todo!(),
        }
    }
}
