#![deny(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
#![allow(clippy::module_name_repetitions)]
mod expr;
mod function_collector;
mod generator;
mod lex;
mod parser;
mod statement;
mod token;
mod top_level;
mod typed_expr;
mod types;
mod typing;
mod variable_collector;

use std::io::Write;

pub fn process<W: Write>(raw_input: &str, mut write: W) {
    let input = raw_input.chars().collect::<Vec<_>>();
    let tokens = &lex::tokenize(&input);
    let mut parser = parser::Parser::new(tokens, raw_input);
    let program = parser.munch_program();
    let function_type_environment = function_collector::collect_functions(&program);
    let typist = typing::Typist::new(function_type_environment);
    let typed_program = typist.type_program(&program);

    let mut generator = generator::Program::new(typed_program, &mut write);

    generator.gen();
}

// この関数は integration_test でテストされる。
