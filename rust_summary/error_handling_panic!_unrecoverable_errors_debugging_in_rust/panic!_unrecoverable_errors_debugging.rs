// =============================================================================
// PANIC! MACRO - UNRECOVERABLE ERRORS IN RUST
// =============================================================================
// panic! is used when something goes wrong and your program cannot recover.
// It will print an error message, unwind the stack, and exit the program.
// =============================================================================

// -----------------------------------------------------------------------------
// EXAMPLE 1: BASIC PANIC
// -----------------------------------------------------------------------------
fn basic_panic() {
    println!("\n--- BASIC PANIC ---");
    println!("About to panic...");
    
    // This will crash the program with an error message
    // panic!("Something went horribly wrong!");
    
    println!("This line will never run if panic is uncommented");
}

// -----------------------------------------------------------------------------
// EXAMPLE 2: PANIC FROM ARRAY ACCESS OUT OF BOUNDS
// -----------------------------------------------------------------------------
fn array_out_of_bounds() {
    println!("\n--- ARRAY OUT OF BOUNDS ---");
    
    let v = vec![1, 2, 3];
    
    // This will panic because index 10 doesn't exist
    // println!("{}", v[10]);  // ❌ panic: index out of bounds
    
    // Safe way: use get() which returns Option
    match v.get(10) {
        Some(value) => println!("Value: {}", value),
        None => println!("Safe: Index 10 doesn't exist (no panic)"),
    }
}

// -----------------------------------------------------------------------------
// EXAMPLE 3: PANIC FROM ASSERTIONS
// -----------------------------------------------------------------------------
fn assertions() {
    println!("\n--- ASSERTIONS ---");
    
    let x = 5;
    let y = 10;
    
    // assert! macro - panics if condition is false
    // assert!(x > y, "x must be greater than y");  // ❌ This would panic
    
    // assert_eq! - panics if values are not equal
    // assert_eq!(x, y, "x and y must be equal");  // ❌ This would panic
    
    // assert_ne! - panics if values are equal
    assert_ne!(x, y, "x and y should be different");  // ✅ This passes
    
    println!("All assertions passed!");
}

// -----------------------------------------------------------------------------
// EXAMPLE 4: CUSTOM PANIC WITH CONTEXT
// -----------------------------------------------------------------------------
fn divide(numerator: i32, denominator: i32) -> i32 {
    if denominator == 0 {
        // Custom panic with helpful message
        panic!("Cannot divide {} by zero! Please provide a non-zero denominator.", numerator);
    }
    numerator / denominator
}

fn custom_panic_example() {
    println!("\n--- CUSTOM PANIC ---");
    
    let result = divide(10, 2);
    println!("10 / 2 = {}", result);
    
    // This would panic with our custom message
    // let bad_result = divide(10, 0);  // ❌ PANIC: "Cannot divide 10 by zero!"
}

// -----------------------------------------------------------------------------
// EXAMPLE 5: UNWRAP() AND EXPECT() - CONVENIENCE METHODS THAT CAN PANIC
// -----------------------------------------------------------------------------
fn unwrap_and_expect() {
    println!("\n--- UNWRAP AND EXPECT ---");
    
    let some_value: Option<i32> = Some(42);
    let no_value: Option<i32> = None;
    
    // unwrap() - returns value if Some, panics if None
    println!("some_value.unwrap(): {}", some_value.unwrap());
    
    // This would panic:
    // println!("no_value.unwrap(): {}", no_value.unwrap());  // ❌ PANIC
    
    // expect() - same as unwrap but with custom message
    println!("some_value.expect(): {}", some_value.expect("Value should exist"));
    
    // This would panic with our message:
    // no_value.expect("Expected a value but got None");  // ❌ PANIC with custom message
    
    // Same with Result
    let ok_result: Result<i32, &str> = Ok(100);
    let err_result: Result<i32, &str> = Err("Something failed");
    
    println!("ok_result.unwrap(): {}", ok_result.unwrap());
    // println!("err_result.unwrap(): {}", err_result.unwrap());  // ❌ PANIC
}

// -----------------------------------------------------------------------------
// EXAMPLE 6: PANIC BACKTRACE (for debugging)
// -----------------------------------------------------------------------------
/*
To get a backtrace when your program panics, run:
RUST_BACKTRACE=1 cargo run

This shows the full call stack leading to the panic.
*/

fn function_a() {
    println!("In function A");
    function_b();
}

fn function_b() {
    println!("In function B");
    function_c();
}

fn function_c() {
    println!("In function C");
    // This will panic, and with RUST_BACKTRACE you can see the full call chain
    // panic!("Something went wrong in function C!");
}

fn backtrace_example() {
    println!("\n--- BACKTRACE EXAMPLE ---");
    println!("Run with RUST_BACKTRACE=1 to see the call stack");
    function_a();
}

// -----------------------------------------------------------------------------
// EXAMPLE 7: CATCHING PANICS (Advanced - usually not needed)
// -----------------------------------------------------------------------------
use std::panic;

fn catching_panics() {
    println!("\n--- CATCHING PANICS (rarely used) ---");
    
    let result = panic::catch_unwind(|| {
        println!("Inside a closure that might panic...");
        // panic!("Oh no!");  // This panic is caught
        42  // Return value if no panic
    });
    
    match result {
        Ok(val) => println!("Success: {}", val),
        Err(_) => println!("Caught a panic!"),
    }
    
    println!("Program continues...");
}

// -----------------------------------------------------------------------------
// EXAMPLE 8: WHEN TO USE PANIC VS RESULT
// -----------------------------------------------------------------------------
/*
USE PANIC WHEN:
---------------
• Examples or prototype code
• Tests (use panic to fail tests)
• You're sure the code will never fail (unwrapping a hardcoded value)
• Something truly unrecoverable happens (out of memory)

USE RESULT WHEN:
----------------
• Expected errors (file not found, invalid input)
• You want the caller to decide how to handle the error
• In library code (let users decide what to do)
• When failure is a normal possibility
*/

fn when_to_panic() {
    println!("\n--- WHEN TO PANIC ---");
    
    // ✅ OK to panic in examples/tests
    // panic!("This test failed");
    
    // ✅ OK to panic on unrecoverable errors
    let config = match std::fs::read_to_string("config.txt") {
        Ok(content) => content,
        Err(e) => panic!("Cannot read config file: {}", e), // App can't run without config
    };
    
    // ✅ OK to unwrap when you're 100% sure it won't fail
    let today = "2023-12-25".parse::<i32>().unwrap(); // We know this parses
    
    // ❌ NOT OK to panic for expected errors
    // Use Result instead:
    let user_input = "not a number";
    match user_input.parse::<i32>() {
        Ok(num) => println!("Number: {}", num),
        Err(_) => println!("Please enter a valid number"), // Handle gracefully
    }
}

// -----------------------------------------------------------------------------
// MAIN FUNCTION
// -----------------------------------------------------------------------------
fn main() {
    println!("=========================================================");
    println!("PANIC! MACRO - UNRECOVERABLE ERRORS");
    println!("=========================================================");
    
    println!("\n📌 WHAT IS PANIC?");
    println!("  • panic! = crash the program with an error message");
    println!("  • Used for UNRECOVERABLE errors");
    println!("  • Prints error, unwinds stack, exits");
    
    // Uncomment examples one by one to see panics in action
    basic_panic();
    array_out_of_bounds();
    assertions();
    custom_panic_example();
    unwrap_and_expect();
    // backtrace_example();  // Uncomment to see panic
    catching_panics();
    when_to_panic();
    
    println!("\n=========================================================");
    println!("📌 PANIC QUICK REFERENCE");
    println!("=========================================================");
    println!("| Code                      | What it does                  |");
    println!("|---------------------------|-------------------------------|");
    println!("| panic!(\"msg\");            | Crash with message            |");
    println!("| assert!(condition);       | Panic if condition is false   |");
    println!("| assert_eq!(a, b);          | Panic if a != b               |");
    println!("| assert_ne!(a, b);          | Panic if a == b               |");
    println!("| option.unwrap()           | Panic if None                 |");
    println!("| option.expect(\"msg\")      | Panic if None with message    |");
    println!("| result.unwrap()            | Panic if Err                  |");
    println!("| result.expect(\"msg\")       | Panic if Err with message     |");
    println!("| vec[999]                   | Panic if index out of bounds  |");
    println!("=========================================================");
    
    println!("\n📌 TO SEE BACKTRACE: RUST_BACKTRACE=1 cargo run");
    println!("📌 TO DISABLE UNWINDING (smaller binary): add to Cargo.toml");
    println!("   [profile.release]");
    println!("   panic = 'abort'");
}

// =============================================================================
// REAL-WORLD EXAMPLE: CONFIGURATION LOADER
// =============================================================================
struct Config {
    server: String,
    port: u16,
}

impl Config {
    fn from_file(path: &str) -> Result<Self, std::io::Error> {
        // Proper error handling with Result
        let content = std::fs::read_to_string(path)?;
        // Parse content...
        Ok(Config {
            server: String::from("localhost"),
            port: 8080,
        })
    }
    
    fn from_env() -> Self {
        // Panic if required env vars are missing - app can't run without them
        let server = std::env::var("SERVER_ADDR")
            .expect("SERVER_ADDR must be set");
        let port = std::env::var("PORT")
            .expect("PORT must be set")
            .parse()
            .expect("PORT must be a number");
        
        Config { server, port }
    }
}

// =============================================================================
// SUMMARY
// =============================================================================
/*
🔑 KEY TAKEAWAYS:
----------------
1. panic! = crash the program (unrecoverable)
2. Use for bugs, impossible states, or when program cannot continue
3. For expected errors (file not found, invalid input), use Result
4. unwrap() and expect() are shortcuts that panic on error
5. Backtrace helps debug where panic originated
6. Tests use panic to indicate failure

💡 SIMPLE RULE:
--------------
• If the error is EXPECTED and the CALLER should decide: use Result
• If the error is UNEXPECTED and the PROGRAM CAN'T CONTINUE: use panic!
*/