use std::io;
fn main() {
    println!("Enter a number to stop at: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Invalid input");
    let limit: u64 = input.trim().parse::<u64>().expect("Input is not an integer.");
    
    println!("{:?}", fib(limit));
}

fn fib(limit: u64) -> Vec<u64> {
    let mut a = 0u64;
    let mut b = 1u64;
    let mut fib_vec = vec![a, b];
    
    let mut fib = a + b;
    while fib <= limit {
        fib_vec.push(fib);
        a = b;
        b = fib;
        fib = a + b;
    }
    
    fib_vec
}