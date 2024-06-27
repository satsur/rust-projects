// Get all possible permutations of a four letter word
use itertools::Itertools;
use std::io;
use std::time::Instant;

fn main() {
    println!("Enter a word: ");
    let mut word = String::new();
    io::stdin().read_line(&mut word).expect("Invalid input");
    let now = Instant::now();
    let permutations = get_combinations(&word.trim().to_string());
    println!("Completed in {:.2?}", now.elapsed());
    println!("{} permutations of {}", permutations.len(), word);
    for perm in permutations {
        println!("{}", perm);
    }
}

// I love iterators so much
fn get_combinations(word: &String) -> Vec<String> {
    word.chars()                            // Returns Chars instance which is an Iterator
    .permutations(word.len())               // Iterating through each of these Vec<char> (permutations)
    .map(|v| v.iter().collect::<String>())  // FnMut closure that, for each Vec<char> permutation, collects those characters and puts them in a String
    .collect()                              // And now, each of these words are collected into a Vec<String> resulting in our final list of permutations
}   