fn main() {
    let input = std::env::args()
        .nth(1)
        .expect("no arguments")
        .chars()
        .collect::<Vec<_>>();

    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");

    if let (mut input, Some(num)) = munch_int(&input) {
        println!("  mov rax, {num}");

        while !input.is_empty() {
            match input {
                ['+', rest @ ..] => {
                    if let (new_input, Some(num)) = munch_int(rest) {
                        println!("  add rax, {num}");
                        input = new_input;
                    }
                }
                ['-', rest @ ..] => {
                    if let (new_input, Some(num)) = munch_int(rest) {
                        println!("  sub rax, {num}");
                        input = new_input;
                    }
                }
                _ => {
                    panic!("予期しない文字です。");
                }
            }
        }
    } else {
        panic!("数から始まっていない");
    }

    println!("  ret");
}

fn munch_int(mut input: &[char]) -> (&[char], Option<i32>) {
    if let ['0'..='9', ..] = input {
        let mut ans = 0;

        while let [digit @ '0'..='9', rest @ ..] = input {
            ans = ans * 10 + (*digit as i32) - ('0' as i32);
            input = rest;
        }

        (input, Some(ans))
    } else {
        (input, None)
    }
}

#[test]
fn test_munch_int() {
    assert_eq!(munch_int(&['0', 'a'][..]), (&['a'][..], Some(0)));
    assert_eq!(
        munch_int(&['3', '5', '4', 'a', 'b', 'c'][..]),
        (&['a', 'b', 'c'][..], Some(354))
    );
    assert_eq!(munch_int(&['a', 'b'][..]), (&['a', 'b'][..], None));
}
