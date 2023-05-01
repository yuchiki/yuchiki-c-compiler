mod expr;
mod generator;
mod lex;
mod parser;
fn main() {
    let raw_input = std::env::args().nth(1).expect("no arguments");
    let input = raw_input.chars().collect::<Vec<_>>();

    let tokens = &lex::tokenize(&input);

    let mut parser_state = parser::ParserState::new(tokens, &raw_input);

    let expr = parser_state.munch_expr();
    if !parser_state.fully_parsed() {
        panic!("parseした後にtokensがあまっている");
    }

    generator::gen(expr);
}
