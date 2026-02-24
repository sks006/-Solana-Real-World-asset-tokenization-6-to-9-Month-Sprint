// =============================================================================
// RUST CHEAT SHEET: ENUMS, STRUCTS, METHODS, AND OPTION
// =============================================================================
// This file contains all the essential concepts with minimal, clear examples.
// Each section is independent and shows the most basic syntax.
// =============================================================================



// =============================================================================
// SECTION 1: ENUMS (Multiple Choice Types)
// =============================================================================
// Enums let you say "this value can be one of these options"
// Like a dropdown menu in code!

// Example 1.1: Simple enum (no data)
#[derive(Debug, PartialEq)]  // Debug = can print, PartialEq = can compare
enum SimpleIP {
    V4,  // Option 1
    V6,  // Option 2
}

// Example 1.2: Enum with data (each option can hold different data)
#[derive(Debug)]
enum IPWithData {
    V4(u8, u8, u8, u8),  // Holds 4 numbers (like 127,0,0,1)
    V6(String),           // Holds 1 string (like "::1")
}

// Example 1.3: Enum with different kinds of data
#[derive(Debug)]
enum Message {
    Quit,                       // No data
    Move { x: i32, y: i32 },    // Struct-like (named fields)
    Write(String),              // Tuple-like (one value)
    ChangeColor(i32, i32, i32), // Tuple-like (three values)
}

// Function to demonstrate enums
fn enum_demo() {
    println!("\n--- ENUM DEMO ---");
    
    // Creating enum instances
    let v4 = SimpleIP::V4;
    let v6 = SimpleIP::V6;
    println!("Simple enums: {:?}, {:?}", v4, v6);
    
    // Comparing enums
    println!("Are they equal? {}", v4 == v6);  // false
    
    // Enums with data
    let home = IPWithData::V4(127, 0, 0, 1);
    let loopback = IPWithData::V6(String::from("::1"));
    println!("With data: {:?}, {:?}", home, loopback);
    
    // Getting data out using match
    match home {
        IPWithData::V4(a, b, c, d) => {
            println!("  Extracted IPv4: {}.{}.{}.{}", a, b, c, d);
        }
        IPWithData::V6(addr) => {
            println!("  Extracted IPv6: {}", addr);
        }
    }
}



// =============================================================================
// SECTION 2: STRUCTS (Grouping Data Together)
// =============================================================================
// Structs are like forms - they group related pieces of data

// Example 2.1: Unit struct (no data) - just a marker
#[derive(Debug)]
struct QuitCommand;

// Example 2.2: Classic struct (named fields) - most common
#[derive(Debug)]
struct MoveCommand {
    x: i32,
    y: i32,
}

// Example 2.3: Tuple struct (unnamed fields) - accessed by index
#[derive(Debug)]
struct WriteCommand(String);  // One value

#[derive(Debug)]
struct ColorCommand(i32, i32, i32);  // Three values

// Function to demonstrate structs
fn struct_demo() {
    println!("\n--- STRUCT DEMO ---");
    
    // Creating instances
    let quit = QuitCommand;
    let move_cmd = MoveCommand { x: 10, y: 20 };
    let write_cmd = WriteCommand(String::from("Hello"));
    let color_cmd = ColorCommand(255, 0, 0);
    
    // Printing
    println!("Unit struct: {:?}", quit);
    println!("Named struct: {:?}", move_cmd);
    println!("Tuple struct: {:?}", write_cmd);
    println!("Tuple struct: {:?}", color_cmd);
    
    // Accessing data
    println!("Move to: ({}, {})", move_cmd.x, move_cmd.y);
    println!("Write: {}", write_cmd.0);        // .0 for first tuple field
    println!("Color: ({},{},{})", color_cmd.0, color_cmd.1, color_cmd.2);
}



// =============================================================================
// SECTION 3: METHODS (Actions that structs/enums can do)
// =============================================================================
// Methods are functions attached to a type using 'impl'

// First, define a struct
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Implementation block - all methods for Rectangle go here
impl Rectangle {
    // Method that borrows self (can read but not change)
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    // Method that can change self (needs &mut self)
    fn double_size(&mut self) {
        self.width *= 2;
        self.height *= 2;
    }
    
    // Associated function (no self) - like a constructor
    // Called with Rectangle::new() instead of rectangle.new()
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }
    
    // Another associated function
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

// Methods for enums too!
impl Message {  // Message enum from Section 1
    fn process(&self) {
        match self {
            Message::Quit => println!("  Processing: Quit"),
            Message::Move { x, y } => println!("  Processing: Move to ({},{})", x, y),
            Message::Write(text) => println!("  Processing: Write '{}'", text),
            Message::ChangeColor(r, g, b) => {
                println!("  Processing: Change color to RGB({},{},{})", r, g, b);
            }
        }
    }
}

// Function to demonstrate methods
fn method_demo() {
    println!("\n--- METHOD DEMO ---");
    
    // Using associated functions (constructors)
    let mut rect = Rectangle::new(10, 20);
    let square = Rectangle::square(15);
    
    // Using methods (called on instances)
    println!("Rectangle area: {}", rect.area());
    println!("Square area: {}", square.area());
    
    rect.double_size();
    println!("After doubling: {}x{} area={}", rect.width, rect.height, rect.area());
    
    // Enum methods
    let msgs = [
        Message::Quit,
        Message::Move { x: 5, y: 10 },
        Message::Write(String::from("Hello")),
        Message::ChangeColor(255, 0, 0),
    ];
    
    for msg in msgs.iter() {
        msg.process();
    }
}



// =============================================================================
// SECTION 4: OPTION (Maybe has value, maybe doesn't)
// =============================================================================
// Option is Rust's way of saying "this might be nothing"
// It's so common it's built into the language!

// This is what Option looks like (simplified):
// enum Option<T> {
//     Some(T),   // Has a value
//     None,      // Has no value
// }

fn option_demo() {
    println!("\n--- OPTION DEMO ---");
    
    // Creating Options
    let has_value = Some(42);           // Has the value 42
    let no_value: Option<i32> = None;   // Has no value (must specify type)
    
    // Printing Options
    println!("has_value: {:?}", has_value);
    println!("no_value: {:?}", no_value);
    
    // 3 WAYS TO GET THE VALUE:
    
    // 1. match - safest, handles both cases
    match has_value {
        Some(x) => println!("  match got: {}", x),
        None => println!("  match got: nothing"),
    }
    
    match no_value {
        Some(x) => println!("  match got: {}", x),
        None => println!("  match got: nothing"),
    }
    
    // 2. unwrap_or - value or default
    println!("  unwrap_or: {}", has_value.unwrap_or(0));  // 42
    println!("  unwrap_or: {}", no_value.unwrap_or(0));   // 0
    
    // 3. if let - when you only care about Some case
    if let Some(x) = has_value {
        println!("  if let got: {}", x);
    }
    
    // Checking before using
    if has_value.is_some() {
        println!("  has_value contains: {}", has_value.unwrap());
    }
    
    if no_value.is_none() {
        println!("  no_value contains nothing (safe!)");
    }
    
    // Common pattern: operations that might fail
    fn divide(numerator: f64, denominator: f64) -> Option<f64> {
        if denominator == 0.0 {
            None  // Can't divide by zero
        } else {
            Some(numerator / denominator)
        }
    }
    
    println!("  10/2 = {:?}", divide(10.0, 2.0));
    println!("  10/0 = {:?}", divide(10.0, 0.0));
}



// =============================================================================
// SECTION 5: ROUTE FUNCTION (Using enum as parameter)
// =============================================================================
// A simple function that works with enums

#[derive(Debug)]
enum IpVersion {
    V4,
    V6,
}

// Function that takes an enum - can handle both variants
fn route(ip: IpVersion) {
    println!("Routing {:?} packet...", ip);
    
    // Different behavior based on variant
    match ip {
        IpVersion::V4 => println!("  Using IPv4 routing table"),
        IpVersion::V6 => println!("  Using IPv6 routing table"),
    }
}

fn route_demo() {
    println!("\n--- ROUTE FUNCTION DEMO ---");
    
    let v4 = IpVersion::V4;
    let v6 = IpVersion::V6;
    
    route(v4);
    route(v6);
}



// =============================================================================
// MAIN FUNCTION - Run all demos
// =============================================================================
fn main() {
    println!("=========================================================");
    println!("RUST QUICK REFERENCE - ENUMS, STRUCTS, METHODS, OPTION");
    println!("=========================================================");
    
    enum_demo();
    struct_demo();
    method_demo();
    option_demo();
    route_demo();
    
    println!("\n=========================================================");
    println!("Each section is independent. Look at the one you need!");
    println!("=========================================================");
}



// =============================================================================
// QUICK REFERENCE - Copy this part if you need a reminder
// =============================================================================
/*
 
ENUMS (multiple choices):
------------------------
enum Name { A, B, C }           // Simple
enum Name { A(i32), B(String) } // With data

Create: let x = Name::A;
Use:    match x { Name::A => {}, Name::B => {} }


STRUCTS (group data):
--------------------
struct Name;                    // Unit (no data)
struct Name { x: i32, y: i32 }  // Named fields
struct Name(i32, String);       // Tuple struct

Create: let a = Name;
        let b = Name { x: 5, y: 10 };
        let c = Name(5, String::from("hi"));

Access: b.x, c.0


METHODS (actions):
-----------------
impl Name {
    fn method(&self) {}          // Read-only
    fn method(&mut self) {}      // Can change
    fn new() -> Self {}          // Constructor (no self)
}

Call: let x = Name::new();       // Constructor
      x.method();                // Method


OPTION (value or nothing):
-------------------------
let x = Some(5);                  // Has value
let y: Option<i32> = None;        // No value

Get value: x.unwrap_or(0)         // Value or default
           match x { Some(v) => {}, None => {} }

*/