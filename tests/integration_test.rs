extern crate rand;
extern crate yuchiki_c_compiler;

use std::process::Command;

use rand::Rng;

use rstest::rstest;
use yuchiki_c_compiler::process;

const OUT_FILE_BASE_NAME: &str = "tmpdir/tmp";
const EXTERNAL_FUNC_FILE_BASE_NAME: &str = "tmpdir/external_func";

#[rstest]
#[case::return_0("main () { return 0; }", 0)]
#[case::return_any_number("main () { return 42; }", 42)]
#[case::addition("main () { return 1+2; }", 3)]
#[case::subtraction("main () { return 5 + 20 - 4; }", 21)]
#[case::multiplication("main () { return 1  + 2 * 3; }", 7)]
#[case::division("main () { return 3 + 10 / 5; }", 5)]
#[case::paren("main () { return 2 * (3 + 4); }", 14)]
#[case::unary_minus("main () { return -10+20; }", 10)]
#[case::unary_plus("main () { return +10+20; }", 30)]
#[case::equality_equal("main () { return 10 == 10; }", 1)]
#[case::equality_not_equal("main () { return 10 == 20; }", 0)]
#[case::inequality_equal("main () { return 10 != 10; }", 0)]
#[case::inequality_not_equal("main () { return 10 != 20; }", 1)]
#[case::lessthan_less("main () { return 10 < 20; }", 1)]
#[case::lessthan_equal("main () { return 10 < 10; }", 0)]
#[case::lessthan_greater("main () { return 20 < 10; }", 0)]
#[case::lessthanorequal_less("main () { return 10 <= 20; }", 1)]
#[case::lessthanorequal_equal("main () { return 10 <= 10; }", 1)]
#[case::lessthanorequal_greater("main () { return 20 <= 10; }", 0)]
#[case::greaterthan_greater("main () { return 20 > 10; }", 1)]
#[case::greaterthan_equal("main () { return 10 > 10; }", 0)]
#[case::greaterthan_less("main () { return 10 > 20; }", 0)]
#[case::greaterthanorequal_greater("main () { return 20 >= 10; }", 1)]
#[case::greaterthanorequal_equal("main () { return 10 >= 10; }", 1)]
#[case::greaterthanorequal_less("main () { return 10 >= 20; }", 0)]
#[case::multiple_statement("main () { 1; 2; 3; }", 3)]
#[case::return_statement("main () { 1; return 2; 3; }", 2)]
#[case::assign("main () { num = 3; }", 3)]
#[case::variable("main () { num = 3; return num + 2;}", 5)]
#[case::chained_assign("main () { num1 = num2 = 3; return num1 + num2; }", 6)]
#[case::multiple_return("main () { return 3; return 5; }", 3)]
#[case::if_true("main () { if (1) return 3; return 5; }", 3)]
#[case::if_false("main () { if (0) return 3; return 5; }", 5)]
#[case::if_else_true("main () { if (1) return 3; else return 5; }", 3)]
#[case::if_else_false("main () { if (0) return 3; else return 5; }", 5)]
#[case::while_loop("main () { i = 1; while (i < 10) i = i * 2; return i; }", 16)]
#[case::for_loop(
    "main () { sum = 0; for (i = 0; i <= 10; i = i + 1) sum = sum + i; return sum; }",
    55
)]
#[case::while_with_block(
    "main () { i = sum = 0; while(i <= 10) { sum = sum + i; i = i + 1; } return sum; }",
    55
)]
#[case::external_function_call("main () { external_func(1,2,3,4,5,6); }", 91)]
#[case::function_call( "my_func(a, b, c, d, e, f){g = 7; h = a + b * 2 + c * 3 + d * 4 + e * 5 + f * 6 + g; return h / 2;} main(){my_func(1,2,3,4,5,6);}", 49)]
#[case::pointer_dereference("main () { a = 5; return f(&a);} f (pointer) { return *pointer; } ", 5)]
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
