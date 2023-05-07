use std::collections::HashMap;

use crate::{
    expr::Expr,
    expr::TypedExpr,
    statement::{Statement, TypedStatement},
    top_level::{TopLevel, TypedTopLevel},
    types::{FunctionType, Type},
    variable_collector::collect_variables,
};

pub struct Typist {
    function_type_environment: HashMap<String, FunctionType>,
}

impl Typist {
    pub const fn new(function_type_environment: HashMap<String, FunctionType>) -> Self {
        Self {
            function_type_environment,
        }
    }

    pub fn type_program(&self, program: &Vec<TopLevel>) -> Vec<TypedTopLevel> {
        let mut typed_program = Vec::new();
        for top_level in program {
            if let Some(top_level) = self.type_top_level(top_level) {
                typed_program.push(top_level);
            }
        }
        typed_program
    }

    pub fn type_top_level(&self, top_level: &TopLevel) -> Option<TypedTopLevel> {
        match top_level {
            TopLevel::FunctionDefinition(name, args, return_type, statements) => {
                let function_typist = FunctionTypist::new(
                    self.function_type_environment.clone(),
                    name.clone(),
                    args.clone(),
                    return_type.clone(),
                    statements.clone(),
                );

                Some(function_typist.type_function())
            }
            TopLevel::ExternalFunctionDeclaration(_, _, _) => None,
        }
    }
}

pub struct FunctionTypist {
    function_type_environment: HashMap<String, FunctionType>,
    variable_type_environment: HashMap<String, Type>,
    function_name: String,
    function_args: Vec<(String, Type)>,
    function_return_type: Type,
    function_body: Vec<Statement>,
}

impl FunctionTypist {
    pub fn new(
        function_type_environment: HashMap<String, FunctionType>,
        function_name: String,
        function_args: Vec<(String, Type)>,
        function_return_type: Type,
        function_body: Vec<Statement>,
    ) -> Self {
        let local_variable_types = collect_variables(&function_args, &function_body);

        Self {
            function_type_environment,
            variable_type_environment: local_variable_types,
            function_name,
            function_args,
            function_return_type,
            function_body,
        }
    }

    pub fn type_function(&self) -> TypedTopLevel {
        let mut typed_statements = Vec::new();
        for statement in &self.function_body {
            typed_statements.push(self.type_statement(statement));
        }
        TypedTopLevel::FunctionDefinition(
            self.function_name.clone(),
            self.function_args.clone(),
            self.function_return_type.clone(),
            typed_statements,
            self.variable_type_environment.clone(),
        )
    }

    fn type_statement(&self, statement: &Statement) -> TypedStatement {
        match statement {
            Statement::Return(expr) => self.type_return_statement(expr),
            Statement::If(expr, statement) => self.type_if_statement(expr, statement),
            Statement::IfElse(expr, then_statement, else_statement) => {
                self.type_if_else_statement(expr, then_statement, else_statement)
            }
            Statement::For(init, update, cond, body) => {
                self.type_for_statement(init, update, cond, body)
            }
            Statement::While(expr, statements) => self.type_while_statement(expr, statements),
            Statement::Expr(expr) => self.type_expr_statement(expr),
            Statement::VariableDeclaration(name, ty) => {
                Self::type_variable_declaration_statement(name, ty)
            }
            Statement::Block(statements) => self.type_block_statement(statements),
        }
    }

    fn type_return_statement(&self, expr: &Expr) -> TypedStatement {
        let typed_expr = self.type_expr(expr);
        assert_eq!(self.function_return_type, typed_expr.get_type());
        TypedStatement::Return(typed_expr)
    }

    fn type_if_statement(&self, expr: &Expr, statement: &Statement) -> TypedStatement {
        let typed_expr = self.type_expr(expr);
        TypedStatement::If(
            Box::new(typed_expr),
            Box::new(self.type_statement(statement)),
        )
    }

    fn type_if_else_statement(
        &self,
        expr: &Expr,
        then_statement: &Statement,
        else_statement: &Statement,
    ) -> TypedStatement {
        let typed_expr = self.type_expr(expr);
        TypedStatement::IfElse(
            Box::new(typed_expr),
            Box::new(self.type_statement(then_statement)),
            Box::new(self.type_statement(else_statement)),
        )
    }

    fn type_while_statement(&self, expr: &Expr, statement: &Statement) -> TypedStatement {
        let typed_expr = self.type_expr(expr);
        let typed_statement = self.type_statement(statement);
        TypedStatement::While(Box::new(typed_expr), Box::new(typed_statement))
    }

    fn type_for_statement(
        &self,
        init: &Expr,
        update: &Expr,
        cond: &Expr,
        body: &Statement,
    ) -> TypedStatement {
        let typed_init = self.type_expr(init);
        let typed_update = self.type_expr(update);
        let typed_cond = self.type_expr(cond);
        let typed_body = self.type_statement(body);
        TypedStatement::For(
            Box::new(typed_init),
            Box::new(typed_update),
            Box::new(typed_cond),
            Box::new(typed_body),
        )
    }

    fn type_expr_statement(&self, expr: &Expr) -> TypedStatement {
        let typed_expr = self.type_expr(expr);
        TypedStatement::Expr(typed_expr)
    }

    fn type_variable_declaration_statement(name: &str, ty: &Type) -> TypedStatement {
        TypedStatement::VariableDeclaration(name.to_string(), ty.clone())
    }

    fn type_block_statement(&self, statements: &Vec<Statement>) -> TypedStatement {
        let mut typed_statements = Vec::new();
        for statement in statements {
            typed_statements.push(self.type_statement(statement));
        }
        TypedStatement::Block(typed_statements)
    }

    pub fn type_expr(&self, expr: &Expr) -> TypedExpr {
        match expr {
            Expr::Add(lhs, rhs)
            | Expr::Sub(lhs, rhs)
            | Expr::Mul(lhs, rhs)
            | Expr::Div(lhs, rhs) => self.type_arithmetic_operator(lhs, rhs, expr),
            Expr::Num(n) => TypedExpr::IntNum(*n),
            Expr::LessThan(lhs, rhs)
            | Expr::LessEqual(lhs, rhs)
            | Expr::Equal(lhs, rhs)
            | Expr::NotEqual(lhs, rhs)
            | Expr::GreaterThan(lhs, rhs)
            | Expr::GreaterEqual(lhs, rhs) => self.type_comparator(lhs, rhs, expr),
            Expr::Assign(lhs, rhs) => self.type_assign(lhs, rhs),
            Expr::Variable(name) => self.type_variable(name),
            Expr::FunctionCall(name, args) => self.type_function_call(name, args),
            Expr::Address(expr) => self.type_address(expr),
            Expr::Dereference(expr) => self.type_dereference(expr),
            Expr::Sizeof(expr) => TypedExpr::Sizeof(Box::new(self.type_expr(expr))),
        }
    }

    fn type_dereference(&self, expr: &Expr) -> TypedExpr {
        let typed_expr = self.type_expr(expr).decay_if_array();
        if let Type::Pointer(ty) = typed_expr.get_type() {
            TypedExpr::Dereference(*ty, Box::new(typed_expr))
        } else {
            panic!("cannot dereference non-pointer type: {typed_expr:?}")
        }
    }

    fn type_address(&self, expr: &Expr) -> TypedExpr {
        let typed_expr = self.type_expr(expr);
        TypedExpr::Address(
            Type::Pointer(Box::new(typed_expr.get_type())),
            Box::new(typed_expr),
        )
    }

    fn type_variable(&self, name: &String) -> TypedExpr {
        let ty = self
            .variable_type_environment
            .get(name)
            .unwrap_or_else(|| panic!("undefined variable: {name}"));
        TypedExpr::Variable(ty.clone(), name.clone())
    }

    fn type_assign(&self, lhs: &Expr, rhs: &Expr) -> TypedExpr {
        let lhs = self.type_expr(lhs);
        let rhs = self.type_expr(rhs);
        //        assert_eq!(lhs.get_type(), rhs.get_type(), "lhs: {lhs:?}, rhs: {rhs:?}",); // 左にポインタ、右に配列の時困るのでコメントアウト
        assert!(!matches!(lhs.get_type(), Type::Array(_, _)));
        TypedExpr::Assign(
            lhs.get_type(),
            Box::new(lhs),
            Box::new(rhs.decay_if_array()),
        )
    }

    fn type_function_call(&self, name: &String, args: &Vec<Expr>) -> TypedExpr {
        let (arg_types, return_type) = self
            .function_type_environment
            .get(name)
            .unwrap_or_else(|| panic!("undefined function: {name}"));
        assert_eq!(
            arg_types.len(),
            args.len(),
            "arg_types: {arg_types:?}, args: {args:?}",
        );
        let typed_args: Vec<TypedExpr> = args
            .iter()
            .zip(arg_types.iter())
            .map(|(arg, ty)| {
                let typed_arg = self.type_expr(arg);
                assert_eq!(
                    typed_arg.get_type(),
                    *ty,
                    "typed_arg: {typed_arg:?}, t: {ty:?}",
                );
                typed_arg.decay_if_array()
            })
            .collect();
        for i in 0..arg_types.len() {
            let typed_arg = &typed_args[i];
            let arg_type = &arg_types[i];
            assert_eq!(
                typed_arg.get_type(),
                *arg_type,
                "typed_arg: {typed_arg:?}, arg_type: {arg_type:?}",
            );
        }
        TypedExpr::FunctionCall(*return_type.clone(), name.clone(), typed_args)
    }

    fn type_comparator(&self, lhs: &Expr, rhs: &Expr, expr: &Expr) -> TypedExpr {
        let lhs = self.type_expr(lhs);
        let rhs = self.type_expr(rhs);
        assert_eq!(lhs.get_type(), rhs.get_type(), "lhs: {lhs:?}, rhs: {rhs:?}",);
        let constructor = match expr {
            Expr::LessThan(_, _) => TypedExpr::LessThan,
            Expr::LessEqual(_, _) => TypedExpr::LessEqual,
            Expr::Equal(_, _) => TypedExpr::Equal,
            Expr::NotEqual(_, _) => TypedExpr::NotEqual,
            Expr::GreaterThan(_, _) => TypedExpr::GreaterThan,
            Expr::GreaterEqual(_, _) => TypedExpr::GreaterEqual,
            _ => unreachable!(),
        };
        constructor(
            Box::new(lhs.decay_if_array()),
            Box::new(rhs.decay_if_array()),
        )
    }

    fn type_arithmetic_operator(&self, lhs: &Expr, rhs: &Expr, expr: &Expr) -> TypedExpr {
        let lhs = self.type_expr(lhs);
        let rhs = self.type_expr(rhs);
        let constructor = match expr {
            Expr::Add(_, _) => TypedExpr::Add,
            Expr::Sub(_, _) => TypedExpr::Sub,
            Expr::Mul(_, _) => TypedExpr::Mul,
            Expr::Div(_, _) => TypedExpr::Div,
            _ => unreachable!(),
        };
        constructor(
            lhs.get_type(),
            Box::new(lhs.decay_if_array()),
            Box::new(rhs.decay_if_array()),
        )
    }
}
