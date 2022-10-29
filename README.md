# Guess_the_number
Here’s how it works: the program will generate a random int between 1 and 100. It will then prompt the player to enter a guess. After a guess is entered, the program will indicate whether the guess is too low or too high. If the guess is correct, the game will print a congratulatory message and exit.
# Build

To build it using cargo, run:

$ cargo build
Run

To compile and execute it using cargo, run:

$ cargo run
Example:

$ cargo run
   Compiling guessing_game v0.1.0 (/path/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 0.33s
     Running `target/debug/guessing_game`
Guess the number!
Please guess a number between 1 and 100:
5
You guessed: 5
Your guess was too low.
