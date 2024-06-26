use rand::Rng;

fn first() {
    println!("First!");
}

fn second() {
    println!("Second");
}

fn third() {
    println!("Third!");
}

fn main() {
    let mut rand = rand::thread_rng();
    match rand.gen_range(1..=3) {
        1 => first(),
        2 => second(),
        3 => third(),
        _ => println!("hmm...")
    }
}