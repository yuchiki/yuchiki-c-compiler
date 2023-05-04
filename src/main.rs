use yuchiki_c_compiler::process;

fn main() {
    let raw_input = std::env::args().nth(1).expect("no arguments");
    process(&raw_input, std::io::stdout().lock());
}
