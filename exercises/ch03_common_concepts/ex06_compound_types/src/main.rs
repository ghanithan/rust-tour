// Exercise: Compound Data Types
//
// Rust has compound types that can group multiple values: tuples and arrays.
// Complete the missing code to work with these fundamental data structures.

fn main() {
    println!("=== Tuples ===");
    
    // Part 1: Basic tuple creation and access
    let person = ("Alice", 30, true); // (name, age, is_employed)
    
    // TODO: Extract values using tuple destructuring
    let (name, age, employed) = person;
    println!("Name: {}, Age: {}, Employed: {}", name, age, employed);
    
    // TODO: Access tuple elements by index
    let first_element = person.0;
    let second_element = /* access second element */;
    let third_element = /* access third element */;
    
    println!("First: {}, Second: {}, Third: {}", first_element, second_element, third_element);
    
    // Part 2: Tuple with different types
    let mixed_tuple = (42, 3.14, 'R', "Rust");
    
    // TODO: Destructure only the first two elements, ignore the rest
    let (number, pi, ..) = mixed_tuple;
    println!("Number: {}, Pi: {}", number, pi);
    
    // Part 3: Nested tuples
    let nested = ((1, 2), (3, 4));
    
    // TODO: Extract the first tuple from nested
    let first_tuple = /* extract first tuple */;
    println!("First nested tuple: {:?}", first_tuple);
    
    // TODO: Extract individual values from nested structure
    let ((a, b), (c, d)) = nested;
    println!("Values: {} {} {} {}", a, b, c, d);
    
    println!("\n=== Arrays ===");
    
    // Part 4: Array creation and access
    let numbers = [1, 2, 3, 4, 5];
    
    // TODO: Access array elements by index
    let first = numbers[0];
    let last = /* access last element */;
    println!("First: {}, Last: {}", first, last);
    
    // Part 5: Array with explicit type and size
    // TODO: Create an array of 5 integers, all initialized to 0
    let zeros: [i32; 5] = /* create array of zeros */;
    println!("Zeros array: {:?}", zeros);
    
    // TODO: Create an array with repeated values
    let threes = /* create array of 4 threes using [value; count] syntax */;
    println!("Threes array: {:?}", threes);
    
    // Part 6: Array methods and properties
    let fruits = ["apple", "banana", "cherry", "date"];
    
    // TODO: Get the length of the array
    let length = /* get array length */;
    println!("Array length: {}", length);
    
    // TODO: Iterate over array elements
    for (index, fruit) in fruits.iter().enumerate() {
        println!("Index {}: {}", index, fruit);
    }
    
    // Part 7: Mutable arrays
    let mut scores = [0; 5];
    
    // TODO: Modify array elements
    scores[0] = 85;
    scores[1] = /* assign a score */;
    scores[2] = /* assign a score */;
    
    println!("Scores: {:?}", scores);
    
    // Part 8: Array slicing
    let slice_array = [10, 20, 30, 40, 50];
    
    // TODO: Create a slice of the middle 3 elements
    let middle_slice = /* create slice [20, 30, 40] */;
    println!("Middle slice: {:?}", middle_slice);
    
    // Part 9: Combining tuples and arrays
    let student_grades = [
        ("Alice", [85, 90, 78]),
        ("Bob", [92, 88, 95]),
        ("Carol", [79, 85, 91])
    ];
    
    // TODO: Extract Alice's grades
    let (alice_name, alice_grades) = student_grades[0];
    println!("{}'s grades: {:?}", alice_name, alice_grades);
    
    // TODO: Calculate and print average for each student
    for (student, grades) in student_grades.iter() {
        let total: i32 = /* sum all grades */;
        let average = total as f64 / grades.len() as f64;
        println!("{}'s average: {:.1}", student, average);
    }
    
    // Part 10: Function returning tuple
    let rect_area = calculate_rectangle_area(5, 3);
    println!("Rectangle area: {}", rect_area);
    
    let (area, perimeter) = calculate_rectangle_stats(4, 6);
    println!("Area: {}, Perimeter: {}", area, perimeter);
}

// TODO: Implement function that calculates rectangle area
fn calculate_rectangle_area(width: i32, height: i32) -> i32 {
    /* return width * height */
}

// TODO: Implement function that returns both area and perimeter as tuple
fn calculate_rectangle_stats(width: i32, height: i32) -> (i32, i32) {
    let area = /* calculate area */;
    let perimeter = /* calculate perimeter */;
    /* return tuple of (area, perimeter) */
}