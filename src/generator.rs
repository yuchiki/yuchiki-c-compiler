use std::collections::HashMap;

use crate::{expr::Expr, offset_calculator, statement::Statement, top_level::TopLevel};

const SYSTEM_V_CALLER_SAVE_REGISTERS: [&str; 6] = ["rdi", "rsi", "rdx", "rcx", "r8", "r9"];

pub struct ProgramGenerator {
    fresh_counter: usize,
    program: Vec<TopLevel>,
}

pub struct FunctionGenerator {
    variable_offsets: HashMap<String, usize>,
    name: String,
    params: Vec<String>,
    body: Vec<Statement>,

    // うまくmutable な composition　が作れなかったのでとりあえずfresh_counterを持たせている
    // base_generator: &'a mut ProgramGenerator,
    fresh_counter: usize,
}

impl ProgramGenerator {
    pub fn new(program: Vec<TopLevel>) -> Self {
        Self {
            fresh_counter: 0,
            program,
        }
    }
    pub fn gen(&mut self) {
        println!(".intel_syntax noprefix");
        println!("  push rbp");
        println!("  mov rbp, rsp");

        println!("  sub rsp, 8");

        println!("  call main");

        println!("  add rsp, 8");

        println!("  mov rsp, rbp");
        println!("  pop rbp");
        println!("  ret");

        let program = self.program.clone();

        for top_level in program {
            self.gen_top_level(&top_level);
        }
    }

    fn gen_top_level(&mut self, top_level: &TopLevel) {
        match top_level {
            TopLevel::FunctionDefinition(name, params, statements) => {
                let mut function_generator = FunctionGenerator::new(
                    name.clone(),
                    offset_calculator::calculate_offset(params, statements),
                    params.clone(),
                    statements.clone(),
                    self.fresh_counter,
                );
                self.fresh_counter = function_generator.gen();
            }
        }
    }
}

impl FunctionGenerator {
    pub fn new(
        name: String,
        variable_offsets: HashMap<String, usize>,
        params: Vec<String>,
        body: Vec<Statement>,
        fresh_counter: usize,
    ) -> Self {
        Self {
            name,
            variable_offsets,
            params,
            body,
            fresh_counter,
        }
    }

    // 一時的に自前のfresh_counter　を参照する設計に
    fn get_fresh_suffix(&mut self) -> String {
        self.fresh_counter += 1;
        format!("{}", self.fresh_counter)
    }

    fn gen(&mut self) -> usize {
        let variable_offsets = offset_calculator::calculate_offset(&self.params, &self.body);

        println!(".globl {}", self.name);
        println!("{}:", self.name);

        println!("  push rbp");
        println!("  mov rbp, rsp");
        println!("  sub rsp, {}", self.variable_offsets.len() * 8);

        for (i, param) in self.params.iter().enumerate() {
            println!(
                "  mov [rbp-{}], {}",
                variable_offsets[param], SYSTEM_V_CALLER_SAVE_REGISTERS[i]
            );
        }

        let body = &self.body.clone(); // borrow checker　が通してくれない...

        self.gen_statements(body, (1 + self.variable_offsets.len()) * 8);

        println!("  mov rsp, rbp");
        println!("  pop rbp");
        println!("  ret");

        self.fresh_counter
    }

    fn gen_statements(&mut self, statements: &Vec<Statement>, rsp_offset: usize) {
        for statement in statements {
            self.gen_statement(&statement, rsp_offset);
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
                for (i, arg) in args.iter().enumerate() {
                    self.gen_expr(arg, rsp_offset + i * 8);
                }

                for i in (0..args.len()).rev() {
                    println!("  pop {}", SYSTEM_V_CALLER_SAVE_REGISTERS[i]);
                }

                if (rsp_offset % 16) != 0 {
                    println!("  sub rsp, 8");
                }

                println!("  call {}", name);

                if (rsp_offset % 16) != 0 {
                    println!("  add rsp, 8");
                }

                println!("  push rax");
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
