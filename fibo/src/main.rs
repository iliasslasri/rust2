fn main() {
    for i in 0..=50 {
        println!("fibo({}) = {}", i, fibo(i));
    }
}
fn fibo(n: u32) -> u32 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        let mut a: u32= 0;
        let mut b = 1;
        let mut c= 0;
        for _ in 0..n {
            // saturated addition
            c = a.saturating_add(b);
            a = b;
            b = c;
        }
        c
    }
}
