use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let mut attempts: u32 = 0;
    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        attempts += 1;

        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("The guess must to be number between 1 and 100.");
                continue;
            }
        };

        if !(1..=100).contains(&guess) {
            println!("The secret number will be between 1 and 100.");
            continue;
        }

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                println!("Number of attempts: {}", attempts);
                break;
            }
        }
    }
}
