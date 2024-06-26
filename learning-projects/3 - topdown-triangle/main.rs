use std::io;

fn main() {

    println!("How many stars should be in the largest row of the triangle? Enter a number: ");
    let mut largest = String::new();
    io::stdin()
    .read_line(&mut largest)
    .expect("Invalid input");
    
    let largest_row: u32 = largest.trim().parse::<u32>().expect("Input is not an integer.");
    /* This is one way of doing it, but is more bulky
    for i in (1..=largest_row).rev() {
        let mut row = String::new();
        for _ in 1..=i {
            row.push('*');
        }
        println!("{row}");
    }
    */
    
    // As of Rust 1.16, we can do this: 
    for i in (1..=largest_row).rev() {
        println!("{}", "*".repeat(i as usize));
    }
    
    
}
