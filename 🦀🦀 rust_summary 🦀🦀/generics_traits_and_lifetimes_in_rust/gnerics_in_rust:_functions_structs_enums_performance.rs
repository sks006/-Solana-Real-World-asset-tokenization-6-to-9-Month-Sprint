// =============================================================================
// GENERICS IN RUST - SIMPLIFIED
// =============================================================================
// Generics = "Write code that works with ANY type"
// Like a template - fill in the type later
// =============================================================================

use std::cmp::PartialOrd;

// -----------------------------------------------------------------------------
// PART 1: GENERIC FUNCTIONS
// -----------------------------------------------------------------------------

// WITHOUT GENERICS (duplicate code)
fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// WITH GENERICS (one function for ALL types!)
// <T: PartialOrd> means "T must be a type that can be compared with >"
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    
    largest
}

fn generic_functions_example() {
    println!("\n📌 GENERIC FUNCTIONS");
    
    // Works with i32
    let numbers = vec![34, 50, 25, 100, 65];
    let result = largest(&numbers);
    println!("  Largest number: {}", result);
    
    // Works with char
    let chars = vec!['y', 'm', 'a', 'q'];
    let result = largest(&chars);
    println!("  Largest char: '{}'", result);
    
    // Works with f64
    let floats = vec![1.5, 3.2, 0.8, 2.7];
    let result = largest(&floats);
    println!("  Largest float: {}", result);
}

// -----------------------------------------------------------------------------
// PART 2: GENERIC STRUCTS
// -----------------------------------------------------------------------------

// Point can hold ANY types for x and y
struct Point<T, U> {
    x: T,
    y: U,
}

// Implementation for ALL Point types
impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }
    
    fn y(&self) -> &U {
        &self.y
    }
    
    // Mix two points together
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

// Special implementation ONLY for Points with f32 fields
impl Point<f32, f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn generic_structs_example() {
    println!("\n📌 GENERIC STRUCTS");
    
    // Both fields same type
    let p1 = Point { x: 5, y: 10 };
    let p2 = Point { x: 1.0, y: 4.2 };
    let p3 = Point { x: 'a', y: 'b' };
    
    println!("  i32 Point: ({}, {})", p1.x, p1.y);
    println!("  f64 Point: ({}, {})", p2.x, p2.y);
    println!("  char Point: ({}, {})", p3.x, p3.y);
    
    // Different types for x and y
    let p4 = Point { x: 5, y: 4.2 };
    let p5 = Point { x: 5, y: 'b' };
    
    println!("  i32+f64 Point: ({}, {})", p4.x, p4.y);
    println!("  i32+char Point: ({}, {})", p5.x, p5.y);
    
    // Distance from origin (only works for f32,f32)
    let origin_point = Point { x: 3.0_f32, y: 4.0_f32 };
    let dist = origin_point.distance_from_origin();
    println!("  Distance from origin: {}", dist);
    
    // Mixup example
    let p6 = Point { x: 5, y: 10.4 };
    let p7 = Point { x: "Hello", y: 'c' };
    let p8 = p6.mixup(p7);
    println!("  Mixed point: ({}, {})", p8.x, p8.y);
}

// -----------------------------------------------------------------------------
// PART 3: GENERIC ENUMS
// -----------------------------------------------------------------------------

// Option<T> - T can be ANY type
#[derive(Debug)]
enum MyOption<T> {
    Some(T),
    None,
}

// Result<T, E> - T and E can be ANY types
#[derive(Debug)]
enum MyResult<T, E> {
    Ok(T),
    Err(E),
}

fn generic_enums_example() {
    println!("\n📌 GENERIC ENUMS");
    
    // Option with different types
    let some_number: MyOption<i32> = MyOption::Some(42);
    let some_text: MyOption<String> = MyOption::Some(String::from("hello"));
    let nothing: MyOption<i32> = MyOption::None;
    
    println!("  Option<i32>: {:?}", some_number);
    println!("  Option<String>: {:?}", some_text);
    println!("  Option<None>: {:?}", nothing);
    
    // Result with different types
    let success: MyResult<i32, String> = MyResult::Ok(100);
    let failure: MyResult<i32, String> = MyResult::Err(String::from("error"));
    
    println!("  Result success: {:?}", success);
    println!("  Result failure: {:?}", failure);
}

// -----------------------------------------------------------------------------
// PART 4: GENERICS WITH TRAIT BOUNDS
// -----------------------------------------------------------------------------

use std::fmt::Display;

// Function that works with ANY type that can be:
// 1. Compared (PartialOrd)
// 2. Printed (Display)
fn find_and_print<T: PartialOrd + Display>(list: &[T]) -> Option<&T> {
    if list.is_empty() {
        return None;
    }
    
    let mut largest = &list[0];
    
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    
    println!("  Found largest: {}", largest);
    Some(largest)
}

// Alternative syntax with 'where' clause (cleaner for complex bounds)
fn find_and_print_where<T>(list: &[T]) -> Option<&T>
where
    T: PartialOrd + Display,
{
    if list.is_empty() {
        return None;
    }
    
    let mut largest = &list[0];
    
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    
    println!("  Found largest: {}", largest);
    Some(largest)
}

fn trait_bounds_example() {
    println!("\n📌 GENERICS WITH TRAIT BOUNDS");
    
    let numbers = vec![10, 20, 5, 30, 15];
    find_and_print(&numbers);
    
    let words = vec!["apple", "zebra", "banana", "mango"];
    find_and_print_where(&words);
}

// -----------------------------------------------------------------------------
// MAIN FUNCTION
// -----------------------------------------------------------------------------
fn main() {
    println!("=========================================================");
    println!("GENERICS IN RUST - SIMPLIFIED");
    println!("=========================================================");
    
    generic_functions_example();
    generic_structs_example();
    generic_enums_example();
    trait_bounds_example();
    
    println!("\n=========================================================");
    println!("📌 QUICK REFERENCE");
    println!("=========================================================");
    println!("| Syntax                | Meaning                          |");
    println!("|-----------------------|----------------------------------|");
    println!("| <T>                   | Any single type                  |");
    println!("| <T, U>                | Two different types              |");
    println!("| T: Display            | T must be printable              |");
    println!("| T: PartialOrd         | T must be comparable             |");
    println!("| T: Display + Clone    | T must be printable AND cloneable|");
    println!("| where T: Display      | Alternative syntax for bounds    |");
    println!("=========================================================");
    
    println!("\n📌 YOU'VE BEEN USING GENERICS ALL ALONG!");
    println!("  • Option<T> - T can be any type");
    println!("  • Vec<T> - T can be any type");
    println!("  • Result<T, E> - T and E can be any types");
    println!("  • HashMap<K, V> - K and V can be any types");
}

// =============================================================================
// SUMMARY - ONE-LINERS
// =============================================================================
/*
🔑 GENERIC FUNCTIONS:  fn name<T>(arg: T) { ... }
🔑 GENERIC STRUCTS:    struct Name<T> { field: T }
🔑 GENERIC ENUMS:      enum Name<T> { Variant(T) }
🔑 TRAIT BOUNDS:       fn name<T: Trait>(arg: T) { ... }
🔑 WHERE CLAUSE:       fn name<T>(arg: T) where T: Trait { ... }

Think of <T> as a placeholder: "I'll tell you the actual type later!"
*/