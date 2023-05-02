mod expr;
mod generator;
mod lex;
mod offset_calculator;
mod parser;
mod statement;
mod token;
fn main() {
    let raw_input = std::env::args().nth(1).expect("no arguments");
    let input = raw_input.chars().collect::<Vec<_>>();

    let tokens = &lex::tokenize(&input);

    let mut parser_state = parser::ParserState::new(tokens, &raw_input);

    let mut statements = vec![];

    while !parser_state.fully_parsed() {
        statements.push(parser_state.munch_statement());
    }

    let variable_offsets = offset_calculator::calculate_offset(&statements);

    let mut generator = generator::Generator::new(variable_offsets);

    generator.gen(&statements);
}
