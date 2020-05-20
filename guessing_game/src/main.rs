use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    //for testing: println!("the secret number is {}", secret_number);

    loop {

        println!("Please input your guess.");

        let mut guess = String::new(); //mut means mutable (variables are immutable by default)

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line"); //'expect' will crash it, 'match' (below) will handle the error

        //converting the guess from string to int
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, //ignores non-numerical inputs
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small!"),
            Ordering::Greater => println!("too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }

    }    
}
