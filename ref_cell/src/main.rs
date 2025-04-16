use ref_cell::*;

fn main() {
    // initialize the worker
    let logger = Worker::new(1);
    
    // initialize the tracker, with the max number of
    // called references as 10
    let track = Tracker::new(&logger, 10);
    
    let _a = logger.track_value.clone(); // |\
    let _a1 = logger.track_value.clone(); // | -> increase the Rc to 4 references
    let _a2 = logger.track_value.clone(); // |/
    
    // take a peek of how much we already used from our quota
    track.peek(&logger.track_value);
    
    // println!("{logger:?}");
    let _b = logger.track_value.clone(); // |\
    let _b1 = logger.track_value.clone(); // |  -> increase the Rc to 8 references
    let _b2 = logger.track_value.clone(); // | /
    let _b3 = logger.track_value.clone(); // |/
    
    // this will set the value and making a verification of
    // how much we already used of our quota
    track.set_value(&logger.track_value);
    
    let _c = logger.track_value.clone(); // | -> increase the Rc to 9 references
    
    // this will set the value and making a verification of
    // how much we already used of our quota
    track.set_value(&logger.track_value);
    
    let _c1 = logger.track_value.clone(); // | -> increase the Rc to 10 references, this will be the limit

    track.set_value(&logger.track_value);

    for (k, v) in logger.mapped_messages.into_inner() {
        println!("{:?}", (k, v));
    }
    println!("{:?}", logger.all_messages.into_inner());
}
/*
Error: Already failed code at commit d66749f

Previous output:
   Compiling ref_cell v0.1.0 (/jail/solutions/ref_cell)
   Compiling ref_cell_test v0.1.0 (/jail/tests/ref_cell_test)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.06s
     Running unittests src/main.rs (tests/ref_cell_test/target/debug/deps/ref_cell_test-32636d42269a0486)

running 3 tests
test tests::test_module_usage_hasmap ... ok
test tests::test_module ... FAILED
test tests::test_module_usage_vector ... ok

failures:

---- tests::test_module stdout ----

thread 'tests::test_module' panicked at src/main.rs:90:13:
assertion `left == right` failed
  left: "you are using up to 40% of your quota"
 right: "Info: you are using up to 40% of your quota"
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::test_module

test result: FAILED. 2 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass `--bin ref_cell_test`


*/
