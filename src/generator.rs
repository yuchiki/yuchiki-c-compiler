use std::{collections::HashMap, io::Write};

use crate::{expr::Expr, offset_calculator, statement::Statement, top_level::TopLevel};

const SYSTEM_V_CALLER_SAVE_REGISTERS: [&str; 6] = ["rdi", "rsi", "rdx", "rcx", "r8", "r9"];

pub struct Program<'a, W: Write> {
    fresh_counter: usize,
    program: Vec<TopLevel>,
    write: &'a mut W,
}

pub struct Function<'a, W: Write> {
    variable_offsets: HashMap<String, usize>,
    name: String,
    params: Vec<String>,
    body: Vec<Statement>,

    // うまくmutable な composition　が作れなかったのでとりあえずfresh_counterを持たせている
    // base_generator: &'a mut ProgramGenerator,
    fresh_counter: usize,
    write: &'a mut W,
}

impl<'a, W: Write> Program<'a, W> {
    pub fn new(program: Vec<TopLevel>, write: &'a mut W) -> Self {
        Self {
            fresh_counter: 0,
            program,
            write,
        }
    }
    pub fn gen(&mut self) {
        writeln!(self.write, ".intel_syntax noprefix").unwrap();
        writeln!(self.write, "  push rbp").unwrap();
        writeln!(self.write, "  mov rbp, rsp").unwrap();

        writeln!(self.write, "  sub rsp, 8").unwrap();

        writeln!(self.write, "  call main").unwrap();

        writeln!(self.write, "  add rsp, 8").unwrap();

        writeln!(self.write, "  mov rsp, rbp").unwrap();
        writeln!(self.write, "  pop rbp").unwrap();
        writeln!(self.write, "  ret").unwrap();

        let program = self.program.clone();

        for top_level in program {
            self.gen_top_level(&top_level);
        }
    }

    fn gen_top_level(&mut self, top_level: &TopLevel) {
        match top_level {
            TopLevel::FunctionDefinition(name, params, statements) => {
                let mut function_generator = Function::new(
                    name.clone(),
                    offset_calculator::calculate_offset(params, statements),
                    params.clone(),
                    statements.clone(),
                    self.fresh_counter,
                    self.write,
                );
                self.fresh_counter = function_generator.gen();
            }
        }
    }
}

impl<'a, W: Write> Function<'a, W> {
    pub fn new(
        name: String,
        variable_offsets: HashMap<String, usize>,
        params: Vec<String>,
        body: Vec<Statement>,
        fresh_counter: usize,
        write: &'a mut W,
    ) -> Self {
        Self {
            variable_offsets,
            name,
            params,
            body,
            fresh_counter,
            write,
        }
    }

    // 一時的に自前のfresh_counter　を参照する設計に
    fn get_fresh_suffix(&mut self) -> String {
        self.fresh_counter += 1;
        format!("{}", self.fresh_counter)
    }

    fn gen(&mut self) -> usize {
        let variable_offsets = offset_calculator::calculate_offset(&self.params, &self.body);

        writeln!(&mut self.write, ".globl {}", self.name).unwrap();
        writeln!(self.write, "{}:", self.name).unwrap();

        writeln!(self.write, "  push rbp").unwrap();
        writeln!(self.write, "  mov rbp, rsp").unwrap();
        writeln!(self.write, "  sub rsp, {}", self.variable_offsets.len() * 8).unwrap();

        for (i, param) in self.params.iter().enumerate() {
            writeln!(
                self.write,
                "  mov [rbp-{}], {}",
                variable_offsets[param], SYSTEM_V_CALLER_SAVE_REGISTERS[i]
            )
            .unwrap();
        }

        let body = &self.body.clone(); // borrow checker　が通してくれない...

        self.gen_statements(body, (1 + self.variable_offsets.len()) * 8);

        writeln!(self.write, "  mov rsp, rbp").unwrap();
        writeln!(self.write, "  pop rbp").unwrap();
        writeln!(self.write, "  ret").unwrap();

        self.fresh_counter
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
                writeln!(self.write, "  pop rax").unwrap();
            }
            Statement::Return(expr) => {
                self.gen_expr(expr, rsp_offset);
                writeln!(self.write, "  pop rax").unwrap();
                writeln!(self.write, "  mov rsp, rbp").unwrap();
                writeln!(self.write, "  pop rbp").unwrap();
                writeln!(self.write, "  ret").unwrap();
            }
            Statement::If(expr, then_statement) => {
                let suffix = self.get_fresh_suffix();

                self.gen_expr(expr, rsp_offset);
                writeln!(self.write, "  pop rax").unwrap();
                writeln!(self.write, "  cmp rax, 0").unwrap();
                writeln!(self.write, "  je .Lend{suffix}").unwrap();

                self.gen_statement(then_statement, rsp_offset);

                writeln!(self.write, ".Lend{suffix}:").unwrap();
            }
            Statement::IfElse(expr, then_statement, else_statement) => {
                let suffix = self.get_fresh_suffix();

                self.gen_expr(expr, rsp_offset);
                writeln!(self.write, "  pop rax").unwrap();
                writeln!(self.write, "  cmp rax, 0").unwrap();
                writeln!(self.write, "  je .Lelse{suffix}").unwrap();

                self.gen_statement(then_statement, rsp_offset);

                writeln!(self.write, "  jmp .Lend{suffix}").unwrap();
                writeln!(self.write, ".Lelse{suffix}:").unwrap();

                self.gen_statement(else_statement, rsp_offset);

                writeln!(self.write, ".Lend{suffix}:").unwrap();
            }
            Statement::While(expr, statement) => {
                let suffix = self.get_fresh_suffix();

                writeln!(self.write, ".Lbegin{suffix}:").unwrap();

                self.gen_expr(expr, rsp_offset);
                writeln!(self.write, "  pop rax").unwrap();
                writeln!(self.write, "  cmp rax, 0").unwrap();
                writeln!(self.write, "  je .Lend{suffix}").unwrap();

                self.gen_statement(statement, rsp_offset);

                writeln!(self.write, "  jmp .Lbegin{suffix}").unwrap();
                writeln!(self.write, ".Lend{suffix}:").unwrap();
            }
            Statement::For(init, cond, update, body) => {
                let suffix = self.get_fresh_suffix();

                self.gen_expr(init, rsp_offset);
                writeln!(self.write, "  pop rax").unwrap();

                writeln!(self.write, ".Lbegin{suffix}:").unwrap();

                self.gen_expr(cond, rsp_offset);
                writeln!(self.write, "  pop rax").unwrap();
                writeln!(self.write, "  cmp rax, 0").unwrap();
                writeln!(self.write, "  je .Lend{suffix}").unwrap();

                self.gen_statement(body, rsp_offset);

                self.gen_expr(update, rsp_offset);
                writeln!(self.write, "  pop rax").unwrap();

                writeln!(self.write, "  jmp .Lbegin{suffix}").unwrap();
                writeln!(self.write, ".Lend{suffix}:").unwrap();
            }
            Statement::Block(statements) => {
                self.gen_statements(statements, rsp_offset);
            }
        }
    }

    // gen_expr 一回の呼び出しで rsp_offsetは 8 増える
    fn gen_expr(&mut self, expr: &Expr, rsp_offset: usize) {
        match expr {
            Expr::Num(n) => {
                writeln!(self.write, "  push {n}").unwrap();
            }
            Expr::Add(lhs, rhs) => {
                self.gen_binary_operation(lhs, rhs, rsp_offset, &["  add rax, rdi"]);
            }

            Expr::Sub(lhs, rhs) => {
                self.gen_binary_operation(lhs, rhs, rsp_offset, &["  sub rax, rdi"]);
            }
            Expr::Mul(lhs, rhs) => {
                self.gen_binary_operation(lhs, rhs, rsp_offset, &["  imul rax, rdi"]);
            }

            Expr::Div(lhs, rhs) => {
                self.gen_binary_operation(lhs, rhs, rsp_offset, &["  cqo", "idiv rdi"]);
            }
            Expr::LessThan(lhs, rhs) => {
                self.gen_comparator(lhs, rhs, rsp_offset, "setl");
            }
            Expr::LessEqual(lhs, rhs) => {
                self.gen_comparator(lhs, rhs, rsp_offset, "setle");
            }
            Expr::Equal(lhs, rhs) => {
                self.gen_comparator(lhs, rhs, rsp_offset, "sete");
            }
            Expr::NotEqual(lhs, rhs) => {
                self.gen_comparator(lhs, rhs, rsp_offset, "setne");
            }
            Expr::GreaterThan(lhs, rhs) => {
                self.gen_comparator(lhs, rhs, rsp_offset, "setg");
            }
            Expr::GreaterEqual(lhs, rhs) => {
                self.gen_comparator(lhs, rhs, rsp_offset, "setge");
            }
            Expr::Assign(lhs, rhs) => {
                self.gen_lvalue(lhs);
                self.gen_expr(rhs, rsp_offset + 8);

                writeln!(self.write, "  pop rdi").unwrap();
                writeln!(self.write, "  pop rax").unwrap();
                writeln!(self.write, "  mov [rax], rdi").unwrap();
                writeln!(self.write, "  push rdi").unwrap();
            }
            Expr::Variable(_) => {
                self.gen_lvalue(expr);
                writeln!(self.write, "  pop rax").unwrap();
                writeln!(self.write, "  mov rax, [rax]").unwrap();
                writeln!(self.write, "  push rax").unwrap();
            }
            Expr::FunctionCall(name, args) => {
                for (i, arg) in args.iter().enumerate() {
                    self.gen_expr(arg, rsp_offset + i * 8);
                }

                for i in (0..args.len()).rev() {
                    writeln!(self.write, "  pop {}", SYSTEM_V_CALLER_SAVE_REGISTERS[i]).unwrap();
                }

                if (rsp_offset % 16) != 0 {
                    writeln!(self.write, "  sub rsp, 8").unwrap();
                }

                writeln!(self.write, "  call {name}").unwrap();

                if (rsp_offset % 16) != 0 {
                    writeln!(self.write, "  add rsp, 8").unwrap();
                }

                writeln!(self.write, "  push rax").unwrap();
            }
        }
    }

    fn gen_binary_operation(&mut self, lhs: &Expr, rhs: &Expr, rsp_offset: usize, ops: &[&str]) {
        self.gen_expr(lhs, rsp_offset);
        self.gen_expr(rhs, rsp_offset + 8);

        writeln!(self.write, "  pop rdi").unwrap();
        writeln!(self.write, "  pop rax").unwrap();

        for op in ops.iter() {
            writeln!(self.write, "{op}").unwrap();
        }

        writeln!(self.write, "  push rax").unwrap();
    }

    fn gen_comparator(&mut self, lhs: &Expr, rhs: &Expr, rsp_offset: usize, op: &str) {
        self.gen_binary_operation(
            lhs,
            rhs,
            rsp_offset,
            &[
                "  cmp rax, rdi",
                format!("  {op} al").as_str(),
                "  movzb rax, al",
            ],
        );
    }

    fn gen_lvalue(&mut self, expr: &Expr) {
        match expr {
            Expr::Variable(name) => {
                let offset = self.variable_offsets.get(name).expect("variable not found");

                writeln!(self.write, "  mov rax, rbp").unwrap();
                writeln!(self.write, "  sub rax, {offset}").unwrap();
                writeln!(self.write, "  push rax").unwrap();
            }
            _ => todo!(),
        }
    }
}

// test しがたいので test は省略...
