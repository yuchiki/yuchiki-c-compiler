mod expr;
mod generator;
mod lex;
mod offset_calculator;
mod parser;
mod statement;
mod token;
mod top_level;
fn main() {
    let raw_input = std::env::args().nth(1).expect("no arguments");
    let input = raw_input.chars().collect::<Vec<_>>();

    let tokens = &lex::tokenize(&input);

    let mut parser_state = parser::ParserState::new(tokens, &raw_input);

    let program = parser_state.munch_program();

    let mut generator = generator::ProgramGenerator::new(program);

    generator.gen();
}
