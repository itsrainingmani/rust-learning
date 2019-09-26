use rand::Rng;
use std::cmp::Ordering;
use std::io;

// vars and refs are immutable by default

fn main() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    // infinite loop
    loop {
        println!("Please input your guess: ");

        // mutable variable bound to a new, empty instance of String
        let mut guess = String::new();

        // pass guess as a mutable reference to read_line so that stdin
        // can place what is typed into std input into the variable
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        // expect panics if the value of the Result enum is Err

        // Shadow the previous value of guess with a new one
        // Useful when you want to convert the a value from one type to another
        // without having to create a new variable
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input");
                continue;
            }
        };

        println!("You guessed: {}", guess);

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
