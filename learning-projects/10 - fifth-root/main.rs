// Find the Fifth root of the sum of the squares of the first 100 ODD numbers only.
use std::ops::Range;
use std::iter::StepBy;

fn main() {
// First 100 odd numbers 
let odd_nums: StepBy<Range<usize>> = (1..200).step_by(2);
println!("There are {} numbers in the range", odd_nums.len()); // First 100 odd numbers
let mut sum: usize = 0;
for n in odd_nums {
    sum += usize::checked_pow(n, 2).expect("There was overflow when squaring one of the numbers.");
}

println!("The fifth root of the sum of the squares of the first 100 odd numbers is: {}", f64::powf(sum as f64, 1.0/5.0));
}
