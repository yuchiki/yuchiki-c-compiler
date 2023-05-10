extern crate rand;
extern crate yuchiki_c_compiler;

use std::process::Command;

use rand::Rng;

use rstest::rstest;
use yuchiki_c_compiler::process;

const OUT_FILE_BASE_NAME: &str = "tmpdir/tmp";
const EXTERNAL_FUNC_FILE_BASE_NAME: &str = "tmpdir/external_func";

#[rstest]
#[case::return_0("int main () { return 0; }", 0)]
#[case::return_any_number("int main () { return 42; }", 42)]
#[case::addition("int main () { return 1+2; }", 3)]
#[case::subtraction("int main () { return 5 + 20 - 4; }", 21)]
#[case::multiplication("int main () { return 1  + 2 * 3; }", 7)]
#[case::division("int main () { return 3 + 10 / 5; }", 5)]
#[case::paren("int main () { return 2 * (3 + 4); }", 14)]
#[case::unary_minus("int main () { return -10+20; }", 10)]
#[case::unary_plus("int main () { return +10+20; }", 30)]
#[case::equality_equal("int main () { return 10 == 10; }", 1)]
#[case::equality_not_equal("int main () { return 10 == 20; }", 0)]
#[case::inequality_equal("int main () { return 10 != 10; }", 0)]
#[case::inequality_not_equal("int main () { return 10 != 20; }", 1)]
#[case::lessthan_less("int main () { return 10 < 20; }", 1)]
#[case::lessthan_equal("int main () { return 10 < 10; }", 0)]
#[case::lessthan_greater("int main () { return 20 < 10; }", 0)]
#[case::lessthanorequal_less("int main () { return 10 <= 20; }", 1)]
#[case::lessthanorequal_equal("int main () { return 10 <= 10; }", 1)]
#[case::lessthanorequal_greater("int main () { return 20 <= 10; }", 0)]
#[case::greaterthan_greater("int main () { return 20 > 10; }", 1)]
#[case::greaterthan_equal("int main () { return 10 > 10; }", 0)]
#[case::greaterthan_less("int main () { return 10 > 20; }", 0)]
#[case::greaterthanorequal_greater("int main () { return 20 >= 10; }", 1)]
#[case::greaterthanorequal_equal("int main () { return 10 >= 10; }", 1)]
#[case::greaterthanorequal_less("int main () { return 10 >= 20; }", 0)]
#[case::multiple_statement("int main () { 1; 2; 3; }", 3)]
#[case::return_statement("int main () { 1; return 2; 3; }", 2)]
#[case::assign("int main () {int num; num = 3; }", 3)]
#[case::variable("int main () {int num; num = 3; return num + 2;}", 5)]
#[case::chained_assign(
    "int main () {int num1; int num2;  num1 = num2 = 3; return num1 + num2; }",
    6
)]
#[case::multiple_return("int main () { return 3; return 5; }", 3)]
#[case::if_true("int main () { if (1) return 3; return 5; }", 3)]
#[case::if_false("int main () { if (0) return 3; return 5; }", 5)]
#[case::if_else_true("int main () { if (1) return 3; else return 5; }", 3)]
#[case::if_else_false("int main () { if (0) return 3; else return 5; }", 5)]
#[case::while_loop(
    "int main () { int i; i = 1; while (i < 10) i = i * 2; return i; }",
    16
)]
#[case::for_loop(
    "int main () {int sum; int i; sum = 0; for (i = 0; i <= 10; i = i + 1) sum = sum + i; return sum; }",
    55
)]
#[case::while_with_block(
    "int main () {int i; int sum;  i = sum = 0; while(i <= 10) { sum = sum + i; i = i + 1; } return sum; }",
    55
)]
#[case::external_function_call("extern int external_func(int a, int b, int c, int d, int e, int f); int main () { external_func(1,2,3,4,5,6); }", 91)]
#[case::function_call_without_args("int my_func() { return 3; } int main () { my_func(); }", 3)]
#[case::function_call_with_an_arg(
    "int my_func(int a) { int b; b = 3; return a + b; } int main () { my_func(3); }",
    6
)]
#[case::function_call_with_two_args(
    "int my_func(int a, int b) { int c; c = 10; return a; } int main () { my_func(3, 5); }",
    3
)]
#[case::function_call( "int my_func(int a, int b, int c, int d, int e, int f){int g; int h; g = 7; h = a + b * 2 + c * 3 + d * 4 + e * 5 + f * 6 + g; return h / 2;} int main(){my_func(1,2,3,4,5,6);}", 49)]
#[case::pointer_dereference(
    "int main () {int a; a = 5; return f(&a); return a; } int f (int *pointer) { *pointer = *pointer + 5 ; } ",
    10
)]
#[case::pointer_addition(
    "extern int *test_malloc_4(); int main() { int *a; a = test_malloc_4(); return *(a + 2); }",
    3
)]
#[case::pointer_subtraction(
    "extern int *test_malloc_4(); int main() { int *a; a = test_malloc_4() + 2; return *(a-1); }",
    2
)]
#[case::pointer_complicated_calculation(
    "extern int *test_malloc_4(); int main() { int *a; a = test_malloc_4(); return *(a+(2*2-3)) ; }",
    2
)]
#[case::sizeof_int("int main() { return sizeof(1); }", 4)]
#[case::sizeof_pointer("int main() { int a; return sizeof(&a); }", 8)]
#[case::access_array(
    "int main() { int a[2]; *a = 1; *(a + 1) = 2; return *a + *(a + 1); }",
    3
)]
#[case::access_array_with_variable(
    "int main() { int a[2]; int i; i = 1; *a = 1; *(a + i) = 2; return *a + *(a + i); }",
    3
)]
#[case::accessing_array_does_not_pollute_array(
    "int main() { int a[2]; *a = 5; *(a + 1) = 7; return *a; }",
    5
)]
#[case::array_access_sugar("int main() { int a[2]; a[0] = 1; a[1] = 2; return a[0] + a[1]; }", 3)]
fn integration_test(#[case] input: &str, #[case] expected: i32) {
    let mut failure_count = 0;
    let status = loop {
        match execute_test_case(input) {
            Ok(status) => break status,
            Err(e) => {
                if failure_count == 5 {
                    panic!("failed to execute test case: {}", e);
                }
            }
        }
        failure_count += 1;
    };

    assert_eq!(status, expected);
}

fn execute_test_case(input: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let suffix = rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(50)
        .map(char::from)
        .collect::<String>();

    {
        let write = std::fs::File::create(format!("{}-{}.s", OUT_FILE_BASE_NAME, suffix))
            .expect("cannot open the assembly file as create mode");
        process(input, write);
    }

    Command::new("gcc")
        .arg("-S")
        .arg("-o")
        .arg(format!("{}.s", EXTERNAL_FUNC_FILE_BASE_NAME))
        .arg(format!("{}.c", EXTERNAL_FUNC_FILE_BASE_NAME))
        .output()
        .or(Err("error on gcc-ing the external file"))?;

    let gcc_output = Command::new("gcc")
        .arg("--static")
        .arg("-o")
        .arg(format!("{}-{}", OUT_FILE_BASE_NAME, suffix))
        .arg(format!("{}-{}.s", OUT_FILE_BASE_NAME, suffix))
        .arg(format!("{}.s", EXTERNAL_FUNC_FILE_BASE_NAME))
        .output()
        .or(Err("error on gcc-ing the generated assembly"))?;
    let status = Command::new(format!("./{}-{}", OUT_FILE_BASE_NAME, suffix)).status();

    Command::new("rm")
        .arg(format!("{}-{}.s", OUT_FILE_BASE_NAME, suffix))
        .arg(format!("{}-{}", OUT_FILE_BASE_NAME, suffix))
        .output()?;

    status?.code().ok_or_else(|| {
        format!(
            "failed to execute test case: stderr: {}, stdout: {}",
            std::str::from_utf8(&gcc_output.stderr).unwrap(),
            std::str::from_utf8(&gcc_output.stdout).unwrap()
        )
        .into()
    })
}
