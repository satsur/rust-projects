use std::io;

fn main() {

    println!("Please enter a number: ");
    let mut input = String::new();
    io::stdin()
    .read_line(&mut input)
    .expect("Input is not valid.");
    
    let n: u64 = input.trim().parse::<u64>().expect("Input is not a number.");
    println!("fib({n}) = {}", fib(n))
}

fn fib(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }
    if n <= 2 {
        return 1;
    }
    
    fib(n - 1) + fib(n - 2)
}