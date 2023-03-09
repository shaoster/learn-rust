use std::io;

/// Convert f to c, I guess?
fn main() {
    for line in io::stdin().lines() {
        let f: f64 = match line {
            Ok(line) => match line.trim().parse() {
                Ok(f) => f,
                Err(_) => continue,
            },
            Err(_) => continue,
        };
        let c = (f - 32.0) / 1.8;
        println!("{f} is {c} in Celsius");
    }
}
