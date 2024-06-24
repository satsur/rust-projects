/* Challenege 400 (Intermediate) - Practical Numbers
* https://www.reddit.com/r/dailyprogrammer/comments/13m4bz1/20230519_challenge_400_intermediate_practical/
* Practical number: A number for which ever number below it can be expressed as a sum of its factors
* -- STEPS TO EVALUATE IF A NUMBER IS PRACTICAL --
* 1. Find all factors of the given number (n)
*    - Divide the given number by each up to and including the root of n
* 2. Interate through every number from 1 to n-1, check if it is a sum of the factors
* 3. If so, it is a practical number
* 
* https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=f7f2f39078a1ea819510d8b1de73cbff
*/


use itertools::Itertools;

#[cfg(test)]
mod tests {
    // use super::*;
    #[test]
    fn works() {
        let prac_nums: Vec<u32> = 
                            vec![1,2,4,6,8,12,16,18,20,24,28,30,32,36,40,42,48,54,
                                 56,60,64,66,72,78,80,84,88,90,96,100,104,108,112,
                                 120,126,128,132,140,144,150,156,160,162,168,176,
                                 180,192,196,198,200,204,208,210,216,220,224,228,
                                 234,240,252];
            let mut generated_prac_nums: Vec<u32> = Vec::new();
            for i in 1..=*prac_nums.last().unwrap() {
                if is_practical(&i) {
                    generated_prac_nums.push(i);
                }
            }
            println!("Generated: {:?}", generated_prac_nums);
            println!("Actual:    {:?}", prac_nums);
            assert_eq!(generated_prac_nums, prac_nums);
    }
}
    
fn main() {
    let mut sum: u64 = 0u64;
    
    for i in 1..=10_000 {
        if is_practical(&i) {
            sum += i as u64;
        }
    }
    
    println!("The sum of all practical numbers from 1 to 10,000 is {sum}");
}

// https://en.wikipedia.org/wiki/Practical_number#Properties
fn is_practical(n: &u32) -> bool {
    // 1 and 2 are practical numbers
    if *n == 1 || *n == 2 {
        return true;
    } 
    // 0 and 3 are not practical numbers
    if *n == 0 || *n == 3 {
        return false;
    } 
    // All practical numbers MUST be even
    if n % 2 != 0 {
        return false;
    } 
    // Practical numbers must be divisible by 4 OR 6 (or both)
    if n % 4 != 0 && n % 6 != 0 {
        return false
    } 
    // All powers of 2 are practical numbers (see https://graphics.stanford.edu/~seander/bithacks.html#DetermineIfPowerOf2)
    if n & (n - 1) == 0 {
        return true;
    }
    // Practical numbers cannot be deficient numbers (https://en.wikipedia.org/wiki/Practical_number#Characterization_of_practical_numbers)
    let factors: Vec<u32> = factors(n);
    if factors.iter().sum::<u32>() < 2 * *n {
        return false;
    }
    
    // Credit to Martin R from https://codereview.stackexchange.com/questions/158142/practical-number-algorithm for this algorithm
    let mut is_sum_of_factors = vec![false; *n as usize];
    is_sum_of_factors[0] = true;
    let mut last = 0; // Index of last `true` element.

    // Loop through all divisors of n excluding n
    for &d in factors.iter().take_while(|&d| d < n) {
        // For all i which are a sum of smaller divisors (in reverse order, so that
        // we can simply update the array):
        for i in (0..=std::cmp::min(last, n - 1 - d)).rev() {
            if is_sum_of_factors[i as usize] {
                is_sum_of_factors[(i + d) as usize] = true;
                if i + d > last {
                    last = i + d;
                }
            }
        }
    }

    // n is "practical" if is_sum_of_divisors[i] == true for all i = 0...n-1:
    !is_sum_of_factors.iter().take(*n as usize).any(|&x| !x)
}

fn factors(n: &u32) -> Vec<u32> {
    let limit: u32 = (*n as f32).sqrt().floor() as u32;
    let mut factors: Vec<u32> = Vec::new();
    
    for i in 1..limit+1 {
        if factors.contains(&i) {
            println!("Skipping {}...", &i);
            continue;
        }
        if n % i == 0 {
            factors.push(i);
            if *n / i != i {
                factors.push(*n / i);
            }
        }
    }
    factors.sort();
    return factors;
}

/* ----------------------------------- OLD (INEFFICIENT) METHOD ----------------------------------- 
This method worked but was quite inefficient and repetitive as it required the same subsets to be recreated for
each number from 1 to n-1. I could've made it more efficient, but I found the above solution instead.

// https://en.wikipedia.org/wiki/Practical_number#Properties
fn is_practical(n: &u32) -> bool {
    // 1 and 2 are practical numbers
    if *n == 1 || *n == 2 {
        return true;
    } 
    // 0 and 3 are not practical numbers
    if *n == 0 || *n == 3 {
        return false;
    } 
    // All practical numbers MUST be even
    if n % 2 != 0 {
        return false;
    } 
    // Practical numbers must be divisible by 4 OR 6 (or both)
    if n % 4 != 0 && n % 6 != 0 {
        return false
    } 
    // All powers of 2 are practical numbers (see https://graphics.stanford.edu/~seander/bithacks.html#DetermineIfPowerOf2)
    if n & (n - 1) == 0 {
        return true;
    }
    // Practical numbers cannot be deficient numbers (https://en.wikipedia.org/wiki/Practical_number#Characterization_of_practical_numbers)
    let factors: Vec<u32> = factors(n);
    if factors.iter().sum::<u32>() < 2 * *n {
        return false;
    }
    
    // Iterate through all numbers from 1 to n-1, check if all can be expressed as a sum of the divisors of n
    for i in 1..*n {
        if !can_be_expressed_as_sum(&i, &factors) {
            return false;
        }
    }
    return true;
}

// Sum up the factor subsets, check if n equals one of those sums
fn can_be_expressed_as_sum(n: &u32, factors: &Vec<u32>) -> bool {
    if factors.contains(n) {
        return true;
    }
    for subset_vec in get_all_subsets_of_nums(&factors[0..factors.len()]) {
        let subset_sum: u32 = subset_vec.iter().fold(0u32, |sum, i| sum + (*i as u32));
        if *n == subset_sum {
            return true;
        }
    }
    return false;
}

// Generate all possible combinations of numbers in nums from length 2 to nums.len()-1
fn get_all_subsets_of_nums(nums: &[u32]) -> Vec<Vec<u32>> {
    let len = nums.len();
    let mut subsets: Vec<Vec<u32>> = Vec::new();
    for i in 2..len {
        let combinations_for_len: Vec<Vec<u32>> = nums
        .iter()
        .combinations_with_replacement(i)
        .map(|combo| combo.into_iter().copied().collect())
        .collect();
        subsets.extend(combinations_for_len);
    }
    subsets
}

fn factors(n: &u32) -> Vec<u32> {
    let limit: u32 = (*n as f32).sqrt().floor() as u32;
    let mut factors: Vec<u32> = Vec::new();
    
    for i in 1..limit+1 {
        if factors.contains(&i) {
            println!("Skipping {}...", &i);
            continue;
        }
        if n % i == 0 {
            factors.push(i);
            if *n / i != i {
                factors.push(*n / i);
            }
        }
    }
    factors.sort();
    return factors;
}
*/
