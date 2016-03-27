// Rust automatically installs a small set of code code
// into every program.
//
// NOTE: See https://doc.rust-lang.org/std/prelude/

// This will also effectively do `use rand` internally
extern crate rand;

// Include libraries. Libraries can be accessed
// through a top level namespace.
use std::io;
use std::cmp::Ordering;
use rand::Rng;

// NOTE: If we don't specify a return type in a function, Rust
// implicitly assumes it to be (), an empty tuple.
fn main() {
    println!("Guess the number!");

    // immutable because we haven't specified **mut**
    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Input a number:");

        // Rust bindings are immutable by default. Adding the **mut**
        // keyword makes a binding mutable.
        //
        // ::new() is like a static method of the String type.
        let mut guess = String::new();

        // This created a new reference to the same underlying
        // string data created above. References are immutable by
        // default.
        //
        // Rust has type inference and will infer guess to be a string type above
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        // Shadow the previous guess with the correct type. We are annotating
        // the binding with it's type -- in this case, an unisgned 32bit integer.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            // This is an enumeration. Will look something like this
            //
            // enum Ordering (
            //  Less,
            //  Greater,
            //  Equal,
            // );
            //
            Ordering::Less    => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal   => {
                println!("You win!");
                break;
            }
        }
    }
}
