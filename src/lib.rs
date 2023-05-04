#![deny(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
mod expr;
mod generator;
mod lex;
mod offset_calculator;
mod parser;
mod statement;
mod token;
mod top_level;

use std::io::Write;

pub fn process<W: Write>(raw_input: &str, mut write: W) {
    let input = raw_input.chars().collect::<Vec<_>>();

    let tokens = &lex::tokenize(&input);

    let mut parser_state = parser::Parser::new(tokens, raw_input);

    let program = parser_state.munch_program();

    let mut generator = generator::Program::new(program, &mut write);

    generator.gen();
}

// この関数は integration_test でテストされる。
