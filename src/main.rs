use std::io;//to user imput
use rand::Rng; // defines methods that random number generators implement
use std::cmp::Ordering;//Ordering type is another enum and has the variants Less, Greater, and Equal.

fn main () {
    	println!("Guess the number!");
    let secret_number = rand::thread_rng()
    	.gen_range(1..=100);
    loop {
    	println!("Please input your guess.");
    let mut guess = String::new(); //was created a mutable var that is currently bound to a new, empty instance of a String.
    io::stdin()
    	.read_line(&mut guess) //calls the read_line method, take whatever the user types.
	.expect("Failed to read line"); // will cause the program to crash and display the message
    let guess: u32 = match guess// Rust allows us to shadow the previous value of guess with a new one.
    	.trim()// this method on string, eliminate any whitespace at the beginning and end.
    	.parse() {//this method on string, converts a string to another type.
	Ok(num) => num,
	Err(_) => continue, // The underscore _, is a catch all value. 
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
