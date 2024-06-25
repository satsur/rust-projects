use std::io;

// Values obtained as of Tuesday June 25th, 2024
const DOLLARS_TO_RUPEES: f64 = 83.47; 
const RUPEES_TO_DOLLARS: f64 = 0.012;

fn main() {
    println!("Enter the currency ('USD' or 'INR'): ");
    let mut input_currency = String::new();
    io::stdin()
    .read_line(&mut input_currency)
    .expect("Input is not valid");
    input_currency = input_currency.trim().to_lowercase();
    
    println!("Enter the amount: ");
    let mut num = String::new();
    io::stdin()
    .read_line(&mut num)
    .expect("Input is not valid");

    let val: f64 = num.trim().parse::<f64>().expect("Input was not a valid number.");
    let converted: f64 = convert_currency(val, &input_currency).expect("Invalid currency");
    
    if input_currency == "usd" {
        println!("{val} USD = {converted} INR");
    } else {
        println!("{val} INR = {converted} USD");
    }
}

fn convert_currency(val:f64, from: &str) -> Option<f64> {
    match from {
        "usd" => Some(val * DOLLARS_TO_RUPEES),
        "inr" => Some(val * RUPEES_TO_DOLLARS),
        _ => None
    }
}