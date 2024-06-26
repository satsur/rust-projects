use rand::Rng;
use std::io;

fn main() {
    let (lower_limit, upper_limit_inclusive) = (1, 100);
    let mut score = 0u32;
    let mut play_again = true;
    
    while play_again {
        let mut rng = rand::thread_rng();
        let number = rng.gen_range(lower_limit..=upper_limit_inclusive);
        dbg!(number);
        
    
        let guess = loop {
            println!("Guess a number between 1 and 100!");
            let mut input = String::new();
            io::stdin()
            .read_line(&mut input)
            .expect("Invalid input");
            
            let num: u32 = input.trim().parse::<u32>().expect("Input is not an integer.");
            if num < lower_limit || num > upper_limit_inclusive {
                println!("Your guess was out of bounds! Please try again.")
            } else {
                break num;
            }
        };
        
        if guess == number {
            println!("You guessed it! The number was {number}");
            score += 1;
            println!("Your current score is: {score}");
        } else {
            println!("Incorrect. The number was {number}");
            println!("Your current score is: {score}");
        }
        
        println!("Would you like to play again? Yes (1) or No (2)? Enter a number.");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Invalid input");
        let input_num = input.trim().parse::<u32>().expect("Input is not an integer");
        if input_num == 2 { 
            play_again = false; 
        }
    }
    
}