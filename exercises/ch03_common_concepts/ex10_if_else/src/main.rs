// Exercise: If/Else Control Flow
//
// Master conditional logic in Rust using if expressions.
// Fix the compilation errors and complete the missing logic.

fn main() {
    println!("=== If/Else Control Flow ===");
    
    // Part 1: Basic if/else
    let temperature = 75;
    
    // TODO: Fix this if statement - condition must be a boolean
    if temperature {
        println!("It's warm outside!");
    } else {
        println!("It's cool outside!");
    }
    
    // Part 2: Multiple conditions
    let age = 25;
    
    // TODO: Complete this logic for voting eligibility
    if /* age condition */ {
        println!("You can vote!");
    } else {
        println!("You cannot vote yet.");
    }
    
    // Part 3: else if chains
    let score = 85;
    
    // TODO: Complete the grade assignment logic
    if score >= 90 {
        println!("Grade: A");
    } else if /* condition for B grade (80-89) */ {
        println!("Grade: B");
    } else if /* condition for C grade (70-79) */ {
        println!("Grade: C");
    } else {
        println!("Grade: F");
    }
    
    // Part 4: if in let expressions
    let weather = "sunny";
    
    // TODO: Use if expression to assign activity based on weather
    let activity = if weather == "sunny" {
        /* return activity for sunny weather */
    } else if weather == "rainy" {
        /* return activity for rainy weather */
    } else {
        /* return default activity */
    };
    
    println!("Today's activity: {}", activity);
    
    // Part 5: Complex boolean logic
    let has_license = true;
    let has_car = true;
    let has_gas = false;
    
    // TODO: Complete the logic - can drive if has license AND car AND gas
    let can_drive = /* complete boolean expression */;
    
    if can_drive {
        println!("You can drive!");
    } else {
        println!("You cannot drive.");
        
        // TODO: Add specific reasons why they can't drive
        if /* check if no license */ {
            println!("Reason: No license");
        }
        if /* check if no car */ {
            println!("Reason: No car");
        }
        if /* check if no gas */ {
            println!("Reason: No gas");
        }
    }
    
    // Part 6: Nested if statements
    let time_of_day = 14; // 24-hour format
    let is_weekend = false;
    
    // TODO: Complete nested logic for restaurant recommendations
    if time_of_day >= 6 && time_of_day < 11 {
        println!("Breakfast time!");
        if is_weekend {
            println!("Try the weekend brunch special!");
        }
    } else if /* lunch time condition (11-15) */ {
        println!("Lunch time!");
        if /* weekend condition */ {
            println!("Weekend lunch has extended hours!");
        }
    } else if /* dinner time condition (17-22) */ {
        println!("Dinner time!");
    } else {
        println!("Kitchen is closed.");
    }
}