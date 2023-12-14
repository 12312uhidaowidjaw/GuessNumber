use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number between 0 and 100");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess. Tpye Ctrl+C to exit");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) if num >= 1 && num <= 100 => num,
            _ => {
                println!("This isn't a number between 0 and 100.");
                continue;   
            }
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

