fn main() {
    let num = std::env::args().nth(1).expect("no arguments");

    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");
    println!("  mov rax, {num}");
    println!("  ret");
}
