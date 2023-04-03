use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!"); 
    let secret_number = rand::thread_rng().gen_range(1..=100); // 1..=100 is a range
    //println!("The secret number is {secret_number}");
    loop {
        println!("Please input your guess: ");
        let mut guess = String::new(); // mut means mutable
        io::stdin() // io is the standard input/output library
            .read_line(&mut guess) // & means reference
            .expect("Failed to read line."); // expect is a method that handles errors
        let guess: u32 = match guess.trim().parse(){ // trim removes whitespace
            Ok(num) => num, // Ok is an enum variant
            Err(_) => continue, // _ is a catchall value
        }; // parse is a method that converts a string to a number
        println!("You guessed: {guess}"); // {} is a placeholder
        match guess.cmp(&secret_number) { // match is a control flow operator
            Ordering::Less => println!("Too small!"), // Ordering is an enum
            Ordering::Greater => println!("Too big!"), // :: is an operator that accesses an enum variant
            Ordering::Equal => {
                println!("You win!");
                break; // break is a keyword that exits the loop
            } 
        }
    }
}
