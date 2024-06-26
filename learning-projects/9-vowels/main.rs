//  Given a string, determine how many of the characters are vowels and how many are consonants. Terminate the string when the input character encountered is non-alphabetic.

use std::io;

static CONSONANTS: [char; 21] = [
    'b', 'c', 'd', 'f', 'g', 'h', 'j', 
    'k', 'l', 'm', 'n', 'p', 'q', 'r',
    's', 't', 'v', 'w', 'x', 'y', 'z',
];

static VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];


fn main() {
    println!("Please enter a string of text. Only alphabets are allowed (no numbers or special characters).");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Invalid input");
    input = input.trim().to_string();
    let abc_str = terminate_at_non_abc(&input);
    if abc_str != input {
        println!("Your string was cut off at any non-alphabet characters. The string is now: {abc_str}.")
    }
    let (consonants, vowels) = count_consonants_and_vowels(&abc_str);
    println!("There are {consonants} consonants and {vowels} vowels in {abc_str}");
    
    
}

fn count_consonants_and_vowels(abc_str: &String) -> (usize, usize) {
    let (mut cons, mut vowels) = (0usize, 0usize);
    for c in abc_str.chars().collect::<Vec<char>>() {
        if CONSONANTS.contains(&c) { cons += 1 }
        else { vowels += 1 }
    }
    return (cons, vowels);
}

fn terminate_at_non_abc(input: &String) -> String {
    let chars: Vec<char> = input.chars().collect();
    let mut abc_str = String::new();
    for c in chars {
        if !VOWELS.contains(&c) && !CONSONANTS.contains(&c) {
            return abc_str;
        }
        abc_str.push(c);
    }
    abc_str
}

