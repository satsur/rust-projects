/* Challenege 397 (Easy) - Roman Numeral Comparison
* https://www.reddit.com/r/dailyprogrammer/comments/oe9qnb/20210705_challenge_397_easy_roman_numeral/
* https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=773977e69598723304bb4b50ad872fcb
*/

use std::collections::HashMap;

fn decipher(numerals: &String, values: &HashMap<char, usize>) -> usize {
    let prev = 0;
    let sum = 0;
    for letter in numerals.chars().collect::<Vec<char>>() {
        let value = values.get(letter);
        if value < prev {
            sum += prev - value;
            prev = 0;
        } else {
            sum += prev;
            prev = value;
        }
    }
}
