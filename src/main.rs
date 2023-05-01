mod expr;
mod generator;
mod lex;
mod parser;
mod statement;
fn main() {
    let raw_input = std::env::args().nth(1).expect("no arguments");
    let input = raw_input.chars().collect::<Vec<_>>();

    let tokens = &lex::tokenize(&input);

    let mut parser_state = parser::ParserState::new(tokens, &raw_input);

    let mut statements = vec![];

    while !parser_state.fully_parsed() {
        statements.push(parser_state.munch_statement());
    }

    generator::gen(statements);
}
