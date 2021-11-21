// import libraries
use rand::Rng;
// how to import 2 modules from the same library
use std::{cmp::Ordering, io};

fn main() {
    // print a line
    println!("Guess the number game!");
    // generate a random number, store it into variable secret_number
    let secret_number = rand::thread_rng().gen_range(0, 101);
    // just print the variable secret_number
    println!("The secret number is: {}", secret_number);
    // infinite loop that will stop only with break
    loop {
        println!("Please input your guess");
        // guess is mutable here
        // declare variable guess which is a new string
        let mut guess = String::new();
        // input data from the terminal
        io::stdin()
            // here &mut guess is a mutable reference of guess
            .read_line(&mut guess)
            // this is a useful message we print if something is wrong
            .expect("Failed to read line");
        // ensure that the input data is actually a u32 type without space after the input
        let guess: u32 = match guess.trim().parse() {
            // this is a result type that can be either success containing a value
            // or an error containing an error value
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guess: {}", guess);

        // check what guess is compared to secret number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small!"),
            Ordering::Greater => println!("too big!"),
            Ordering::Equal => {
                println!("you win!");
                // break the program if you win, otherwise infinite loop
                break;
            }
        }
    }
}
