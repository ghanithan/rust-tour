// Reference solution for If/Else Control Flow

fn main() {
    println!("=== If/Else Control Flow ===");
    
    // Part 1: Basic if/else
    let temperature = 75;
    
    // Fixed: Use comparison to create boolean condition
    if temperature > 70 {
        println!("It's warm outside!");
    } else {
        println!("It's cool outside!");
    }
    
    // Part 2: Multiple conditions
    let age = 25;
    
    if age >= 18 {
        println!("You can vote!");
    } else {
        println!("You cannot vote yet.");
    }
    
    // Part 3: else if chains
    let score = 85;
    
    if score >= 90 {
        println!("Grade: A");
    } else if score >= 80 {
        println!("Grade: B");
    } else if score >= 70 {
        println!("Grade: C");
    } else {
        println!("Grade: F");
    }
    
    // Part 4: if in let expressions
    let weather = "sunny";
    
    let activity = if weather == "sunny" {
        "go to the park"
    } else if weather == "rainy" {
        "read a book indoors"
    } else {
        "watch a movie"
    };
    
    println!("Today's activity: {}", activity);
    
    // Part 5: Complex boolean logic
    let has_license = true;
    let has_car = true;
    let has_gas = false;
    
    let can_drive = has_license && has_car && has_gas;
    
    if can_drive {
        println!("You can drive!");
    } else {
        println!("You cannot drive.");
        
        if !has_license {
            println!("Reason: No license");
        }
        if !has_car {
            println!("Reason: No car");
        }
        if !has_gas {
            println!("Reason: No gas");
        }
    }
    
    // Part 6: Nested if statements
    let time_of_day = 14;
    let is_weekend = false;
    
    if time_of_day >= 6 && time_of_day < 11 {
        println!("Breakfast time!");
        if is_weekend {
            println!("Try the weekend brunch special!");
        }
    } else if time_of_day >= 11 && time_of_day < 15 {
        println!("Lunch time!");
        if is_weekend {
            println!("Weekend lunch has extended hours!");
        }
    } else if time_of_day >= 17 && time_of_day < 22 {
        println!("Dinner time!");
    } else {
        println!("Kitchen is closed.");
    }
}