// Reference solution for Basic Functions

fn main() {
    println!("=== Basic Functions ===");
    
    greet();
    say_hello_to("Alice");
    
    let sum = add_numbers(5, 3);
    println!("Sum: {}", sum);
    
    let product = multiply(4, 7);
    println!("Product: {}", product);
    
    let is_even = check_even(10);
    println!("Is 10 even? {}", is_even);
}

fn greet() {
    println!("Hello, world!");
}

fn say_hello_to(name: &str) {
    println!("Hello, {}!", name);
}

fn add_numbers(a: i32, b: i32) -> i32 {
    a + b
}

fn multiply(x: i32, y: i32) -> i32 {
    x * y
}

fn check_even(number: i32) -> bool {
    number % 2 == 0
}