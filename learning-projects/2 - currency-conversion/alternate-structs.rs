// An alternate (and more flexible) implementation of the currency conversion program
// using structs and HashMap to quickly look up different conversion rates

use std::io;
use std::collections::HashMap;

pub struct Currency {
    name: String,
    from_usd_rate: f64,
    to_usd_rate: f64,
}

impl Currency {
    pub fn new(name: String, from_usd_rate: f64, to_usd_rate: f64) -> Self {
        Currency {
            name,
            from_usd_rate,
            to_usd_rate,
        }
    }
    
    pub fn name(&self) -> &String {
        &self.name
    }
    
    fn from_usd(&self, val: &f64) -> f64 {
        *val * self.from_usd_rate
    }
    
    fn to_usd(&self, val: &f64) -> f64{
        *val * self.to_usd_rate
    }
    
    pub fn to_currency(&self, other: &Currency, val: &f64) -> f64 {
        if *other.name() == String::from("USD") {
            return self.to_usd(&val)
        } 
        let self_to_usd = self.to_usd(&val);
        other.from_usd(&self_to_usd)
    }
}

impl PartialEq for Currency {
    fn eq(&self, other: &Self) -> bool {
        self.name() == other.name()
    }
}

fn main() {
    let currency_list: HashMap<String, Currency> = HashMap::from([
        (String::from("USD"), Currency::new(String::from("USD"), 1.0, 1.0)),
        (String::from("INR"), Currency::new(String::from("INR"), 83.47, 0.012)),
        (String::from("EUR"), Currency::new(String::from("EUR"), 0.93, 1.07)),
        (String::from("GBP"), Currency::new(String::from("GBP"), 0.79, 1.27)),
    ]);
    let from_currency_input = get_input("Enter the currency to convert from: ".to_string()).trim().to_string().to_uppercase();
    let from_currency = currency_list.get(&from_currency_input).expect("Invalid currency!");
    
    let to_currency_input = get_input("Enter the currency to convert to: ".to_string()).trim().to_string().to_uppercase();
    let to_currency = currency_list.get(&to_currency_input).expect("Invalid currency!");
    
    let amount: f64 = get_input("Enter the amount: ".to_string()).trim().parse::<f64>().expect("Invalid numerical input!");
    let converted_amount: f64 = from_currency.to_currency(&to_currency, &amount);
    println!("{amount} {} = {converted_amount} {}", from_currency.name(), to_currency.name());
    
}

fn get_input(prompt: String) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin()
    .read_line(&mut input)
    .expect("Invalid input");
    
    input
}