use clap::Parser;

#[derive(Parser)]
#[command(author, version, about)]
/// Compute Fibonacci suite values
struct Args {
    /// The maximal number to print the fibo value of
    value: u32,

    /// Print intermediate values
    #[arg(short, long)]
    verbose: bool,

    /// The minimum number to compute
    #[arg(short = 'm', long)]
    min: Option<u32>,
}

fn main() {
    let args = Args::parse();
    let min_val = args.min.unwrap_or(0);
    let mut res = 0;

    for i in min_val..=args.value {
        if args.verbose {
            print!("fibo({}) = ", i);
        }
        let x = fibo(i);
        match x {
            Some(y) => {
                if args.verbose {
                    println!("{}", y);
                } else {
                    res = y
                }
            }
            None => {
                println!("Error overflow");
                break;
            }
        }
    }
    if !args.verbose {
        println!("fibo({}) = {}", args.value, res);
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
