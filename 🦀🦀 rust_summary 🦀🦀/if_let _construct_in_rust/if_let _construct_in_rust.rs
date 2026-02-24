// =============================================================================
// IF LET - Concise Control Flow
// =============================================================================
// if let = "check if this pattern matches, and if so, do something"
// It's a shorter way to write match when you only care about ONE case
// =============================================================================

// -----------------------------------------------------------------------------
// EXAMPLE 1: Basic if let with Option
// -----------------------------------------------------------------------------

fn main() {
    let config_max: Option<i32> = Some(100);
    
    // LONG WAY with match:
    match config_max {
        Some(max) => println!("Max is: {}", max),
        None => (),  // Do nothing (boring!)
    }
    
    // SHORT WAY with if let:
    if let Some(max) = config_max {
        println!("Max is: {}", max);  // Only runs if it's Some
    }  // No need to handle None case!
    
    // This says: "if config_max matches the pattern Some(max), 
    // then assign the inner value to 'max' and run the block"
}

// -----------------------------------------------------------------------------
// EXAMPLE 2: if let with else (when you want to handle the other case too)
// -----------------------------------------------------------------------------

fn main() {
    let maybe_number: Option<i32> = None;
    
    if let Some(x) = maybe_number {
        println!("Got: {}", x);
    } else {
        println!("Got nothing");  // Runs when it's None
    }
    
    // Same as this match:
    match maybe_number {
        Some(x) => println!("Got: {}", x),
        None => println!("Got nothing"),
    }
}

// -----------------------------------------------------------------------------
// EXAMPLE 3: if let with enums
// -----------------------------------------------------------------------------

#[derive(Debug)]
enum Rarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(Rarity),  // Quarter has a rarity
}

fn main() {
    let coin = Coin::Quarter(Rarity::Rare);
    
    // Check if it's a Quarter and get the rarity
    if let Coin::Quarter(rarity) = coin {
        println!("This quarter is {:?}!", rarity);
    } else {
        println!("Not a quarter");
    }
    
    // Try with a different coin
    let penny = Coin::Penny;
    
    if let Coin::Quarter(rarity) = penny {
        println!("This quarter is {:?}!", rarity);
    } else {
        println!("Not a quarter");  // This will run
    }
}

// -----------------------------------------------------------------------------
// EXAMPLE 4: if let with multiple conditions
// -----------------------------------------------------------------------------

fn main() {
    let numbers = Some((5, 10));  // Option with a tuple inside
    
    // Check if it's Some and the first value is 5
    if let Some((x, y)) = numbers {
        if x == 5 {
            println!("First number is 5, second is {}", y);
        }
    }
    
    // Or combine with && (and)
    if let Some((x, y)) = numbers && x == 5 {
        println!("First is 5, second is {}", y);
    }
}

// -----------------------------------------------------------------------------
// EXAMPLE 5: Real-world use - checking if a value exists
// -----------------------------------------------------------------------------

fn get_user_name(id: u32) -> Option<String> {
    // Pretend this looks up a user
    if id == 1 {
        Some(String::from("Alice"))
    } else {
        None
    }
}

fn main() {
    let user_id = 1;
    
    // Only print if user exists
    if let Some(name) = get_user_name(user_id) {
        println!("User found: {}", name);
    } else {
        println!("User not found");
    }
    
    // With multiple options
    let user_id = 2;
    
    if let Some(name) = get_user_name(user_id) {
        println!("User: {}", name);
    } else {
        println!("No user with id {}", user_id);
    }
}

// -----------------------------------------------------------------------------
// EXAMPLE 6: while let (keep doing while pattern matches)
// -----------------------------------------------------------------------------

fn main() {
    let mut stack = vec![1, 2, 3];
    
    // Keep popping while we get Some value
    while let Some(top) = stack.pop() {
        println!("Popped: {}", top);
    }
    // Prints: 3, 2, 1
}

// -----------------------------------------------------------------------------
// SUMMARY: if let vs match
// -----------------------------------------------------------------------------
/*
USE IF LET WHEN:
- You only care about ONE pattern
- Other cases can be ignored or handled in else
- You want concise code

USE MATCH WHEN:
- You need to handle MULTIPLE patterns
- You MUST handle all cases (exhaustive checking)
- You have complex logic for each case

SYNTAX:
if let PATTERN = VALUE {
    // runs if VALUE matches PATTERN
} else {
    // runs if no match (optional)
}

COMMON PATTERNS:
if let Some(x) = option_value      // Option with value
if let Ok(x) = result_value         // Result success
if let Coin::Quarter(r) = coin      // Enum variant with data
if let (x, y) = tuple && x > 5      // Tuple with condition
*/