use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    // number between 1 and 100 -- inclusive because of the "="
    // ".." tells us that this is the range between 1 and 100 -- syntax sugar
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Overwrite guess here -- it will not change after this
        // Note that the parse function is converting the string into the type specified on the left
        // u32 <--- &str
        let guess: u32 = match guess.trim().parse() {
            // lambda functions to check errors
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("you win");
                break;
            }
        }

        if guess < secret_number {
            println!("too small bro");
        } else if guess > secret_number {
            println!("too big bro");
        } else {
            println!("just right bro")
        }
    }
}
