use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::io::Write;

fn main() {
    // Random number generator for current thread
    let secret = rand::thread_rng().gen_range(1..101);
    loop {
        print!("Guess: ");
        io::stdout().flush().expect("Failed to flush");

        // Create a mutable variable
        let mut guess = String::new();

        // Pass a mutable reference of guess as argument to read_line
        io::stdin()
            .read_line(&mut guess)
            // read_line returns a Result which can be Ok or Err
            // expect will crash on Err and will return the value held by Ok
            .expect("Failed to read line");

        // Trim, removes whitespaces, parse converts the string to u32
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // next loop iteration
                                // Underscore is a catch all value
        };

        // cmp can be called on anything that can be compared
        // which means any type implementing the Ord trait
        match guess.cmp(&secret) {
            Ordering::Equal => {
                println!("You guessed right");
                break; // out of the loop
            }
            Ordering::Greater => println!("Too big"),
            Ordering::Less => println!("Too small"),
        }
    }
}
