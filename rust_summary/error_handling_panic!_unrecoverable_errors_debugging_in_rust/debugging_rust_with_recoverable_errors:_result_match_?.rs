// =============================================================================
// RECOVERABLE ERRORS WITH RESULT<T, E>
// =============================================================================
// Result is used for errors that can be handled gracefully
// The program can recover and continue running
// =============================================================================

use std::fs::File;
use std::io::{self, Read, Write};
use std::io::ErrorKind;
use std::error::Error;
use std::fs;

// -----------------------------------------------------------------------------
// 1. BASIC RESULT HANDLING WITH MATCH
// -----------------------------------------------------------------------------
fn basic_match_example() {
    println!("\n📌 1. BASIC MATCH HANDLING");
    
    // File::open returns Result<File, std::io::Error>
    let result = File::open("hello.txt");
    
    match result {
        Ok(file) => println!("  ✅ File opened successfully: {:?}", file),
        Err(error) => println!("  ❌ Failed to open file: {}", error),
    }
}

// -----------------------------------------------------------------------------
// 2. HANDLING DIFFERENT ERROR TYPES
// -----------------------------------------------------------------------------
fn handle_specific_errors() {
    println!("\n📌 2. HANDLING SPECIFIC ERRORS");
    
    let result = File::open("hello.txt");
    
    match result {
        Ok(file) => println!("  ✅ File opened: {:?}", file),
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                println!("  ❌ File not found! Creating a new file...");
                match File::create("hello.txt") {
                    Ok(new_file) => println!("  ✅ Created new file: {:?}", new_file),
                    Err(e) => println!("  ❌ Failed to create file: {}", e),
                }
            }
            ErrorKind::PermissionDenied => {
                println!("  ❌ Permission denied! Can't access file");
            }
            other_error => {
                println!("  ❌ Other error: {}", other_error);
            }
        },
    }
}

// -----------------------------------------------------------------------------
// 3. PROPAGATING ERRORS (Returning Result to caller)
// -----------------------------------------------------------------------------
fn read_username_from_file_match() -> Result<String, io::Error> {
    println!("\n📌 3. PROPAGATING ERRORS WITH MATCH");
    
    // Open file - if error, return it immediately
    let file_result = File::open("hello.txt");
    let mut file = match file_result {
        Ok(f) => f,
        Err(e) => return Err(e),  // Return error to caller
    };
    
    // Read file contents - if error, return it
    let mut username = String::new();
    match file.read_to_string(&mut username) {
        Ok(_) => Ok(username),     // Return success with data
        Err(e) => Err(e),          // Return error
    }
}

// -----------------------------------------------------------------------------
// 4. THE ? OPERATOR (Shorthand for propagating errors)
// -----------------------------------------------------------------------------
fn read_username_from_file_question() -> Result<String, io::Error> {
    println!("\n📌 4. USING ? OPERATOR");
    
    // ? does the same as the match above:
    // - If Ok, unwrap the value
    // - If Err, return it immediately
    let mut file = File::open("hello.txt")?;
    let mut username = String::new();
    file.read_to_string(&mut username)?;
    
    Ok(username)
}

// -----------------------------------------------------------------------------
// 5. CHAINING ? OPERATOR
// -----------------------------------------------------------------------------
fn read_username_chained() -> Result<String, io::Error> {
    println!("\n📌 5. CHAINING ? OPERATOR");
    
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    
    Ok(username)
}

// -----------------------------------------------------------------------------
// 6. USING fs::read_to_string (Built-in helper)
// -----------------------------------------------------------------------------
fn read_username_builtin() -> Result<String, io::Error> {
    println!("\n📌 6. USING fs::read_to_string");
    
    // This does everything in one line!
    fs::read_to_string("hello.txt")
}

// -----------------------------------------------------------------------------
// 7. MAIN CAN RETURN RESULT
// -----------------------------------------------------------------------------
fn main() -> Result<(), Box<dyn Error>> {
    println!("=========================================================");
    println!("RECOVERABLE ERRORS WITH RESULT");
    println!("=========================================================");
    
    // Run all examples
    basic_match_example();
    
    // This will create hello.txt if it doesn't exist
    handle_specific_errors();
    
    // Try reading username
    match read_username_from_file_match() {
        Ok(name) => println!("  ✅ Username from match: {}", name),
        Err(e) => println!("  ❌ Error reading username: {}", e),
    }
    
    match read_username_from_file_question() {
        Ok(name) => println!("  ✅ Username with ?: {}", name),
        Err(e) => println!("  ❌ Error reading with ?: {}", e),
    }
    
    match read_username_chained() {
        Ok(name) => println!("  ✅ Username chained: {}", name),
        Err(e) => println!("  ❌ Error chained: {}", e),
    }
    
    match read_username_builtin() {
        Ok(name) => println!("  ✅ Username builtin: {}", name),
        Err(e) => println!("  ❌ Error builtin: {}", e),
    }
    
    // -------------------------------------------------------------------------
    // 8. WRITING TO FILE EXAMPLE
    // -------------------------------------------------------------------------
    println!("\n📌 8. WRITING TO FILE");
    
    let mut file = File::create("output.txt")?;
    file.write_all(b"Hello, Rust!")?;
    println!("  ✅ Wrote to output.txt");
    
    // Read it back
    let content = fs::read_to_string("output.txt")?;
    println!("  ✅ Read back: '{}'", content);
    
    // -------------------------------------------------------------------------
    // 9. COMBINING MULTIPLE OPERATIONS
    // -------------------------------------------------------------------------
    println!("\n📌 9. COMBINING OPERATIONS");
    
    fn process_file() -> Result<String, io::Error> {
        let content = fs::read_to_string("input.txt")?;
        
        if content.is_empty() {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "File is empty"));
        }
        
        let processed = content.to_uppercase();
        fs::write("output.txt", &processed)?;
        
        Ok(processed)
    }
    
    match process_file() {
        Ok(data) => println!("  ✅ Processed: {}", data),
        Err(e) => println!("  ❌ Processing failed: {}", e),
    }
    
    // -------------------------------------------------------------------------
    // SUMMARY TABLE
    // -------------------------------------------------------------------------
    println!("\n=========================================================");
    println!("📌 RESULT OPERATIONS QUICK REFERENCE");
    println!("=========================================================");
    println!("| Operation                    | Example                          |");
    println!("|------------------------------|----------------------------------|");
    println!("| Match on Result              | match result { Ok(v) => {}, Err(e) => {} } |");
    println!("| Propagate error (match)      | Err(e) => return Err(e)          |");
    println!("| Propagate error (? operator) | File::open()?;                   |");
    println!("| Create custom error          | Err(io::Error::new(...))         |");
    println!("| Unwrap (panic on error)      | result.unwrap()                  |");
    println!("| Expect (panic with message)  | result.expect(\"msg\")            |");
    println!("| Default on error             | result.unwrap_or(default)        |");
    println!("| Map Ok value                  | result.map(|v| v+1)              |");
    println!("| Map Err value                 | result.map_err(|e| e.to_string())|");
    println!("=========================================================");
    
    println!("\n📌 RULES FOR ? OPERATOR:");
    println!("  • Can ONLY be used in functions that return Result or Option");
    println!("  • If Ok, unwraps the value");
    println!("  • If Err, returns it immediately");
    println!("  • Makes error handling much cleaner!");
    
    Ok(())  // Success!
}

// =============================================================================
// CUSTOM ERROR TYPES EXAMPLE
// =============================================================================
#[derive(Debug)]
enum MyError {
    IoError(std::io::Error),
    ParseError(std::num::ParseIntError),
    CustomError(String),
}

impl From<std::io::Error> for MyError {
    fn from(error: std::io::Error) -> Self {
        MyError::IoError(error)
    }
}

impl From<std::num::ParseIntError> for MyError {
    fn from(error: std::num::ParseIntError) -> Self {
        MyError::ParseError(error)
    }
}

fn read_number_from_file() -> Result<i32, MyError> {
    let content = fs::read_to_string("number.txt")?;  // Converts to MyError automatically
    let number = content.trim().parse::<i32>()?;      // Converts to MyError automatically
    Ok(number)
}

// =============================================================================
// COMPARISON: PANIC vs RESULT
// =============================================================================
/*
┌─────────────────┬─────────────────────────────────┬─────────────────────────────────┐
│                 │ PANIC!                          │ RESULT<T, E>                    │
├─────────────────┼─────────────────────────────────┼─────────────────────────────────┤
│ What it does    │ Crashes the program             │ Returns Ok or Err for handling  │
│ Recoverable?    │ No - unrecoverable              │ Yes - caller decides what to do │
│ When to use     │ Bugs, impossible states         │ Expected errors (file not found)│
│ Example         │ Index out of bounds             │ File::open("file.txt")          │
│ Return type     │ ! (never type)                  │ Result<T, E>                    │
│ Handling        │ Cannot be handled               │ match, ?, unwrap(), etc.        │
└─────────────────┴─────────────────────────────────┴─────────────────────────────────┘
*/

// =============================================================================
// COMMON PATTERNS
// =============================================================================
fn common_patterns() {
    println!("\n📌 COMMON RESULT PATTERNS");
    
    // unwrap() - panic on error (use only when you're sure it won't fail)
    let file = File::open("Cargo.toml").unwrap();  // Safe - Cargo.toml exists
    
    // expect() - panic with custom message
    // let file = File::open("missing.txt").expect("Failed to open config file");
    
    // unwrap_or() - provide default on error
    let port = "8080".parse::<u16>().unwrap_or(3000);
    println!("  Port: {}", port);
    
    // unwrap_or_else() - compute default lazily
    let config = fs::read_to_string("config.txt")
        .unwrap_or_else(|_| String::from("default config"));
    
    // map() - transform Ok value
    let len = File::open("Cargo.toml")
        .map(|f| f.metadata().map(|m| m.len()).unwrap_or(0))
        .unwrap_or(0);
    println!("  File length: {}", len);
    
    // and_then() - chain operations that return Result
    let content = File::open("Cargo.toml")
        .and_then(|mut f| {
            let mut s = String::new();
            f.read_to_string(&mut s).map(|_| s)
        })
        .unwrap_or_default();
    println!("  Content length: {}", content.len());
}

// Uncomment to run common patterns
// fn main() -> Result<(), Box<dyn Error>> {
//     common_patterns();
//     Ok(())
// }

Key Points to Remember:
Result<T, E> = Ok(T) on success, Err(E) on failure

Match = Full control over error handling

? operator = Propagate errors to caller (clean and concise)

unwrap() = Panic on error (use carefully!)

expect() = Panic with custom message

Main can return Result = Use -> Result<(), Box<dyn Error>>