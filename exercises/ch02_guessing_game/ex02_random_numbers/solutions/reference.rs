use rand::Rng;

fn main() {
    println!("Generating random numbers...");
    
    // Create a random number generator
    let mut rng = rand::thread_rng();
    
    // Generate and display multiple random numbers
    for i in 1..=3 {
        let random_number = rng.gen_range(1..=100);
        println!("Random number {}: {}", i, random_number);
    }
    
    println!("All numbers are between 1 and 100 (inclusive).");
}