use std::io;

/// How does the mutable buffer reference ride the stack?
fn get_line_from_stdin_and_eprint_bytes_read(buf: &mut String) -> () {
    let bytes_read = io::stdin()
        // buf is already a mutable reference. We should not (and cannot) re-borrow it.
        .read_line(buf)
        .expect("Failed to read line");
    eprintln!("Read {bytes_read} bytes.");
}

fn main() {
    println!("Guess the number!");
    let mut guess = String::new();
    /*
    No use after borrow, so the &mut prefix must be supplied in this layer.
    "References are immutable by default. Hence, you need to write &mut guess rather than &guess".
    */
    get_line_from_stdin_and_eprint_bytes_read(&mut guess);
    println!("You guessed: {guess}");
}
