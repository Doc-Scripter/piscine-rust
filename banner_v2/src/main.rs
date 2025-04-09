use banner_v2::*;
use std::collections::HashMap;

fn main() {
    let mut handler = FlagsHandler { flags: HashMap::new() };

    let d = Flag::opt_flag("division", "divides the values, formula (a / b)");
    let r = Flag::opt_flag(
        "remainder",
        "remainder of the division between two values, formula (a % b)",
    );

    handler.add_flag(d, div);
    handler.add_flag(r, rem);

    println!("{:?}", handler.exec_func("-d", &["1.0", "2.0"]));

    println!("{:?}", handler.exec_func("-r", &["2.0", "2.0"]));

    println!("{:?}", handler.exec_func("--division", &["a", "2.0"]));

    println!("{:?}", handler.exec_func("--remainder", &["2.0", "fd"]));
}
/*
   Compiling banner v0.1.0 (/jail/solutions/banner)
   Compiling banner_test v0.1.0 (/jail/tests/banner_test)
warning: static `HANDLER` is never used
 --> src/lib.rs:4:8
  |
4 | static HANDLER: LazyLock<FlagsHandler> = LazyLock::new(|| {
  |        ^^^^^^^
  |
  = note: `#[warn(dead_code)]` on by default

warning: `banner_test` (lib) generated 1 warning
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.05s
     Running unittests src/lib.rs (tests/banner_test/target/debug/deps/banner_test-ca8779e8b6b7c196)

running 2 tests
test test_edge_cases ... FAILED
test test_simple ... FAILED

failures:

---- test_edge_cases stdout ----

thread 'test_edge_cases' panicked at src/lib.rs:49:9:
assertion `left == right` failed
  left: Err("inf")
 right: Err("invalid float literal")
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

---- test_simple stdout ----

thread 'test_simple' panicked at src/lib.rs:24:9:
assertion `left == right` failed
  left: Ok("invalid float literal")
 right: Ok("0.5")


failures:
    test_edge_cases
    test_simple

test result: FAILED. 0 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass `--lib`
' 

*/