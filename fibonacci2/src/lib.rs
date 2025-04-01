pub fn fibonacci(n: u32) -> u32 {
    let mut fib = vec![0, 1];
    if n <= 1 {
        return n;
    }

    for i in 2..=n {
        let next_fib=fib[(i as usize) - 1] + fib[(i as usize) - 2];
        fib.push(next_fib);
    }
    fib[n as usize]
}
