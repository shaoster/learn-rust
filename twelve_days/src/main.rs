const ORDINALS: [&str; 12] = [
    "first",
    "second",
    "third",
    "fourth",
    "fifth",
    "sixth",
    "seventh",
    "eighth",
    "ninth",
    "tenth",
    "eleventh",
    "twelfth"
];

fn main() {
    for i in 0..12 {
        let ordinal = ORDINALS[i];
        println!("On the {ordinal} day of Christmas, my true love gave to me");
        for j in 0..=i {
            let day_number = i - j + 1;
            // Abstract base lyrics.
            println!("{day_number} thing(s) doing things");
        }
    }
}
