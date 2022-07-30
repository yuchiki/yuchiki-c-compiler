type SourcePosition = usize;

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

    if let [(Token::Num(num), _), tokens @ ..] = tokens {
        println!("  mov rax, {num}");

        let mut tokens = tokens;
        while !tokens.is_empty() {
            match tokens {
                [(Token::Plus, _), (Token::Num(num), _), rest @ ..] => {
                    println!("  add rax, {num}");
                    tokens = rest;
                }
                [(Token::Minus, _), (Token::Num(num), _), rest @ ..] => {
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

fn tokenize(mut input: &[char]) -> Vec<(Token, SourcePosition)> {
    let mut ans = vec![];
    let mut pos: SourcePosition = 0;

    while !input.is_empty() {
        match input {
            ['+', rest @ ..] => {
                ans.push((Token::Plus, pos));
                input = rest;
                pos += 1;
            }
            ['-', rest @ ..] => {
                ans.push((Token::Minus, pos));
                input = rest;
                pos += 1;
            }
            ['0'..='9', ..] => {
                if let (rest, Some(num), char_count) = munch_int(input) {
                    ans.push((Token::Num(num), pos));
                    input = rest;
                    pos += char_count;
                }
            }
            _ => {
                panic!("予期しない文字です。");
            }
        }
    }

    ans
}

fn munch_int(mut input: &[char]) -> (&[char], Option<i32>, usize) {
    let mut char_count = 0;

    if let ['0'..='9', ..] = input {
        let mut ans = 0;
        while let [digit @ '0'..='9', rest @ ..] = input {
            ans = ans * 10 + (*digit as i32) - ('0' as i32);
            input = rest;
            char_count += 1;
        }

        (input, Some(ans), char_count)
    } else {
        (input, None, 0)
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
