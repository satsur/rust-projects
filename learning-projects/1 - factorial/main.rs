use num_traits::PrimInt;
use num_traits::identities::Zero;
use std::ops::Mul;
use std::io;
use num_bigint::BigUint;
use std::time::Instant;

fn main() {
    println!("Please enter a number: ");
    let mut input = String::new();
    io::stdin()
    .read_line(&mut input)
    .expect("Input is not valid.");
    
    let n: u64 = input.trim().parse::<u64>().expect("Input is not a number.");
    let now = Instant::now();
    let factorial: Factorial = loop_fac(n);
    println!("\nCompleted in {0:.2?}", now.elapsed());
    
    println!("{n}! = {0}", factorial.value());
    println!("It has {0} digits", factorial.num_digits());
    println!("The least number of bits required to represent this number is: {0}", factorial.fewest_bits());
    println!("There are {0} trailing zeroes", factorial.trailing_zeros());

}

fn loop_fac(n: u64) -> Factorial {
    let mut fac: BigUint = BigUint::from(1u64);
    for i in 1..=n {
        fac = fac.mul(i);
    }
    
    Factorial::new(fac, &n)
}

struct Factorial {
    value: BigUint,
    num_digits: u64,
    fewest_bits: u64,
    trailing_zeros: u64,
}

impl Factorial {
    pub fn new(value: BigUint, n: &u64) -> Self{
        let num_digits = Self::calc_num_digits(&value);
        let fewest_bits = value.bits();
        let trailing_zeros = Self::calc_trailing_zeros(n);
        Factorial {
            value, // nth factorial value
            num_digits,
            fewest_bits,
            trailing_zeros,
        }
    }
    
    pub fn value(&self) -> &BigUint {
        &self.value
    }
    
    pub fn num_digits(&self) -> &u64 {
        &self.num_digits
    }
    
    pub fn fewest_bits(&self) -> &u64 {
        &self.fewest_bits
    }
    
    pub fn trailing_zeros(&self) -> &u64 {
        &self.trailing_zeros
    }
    
    // Utilizes de Polignac's formula to find the number of trailing zeros of a factorial 
    // (see https://en.wikipedia.org/wiki/Trailing_zero#Factorial)
    fn calc_trailing_zeros<T: PrimInt>(num: &T) -> T {
        let mut count = T::zero();
        let mut divisor = T::from(5).unwrap();
        let mut current = *num / divisor;
    
        while current > T::zero() {
            count = count + current;
            divisor = divisor * T::from(5).unwrap();
            current = *num / divisor;
        }
        count
    }
    
    fn calc_num_digits(n: &BigUint) -> u64 {
        if n.is_zero() {
            return 1;
        }
        
        let ten = BigUint::from(10u32);
        let mut num_digits: u64 = 0;
        let mut current = n.clone();
    
        while !current.is_zero() {
            num_digits += 1;
            current /= &ten;
        }

        num_digits
    }

}

/* 
Slow due to limited stack size (recursion piles up memory on the stack) and limits input to numbers <= 20
because I used u64 instead of a larger dynamic type like BigUint
Use loop instead?

fn recursive_fac(n: u64) -> u64 {
    if n <= 1 {
        return 1;
    } else if n == 2 {
        return 2;
    }
    n * fac(n - 1)
}
*/