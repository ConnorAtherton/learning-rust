//
// FizzBuzz program in Rust
//
fn main() {
    for n in 1..101 {
        match (n % 3, n % 5) {
            (0, 0) => println!("Fizzbuzz"),
            (0, _) => println!("Fizz"),
            (_, 0) => println!("Buzz"),
            _      => println!("{}", n),
        }
    }
}
