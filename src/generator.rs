use std::{
    collections::{hash_map, HashMap},
    io::Write,
};

use crate::{expr::TypedExpr, statement::TypedStatement, top_level::TypedTopLevel, types::Type};

const SYSTEM_V_CALLER_SAVE_REGISTERS: [&str; 6] = ["rdi", "rsi", "rdx", "rcx", "r8", "r9"];

pub struct Program<'a, W: Write> {
    fresh_counter: usize,
    program: Vec<TypedTopLevel>,
    write: &'a mut W,
}

impl<'a, W: Write> Program<'a, W> {
    pub fn new(program: Vec<TypedTopLevel>, write: &'a mut W) -> Self {
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

    fn gen_top_level(&mut self, top_level: &TypedTopLevel) {
        match top_level {
            TypedTopLevel::FunctionDefinition(
                name,
                params,
                _,
                statements,
                variable_type_environment,
            ) => {
                let mut function_generator = Function::new(
                    name.clone(),
                    variable_type_environment,
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

pub struct Function<'a, W: Write> {
    variable_offsets: HashMap<String, usize>,
    variables_offset: usize,
    name: String,
    params: Vec<(String, Type)>,
    body: Vec<TypedStatement>,

    // TODO: うまくmutable な composition　が作れなかったのでとりあえずfresh_counterを持たせている
    // base_generator: &'a mut ProgramGenerator,
    fresh_counter: usize,
    write: &'a mut W,
    rsp_offset: usize,
}

impl<'a, W: Write> Function<'a, W> {
    pub fn new(
        name: String,
        variable_type_environment: &HashMap<String, Type>,
        params: Vec<(String, Type)>,
        body: Vec<TypedStatement>,
        fresh_counter: usize,
        write: &'a mut W,
    ) -> Self {
        let (variable_offsets, variables_offset) =
            Self::calc_variable_offset(variable_type_environment);
        Self {
            variable_offsets,
            variables_offset,
            name,
            params,
            body,
            fresh_counter,
            write,
            rsp_offset: 8 + variables_offset,
        }
    }

    // TODO: 一時的に自前のfresh_counter　を参照する設計に
    fn get_fresh_suffix(&mut self) -> String {
        self.fresh_counter += 1;
        format!("{}", self.fresh_counter)
    }

    fn calc_variable_offset(
        local_variable_type_environment: &HashMap<String, Type>,
    ) -> (HashMap<String, usize>, usize) {
        let mut offset_map = HashMap::new();
        let mut offset = 8;
        for (variable, ty) in local_variable_type_environment {
            if let hash_map::Entry::Vacant(e) = offset_map.entry(variable.clone()) {
                e.insert(offset);
                offset += round_up_as_multiple_of_8(ty.get_size());
            }
        }
        (offset_map, offset)
    }
    fn gen(&mut self) -> usize {
        writeln!(&mut self.write, ".globl {}", self.name).unwrap();
        writeln!(self.write, "{}:", self.name).unwrap();

        writeln!(self.write, "  push rbp").unwrap();
        writeln!(self.write, "  mov rbp, rsp").unwrap();
        writeln!(self.write, "  sub rsp, {}", self.variables_offset).unwrap();

        for (i, param) in self.params.iter().enumerate() {
            writeln!(
                self.write,
                "  mov [rbp-{}], {}",
                self.variable_offsets[&param.0], SYSTEM_V_CALLER_SAVE_REGISTERS[i]
            )
            .unwrap();
        }

        let body = &self.body.clone(); // TODO: borrow checker　が通してくれない...

        self.gen_statements(body);

        writeln!(self.write, "  mov rsp, rbp").unwrap();
        writeln!(self.write, "  pop rbp").unwrap();
        writeln!(self.write, "  ret").unwrap();

        self.fresh_counter
    }

    fn gen_statements(&mut self, statements: &Vec<TypedStatement>) {
        for statement in statements {
            self.gen_statement(statement);
        }
    }

    fn gen_statement(&mut self, statement: &TypedStatement) {
        match statement {
            TypedStatement::VariableDeclaration(_, _) => {}
            TypedStatement::Expr(expr) => {
                self.gen_expr(expr);
                writeln!(self.write, "  pop rax").unwrap();
            }
            TypedStatement::Return(expr) => {
                self.gen_expr(expr);
                writeln!(self.write, "  pop rax").unwrap();
                writeln!(self.write, "  mov rsp, rbp").unwrap();
                writeln!(self.write, "  pop rbp").unwrap();
                writeln!(self.write, "  ret").unwrap();
            }
            TypedStatement::If(expr, then_statement) => {
                let suffix = self.get_fresh_suffix();

                self.gen_expr(expr);
                writeln!(self.write, "  pop rax").unwrap();
                writeln!(self.write, "  cmp rax, 0").unwrap();
                writeln!(self.write, "  je .Lend{suffix}").unwrap();

                self.gen_statement(then_statement);

                writeln!(self.write, ".Lend{suffix}:").unwrap();
            }
            TypedStatement::IfElse(expr, then_statement, else_statement) => {
                let suffix = self.get_fresh_suffix();

                self.gen_expr(expr);
                writeln!(self.write, "  pop rax").unwrap();
                writeln!(self.write, "  cmp rax, 0").unwrap();
                writeln!(self.write, "  je .Lelse{suffix}").unwrap();

                self.gen_statement(then_statement);

                writeln!(self.write, "  jmp .Lend{suffix}").unwrap();
                writeln!(self.write, ".Lelse{suffix}:").unwrap();

                self.gen_statement(else_statement);

                writeln!(self.write, ".Lend{suffix}:").unwrap();
            }
            TypedStatement::While(expr, statement) => {
                let suffix = self.get_fresh_suffix();

                writeln!(self.write, ".Lbegin{suffix}:").unwrap();

                self.gen_expr(expr);
                writeln!(self.write, "  pop rax").unwrap();
                writeln!(self.write, "  cmp rax, 0").unwrap();
                writeln!(self.write, "  je .Lend{suffix}").unwrap();

                self.gen_statement(statement);

                writeln!(self.write, "  jmp .Lbegin{suffix}").unwrap();
                writeln!(self.write, ".Lend{suffix}:").unwrap();
            }
            TypedStatement::For(init, cond, update, body) => {
                let suffix = self.get_fresh_suffix();

                self.gen_expr(init);
                writeln!(self.write, "  pop rax").unwrap();

                writeln!(self.write, ".Lbegin{suffix}:").unwrap();

                self.gen_expr(cond);
                writeln!(self.write, "  pop rax").unwrap();
                writeln!(self.write, "  cmp rax, 0").unwrap();
                writeln!(self.write, "  je .Lend{suffix}").unwrap();

                self.gen_statement(body);

                self.gen_expr(update);
                writeln!(self.write, "  pop rax").unwrap();

                writeln!(self.write, "  jmp .Lbegin{suffix}").unwrap();
                writeln!(self.write, ".Lend{suffix}:").unwrap();
            }
            TypedStatement::Block(statements) => {
                self.gen_statements(statements);
            }
        }
    }

    fn gen_expr(&mut self, expr: &TypedExpr) {
        match expr {
            TypedExpr::IntNum(n) => {
                writeln!(self.write, "  push {n}").unwrap();
            }
            TypedExpr::Add(_, lhs, rhs) => {
                self.gen_add_sub_operation(lhs, rhs, "add");
            }

            TypedExpr::Sub(_, lhs, rhs) => {
                self.gen_add_sub_operation(lhs, rhs, "sub");
            }
            TypedExpr::Mul(_, lhs, rhs) => {
                self.gen_binary_operation(lhs, rhs, &["  imul rax, rdi"]);
            }

            TypedExpr::Div(_, lhs, rhs) => {
                self.gen_binary_operation(lhs, rhs, &["  cqo", "idiv rdi"]);
            }
            TypedExpr::LessThan(lhs, rhs) => {
                self.gen_comparator(lhs, rhs, "setl");
            }
            TypedExpr::LessEqual(lhs, rhs) => {
                self.gen_comparator(lhs, rhs, "setle");
            }
            TypedExpr::Equal(lhs, rhs) => {
                self.gen_comparator(lhs, rhs, "sete");
            }
            TypedExpr::NotEqual(lhs, rhs) => {
                self.gen_comparator(lhs, rhs, "setne");
            }
            TypedExpr::GreaterThan(lhs, rhs) => {
                self.gen_comparator(lhs, rhs, "setg");
            }
            TypedExpr::GreaterEqual(lhs, rhs) => {
                self.gen_comparator(lhs, rhs, "setge");
            }
            TypedExpr::Assign(_, lhs, rhs) => {
                self.gen_lvalue(lhs);
                self.rsp_offset += 8;
                self.gen_expr(rhs);
                self.rsp_offset -= 8;

                let di_register = match rhs.get_type().get_size() {
                    4 => "edi",
                    8 => "rdi",
                    _ => panic!("unexpected size"),
                };

                writeln!(self.write, "  pop rdi").unwrap();
                writeln!(self.write, "  pop rax").unwrap();
                writeln!(self.write, "  mov [rax], {di_register}").unwrap();
                writeln!(self.write, "  push rdi").unwrap();
            }
            TypedExpr::Variable(ty, _) => {
                let ax_register = match ty.get_size() {
                    4 => "eax",
                    8 => "rax",
                    _ => panic!("unexpected size"),
                };

                self.gen_lvalue(expr);
                writeln!(self.write, "  pop rax").unwrap();
                writeln!(self.write, "  mov {ax_register}, [rax]").unwrap();
                writeln!(self.write, "  push rax").unwrap();
            }
            TypedExpr::FunctionCall(_, name, args) => {
                for (i, arg) in args.iter().enumerate() {
                    self.gen_expr(arg);
                    self.rsp_offset += 8;
                }

                self.rsp_offset -= args.len() * 8;

                for i in (0..args.len()).rev() {
                    writeln!(self.write, "  pop {}", SYSTEM_V_CALLER_SAVE_REGISTERS[i]).unwrap();
                }

                let misalignment = self.rsp_offset % 16;
                writeln!(self.write, "  sub rsp, {misalignment}").unwrap();
                writeln!(self.write, "  call {name}").unwrap();
                writeln!(self.write, "  add rsp, {misalignment}").unwrap();
                writeln!(self.write, "  push rax").unwrap();
            }
            TypedExpr::Address(_, expr) => {
                self.gen_lvalue(expr);
            }
            TypedExpr::Dereference(_, expr) => {
                self.gen_expr(expr);
                writeln!(self.write, "  pop rax").unwrap();
                writeln!(self.write, "  mov rax, [rax]").unwrap();
                writeln!(self.write, "  push rax").unwrap();
            }
            TypedExpr::Sizeof(expr) => {
                self.gen_expr(expr);
                writeln!(self.write, "  pop rax").unwrap();
                writeln!(self.write, "  mov rax, {}", expr.get_type().get_size()).unwrap();
                writeln!(self.write, "  push rax").unwrap();
            }
        }
    }

    fn gen_add_sub_operation(&mut self, lhs: &TypedExpr, rhs: &TypedExpr, op: &str) {
        match (lhs.get_type(), rhs.get_type()) {
            (Type::Pointer(_), Type::Pointer(_)) => {
                panic!("pointer + pointer is not supported")
            }
            (Type::Pointer(_), _) => {
                self.gen_binary_operation(
                    lhs,
                    rhs,
                    &["  imul rdi, 8", &format!("  {op} rax, rdi")],
                );
            }
            (_, Type::Pointer(_)) => {
                self.gen_binary_operation(
                    lhs,
                    rhs,
                    &["  imul rax, 8", &format!("  {op} rax, rdi")],
                );
            }
            _ => {
                self.gen_binary_operation(lhs, rhs, &[&format!("  {op} rax, rdi")]);
            }
        }
    }

    fn gen_binary_operation(&mut self, lhs: &TypedExpr, rhs: &TypedExpr, ops: &[&str]) {
        self.gen_expr(lhs);
        self.rsp_offset += 8;
        self.gen_expr(rhs);
        self.rsp_offset -= 8;

        writeln!(self.write, "  pop rdi").unwrap();
        writeln!(self.write, "  pop rax").unwrap();

        for op in ops.iter() {
            writeln!(self.write, "{op}").unwrap();
        }

        writeln!(self.write, "  push rax").unwrap();
    }

    fn gen_comparator(&mut self, lhs: &TypedExpr, rhs: &TypedExpr, op: &str) {
        self.gen_binary_operation(
            lhs,
            rhs,
            &[
                "  cmp rax, rdi",
                format!("  {op} al").as_str(),
                "  movzb rax, al",
            ],
        );
    }

    fn gen_lvalue(&mut self, expr: &TypedExpr) {
        match expr {
            TypedExpr::Variable(_, name) => {
                let error_message = format!("variable {name} not found");

                let offset = self.variable_offsets.get(name).expect(&error_message);

                writeln!(self.write, "  mov rax, rbp").unwrap();
                writeln!(self.write, "  sub rax, {offset}").unwrap();
                writeln!(self.write, "  push rax").unwrap();
            }
            TypedExpr::Dereference(_, expr) => {
                self.gen_expr(expr);
            }
            _ => todo!(),
        }
    }
}

pub const fn round_up_as_multiple_of_8(num: usize) -> usize {
    if num % 8 == 0 {
        num
    } else {
        num + 8 - (num % 8)
    }
}

// test しがたいので test は省略...
