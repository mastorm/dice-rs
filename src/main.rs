use std::io;

fn get_random_number() -> i8 {
    let target: i8 = rand::random();
    target.abs().into()
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

        let num_guess = match input.parse::<i32>() {
            Ok(n) => n,
            _ => 0,
        };

        if num_guess == target.into() {
            break;
        } else {
            println!("Ah bummer, thats wrong! :( Try again!")
        }
    }

    println!("You found it! It was {} :)", target)
}
