use std::time::Instant;

fn main() {
    let start = Instant::now();
    for i in 0..=42 {
        println!("fibo({}) = {}", i, fibo(i));
    }
    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
}

fn fibo(n: u32) -> u32 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fibo(n - 1) + fibo(n - 2)
    }
}
