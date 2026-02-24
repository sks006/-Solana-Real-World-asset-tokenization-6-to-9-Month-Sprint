// =============================================================================
// RUST MATCH - SIMPLE EXPLANATION
// =============================================================================
// Match = "check this value and do something for each possibility"

// -----------------------------------------------------------------------------
// EXAMPLE 1: Basic match (like a switch statement)
// -----------------------------------------------------------------------------

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {              // Check which coin we have
        Coin::Penny => {       // If Penny:
            println!("Penny!");
            1                  // Return 1
        }
        Coin::Nickel => 5,     // If Nickel: return 5
        Coin::Dime => 10,       // If Dime: return 10
        Coin::Quarter => 25,    // If Quarter: return 25
    }
}

fn main() {
    let my_coin = Coin::Dime;
    println!("Value: {} cents", value_in_cents(my_coin));
}

// -----------------------------------------------------------------------------
// EXAMPLE 2: Match with data (getting values out)
// -----------------------------------------------------------------------------

enum IpAddr {
    V4(u8, u8, u8, u8),  // Holds 4 numbers
    V6(String),           // Holds 1 string
}

fn print_ip(ip: IpAddr) {
    match ip {
        IpAddr::V4(a, b, c, d) => {           // Get the 4 numbers out
            println!("IPv4: {}.{}.{}.{}", a, b, c, d);
        }
        IpAddr::V6(address) => {               // Get the string out
            println!("IPv6: {}", address);
        }
    }
}

// -----------------------------------------------------------------------------
// EXAMPLE 3: Match with Option (most common use)
// -----------------------------------------------------------------------------

fn main() {
    let maybe_number = Some(5);
    let no_number: Option<i32> = None;
    
    // Handle both cases
    match maybe_number {
        Some(x) => println!("Got: {}", x),  // If there's a value
        None => println!("Got nothing"),     // If there's no value
    }
    
    match no_number {
        Some(x) => println!("Got: {}", x),
        None => println!("Got nothing"),
    }
}

// -----------------------------------------------------------------------------
// EXAMPLE 4: Match with default case (_)
// -----------------------------------------------------------------------------

fn describe_number(n: i32) -> &'static str {
    match n {
        1 => "one",
        2 => "two",
        3 => "three",
        _ => "something else",  // _ = "anything not listed above"
    }
}

// -----------------------------------------------------------------------------
// EXAMPLE 5: Match with multiple patterns (|) and ranges (..=)
// -----------------------------------------------------------------------------

fn classify_number(n: i32) -> &'static str {
    match n {
        0 => "zero",
        1 | 2 | 3 => "small",        // | means OR
        4..=10 => "medium",           // ..= means range (4 to 10)
        _ if n < 0 => "negative",     // guard (extra condition)
        _ => "large",
    }
}

// -----------------------------------------------------------------------------
// SUMMARY - WHAT TO REMEMBER
// -----------------------------------------------------------------------------
/*
match value {
    Pattern1 => result1,           // Single line result
    Pattern2 => {                   // Multi-line result
        do_something();
        result2
    }
    Pattern3 | Pattern4 => result,  // Multiple patterns
    1..=10 => result,                // Range pattern
    _ => default_result,             // Default case (catch-all)
}
*/