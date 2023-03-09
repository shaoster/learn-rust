use std::io;

fn fib(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fib(n-1) + fib(n-2)
    }
}

fn main() {
    for line in io::stdin().lines() {
        let n: u32 = match line {
            Ok(line) => match line.trim().parse() {
                Ok(n) => n,
                Err(_) => continue,
            },
            Err(_) => continue,
        };
        let result = fib(n);
        println!("{result}");
    }
}
