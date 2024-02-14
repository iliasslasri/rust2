fn main() {
    for i in 0..=50 {
        print!("fibo({}) = ", i);
        let x = fibo(i);
        match x {
            Some(y) => println!("{}", y),
            None => {
                println!("Error overflow");
                break;
            }
        }
    }
}

fn fibo(n: u32) -> Option<u32> {
    if n == 0 {
        Some(0)
    } else if n == 1 {
        Some(1)
    } else {
        let mut a: u32 = 0;
        let mut b = 1;
        let mut c = 0;
        for _ in 0..n {
            let x = a.checked_add(b);
            match x {
                Some(y) => c = y,
                None => return None,
            }
            a = b;
            b = c;
        }
        Some(c)
    }
}
