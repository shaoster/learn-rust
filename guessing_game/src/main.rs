use std::{io, cmp::Ordering};
use rand::Rng;

/// How does the mutable buffer reference ride the stack?
fn get_line_from_stdin_and_eprint_bytes_read(buf: &mut String) -> () {
    let bytes_read = io::stdin()
        // buf is already a mutable reference. We should not (and cannot) re-borrow it.
        .read_line(buf)
        .expect("Failed to read line");
    eprintln!("Read {bytes_read} bytes.");
}

/// Pattern matching plays really nice with default value return...
fn print_lte(secret_number: u32, guess: u32) -> bool {
    // No nested returns, with type safety!
    match guess.cmp(&secret_number) {
        Ordering::Less => { println!("Too small!"); false },
        Ordering::Greater => { println!("Too big!"); false },
        // Strict typing here! If I forget a return value, the type-check catches it.
        Ordering::Equal => { println!("You win!"); true },
    } // no semi-colon here!
}

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    loop {
        /*
        No use after borrow, so the &mut prefix must be supplied in this layer.
        "References are immutable by default. Hence, you need to write &mut guess rather than &guess".
        */
        let mut guess = String::new();
        get_line_from_stdin_and_eprint_bytes_read(&mut guess);
        println!("You guessed: {guess}");

        // Shadowing is cool.
        let guess = guess.trim();
        let guess: u32 = match guess.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("[{guess}] doesn't look like a number...");
                // How does this work? I guess everything really is an expression?!?!?
                // https://stackoverflow.com/questions/70517235/how-does-match-compiles-with-continue-in-its-arms
                // TL;DR continue is actually expression of type `never`, which coerces to any type and thus
                // unifies with any type, like the u32 here.
                continue
            }
        };

        let guessed_right = print_lte(secret_number, guess);
        if guessed_right {
            break;
        }
    }
}