enum Token {
    Num(i32),
    Plus,
    Minus,
}

fn main() {
    let input = std::env::args()
        .nth(1)
        .expect("no arguments")
        .chars()
        .collect::<Vec<_>>();

    let tokens = tokenize(&input);
    let tokens = &tokens[..];

    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");

    if let [Token::Num(num), tokens @ ..] = tokens {
        println!("  mov rax, {num}");

        let mut tokens = tokens;
        while !tokens.is_empty() {
            match tokens {
                [Token::Plus, Token::Num(num), rest @ ..] => {
                    println!("  add rax, {num}");
                    tokens = rest;
                }
                [Token::Minus, Token::Num(num), rest @ ..] => {
                    println!("  sub rax, {num}");
                    tokens = rest;
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

fn tokenize(mut input: &[char]) -> Vec<Token> {
    let mut ans = vec![];
    while !input.is_empty() {
        match input {
            ['+', rest @ ..] => {
                ans.push(Token::Plus);
                input = rest;
            }
            ['-', rest @ ..] => {
                ans.push(Token::Minus);
                input = rest;
            }
            ['0'..='9', ..] => {
                if let (rest, Some(num)) = munch_int(input) {
                    ans.push(Token::Num(num));
                    input = rest;
                }
            }
            _ => {
                panic!("予期しない文字です。");
            }
        }
    }

    ans
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
