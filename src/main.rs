use std::io;

use rand::Rng;

fn get_random_number() -> i32 {
    rand::thread_rng().gen_range(1..=100)
}

fn main() {
    println!("Welcome to the EPIC number guesser! Guess a number, will you?");

    let target = get_random_number();

    println!("{}", target);

    loop {
        let mut buf = String::new();
        io::stdin()
            .read_line(&mut buf)
            .expect("error while reading");

        let input = buf.trim();

        let input = match input.parse::<i32>() {
            Ok(n) => n,
            _ => 0,
        };

        match input.cmp(&target) {
            std::cmp::Ordering::Equal => break,
            std::cmp::Ordering::Greater => println!("Too big!"),
            std::cmp::Ordering::Less => println!("Too small!"),
        };
    }

    println!("You found it! It was {} :)", target)
}
