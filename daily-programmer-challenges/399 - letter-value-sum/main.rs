/* Challenege 399 (Easy) - Letter Value Sum
* https://www.reddit.com/r/dailyprogrammer/comments/onfehl/20210719_challenge_399_easy_letter_value_sum/
* https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=d706f06b9e2b8a3fd77e637004e7b892
*/

use std::collections::HashMap;
fn main() {
    let letter_values: HashMap<char, usize> = HashMap::from([
        ('a', 1), ('b', 2), ('c', 3), ('d', 4), ('e', 5), ('f', 6),
        ('g', 7), ('h', 8), ('i', 9), ('j', 10), ('k', 11), ('l', 12),
        ('m', 13), ('n', 14), ('o', 15), ('p', 16), ('q', 17), ('r', 18),
        ('s', 19), ('t', 20), ('u', 21), ('v', 22), ('w', 23), ('x', 24),
        ('y', 25), ('z', 26),
    ]);
    
    let word = String::from("cab");
    println!("{}", letter_sum(&word, &letter_values));
    
}


fn letter_sum(word: &String, values: &HashMap<char, usize>) -> usize {
    let mut sum = 0;
    for letter in word.chars().collect::<Vec<char>>() {
        sum += values.get(&letter).unwrap_or(&0);
    }
    sum
    
}