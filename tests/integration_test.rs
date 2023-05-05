extern crate rand;
extern crate yuchiki_c_compiler;

use std::process::Command;

use rand::Rng;

use rstest::rstest;
use yuchiki_c_compiler::process;

const OUT_FILE_BASE_NAME: &str = "tmpdir/tmp";
const EXTERNAL_FUNC_FILE_BASE_NAME: &str = "tmpdir/external_func";

#[rstest]
#[case("main () { return 0; }", 0)]
#[case("main () { return 42; }", 42)]
#[case("main () { return 1+2; }", 3)]
#[case("main () { return 5 + 20 - 4; }", 21)]
#[case("main () { return 1  + 2 * 3; }", 7)]
#[case("main () { return 3 + 10 / 5; }", 5)]
#[case("main () { return 2 * (3 + 4); }", 14)]
#[case("main () { return -10+20; }", 10)]
#[case("main () { return +10+20; }", 30)]
#[case("main () { return 10 == 10; }", 1)]
#[case("main () { return 10 == 20; }", 0)]
#[case("main () { return 10 != 10; }", 0)]
#[case("main () { return 10 != 20; }", 1)]
#[case("main () { return 10 < 20; }", 1)]
#[case("main () { return 10 < 10; }", 0)]
#[case("main () { return 20 < 10; }", 0)]
#[case("main () { return 10 <= 20; }", 1)]
#[case("main () { return 10 <= 10; }", 1)]
#[case("main () { return 20 <= 10; }", 0)]
#[case("main () { return 20 > 10; }", 1)]
#[case("main () { return 10 > 10; }", 0)]
#[case("main () { return 10 > 20; }", 0)]
#[case("main () { return 20 >= 10; }", 1)]
#[case("main () { return 10 >= 10; }", 1)]
#[case("main () { return 10 >= 20; }", 0)]
#[case("main () { 1; 2; 3; }", 3)]
#[case("main () { 1; return 2; 3; }", 2)]
#[case("main () { num = 3; }", 3)]
#[case("main () { num = 3; return num + 2;}", 5)]
#[case("main () { num1 = num2 = 3; return num1 + num2; }", 6)]
#[case("main () { return 3; return 5; }", 3)]
#[case("main () { if (1) return 3; return 5; }", 3)]
#[case("main () { if (0) return 3; return 5; }", 5)]
#[case("main () { if (1) return 3; else return 5; }", 3)]
#[case("main () { if (0) return 3; else return 5; }", 5)]
#[case("main () { i = 1; while (i < 10) i = i * 2; return i; }", 16)]
#[case(
    "main () { sum = 0; for (i = 0; i <= 10; i = i + 1) sum = sum + i; return sum; }",
    55
)]
#[case(
    "main () { i = sum = 0; while(i <= 10) { sum = sum + i; i = i + 1; } return sum; }",
    55
)]
#[case("main () { external_func(1,2,3,4,5,6); }", 91)]
#[case( "my_func(a, b, c, d, e, f){g = 7; h = a + b * 2 + c * 3 + d * 4 + e * 5 + f * 6 + g; return h / 2;} main(){my_func(1,2,3,4,5,6);}", 49)]
fn integration_test(#[case] input: &str, #[case] expected: i32) {
    let mut failure_count = 0;
    let status = loop {
        if let Some(status) = execute_test_case(input) {
            break status;
        } else if failure_count == 5 {
            panic!("failed to execute test case");
        }

        failure_count += 1;
    };

    assert_eq!(status, expected);
}

fn execute_test_case(input: &str) -> Option<i32> {
    let suffix = rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(50)
        .map(char::from)
        .collect::<String>();

    {
        let write = std::fs::File::create(format!("{}-{}.s", OUT_FILE_BASE_NAME, suffix)).unwrap();
        process(input, write);
    }

    Command::new("gcc")
        .arg("-S")
        .arg("-o")
        .arg(format!("{}.s", EXTERNAL_FUNC_FILE_BASE_NAME))
        .arg(format!("{}.c", EXTERNAL_FUNC_FILE_BASE_NAME))
        .output()
        .ok()?;

    Command::new("gcc")
        .arg("--static")
        .arg("-o")
        .arg(format!("{}-{}", OUT_FILE_BASE_NAME, suffix))
        .arg(format!("{}-{}.s", OUT_FILE_BASE_NAME, suffix))
        .arg(format!("{}.s", EXTERNAL_FUNC_FILE_BASE_NAME))
        .output()
        .ok()?;

    let status = Command::new(format!("./{}-{}", OUT_FILE_BASE_NAME, suffix)).status();

    Command::new("rm")
        .arg(format!("{}-{}.s", OUT_FILE_BASE_NAME, suffix))
        .arg(format!("{}-{}", OUT_FILE_BASE_NAME, suffix))
        .output()
        .ok()?;

    status.ok()?.code()
}
