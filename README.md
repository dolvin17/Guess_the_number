# guessing-game-program

This program uses common Rust concepts. The program will generate a random integer between 1 and 100, then prompt the player to enter a guess. After you enter an assumption, the program will indicate whether the assumption is too low or too high. If the guess is correct, the game will print a congratulatory message and end.

## Build
To build it using cargo, run:
```
$ cargo build
```

## Run
To compile and execute it using cargo, run:
```
$ cargo run
```

**Example:**
```
$ cargo run
   Compiling guessing_game v0.1.0 (/path/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 0.33s
     Running `target/debug/guessing_game`
Guess the number!
Please guess a number between 1 and 100:
5
You guessed: 5
Your guess was too low.
```

This program can be found explained in the [Rust book](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html).
