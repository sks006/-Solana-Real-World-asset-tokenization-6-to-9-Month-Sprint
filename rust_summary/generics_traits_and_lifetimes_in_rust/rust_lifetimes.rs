// =============================================================================
// LIFETIMES IN RUST - COMPLETE SUMMARY
// =============================================================================
// Lifetimes = "How long does this reference stay valid?"
// They prevent dangling references (pointers to invalid data)
// =============================================================================

use std::fmt::Display;

// -----------------------------------------------------------------------------
// 1. PROBLEM: DANGLING REFERENCES (What lifetimes prevent)
// -----------------------------------------------------------------------------
fn dangling_reference_example() {
    println!("\n📌 PROBLEM: DANGLING REFERENCES");
    
    // This code WON'T compile - it would create a dangling reference!
    /*
    let r: &i32;          // Declare reference
    {
        let x = 5;         // x is created here
        r = &x;            // r points to x
    }                      // x is DESTROYED here (goes out of scope)
    println!("{}", r);     // ❌ ERROR: r points to nothing!
    */
    
    println!("  ❌ Rust prevents dangling references at compile time!");
}

// -----------------------------------------------------------------------------
// 2. LIFETIME NOTATION - VISUALIZING LIFETIMES
// -----------------------------------------------------------------------------
/*
fn main() {
    let r: &i32;          // ---+-- 'a (outer lifetime)
    {                      //    |
        let x = 5;         // ---+-- 'b (inner lifetime)
        r = &x;            //    |
    }                      // ---+-- 'b ends here (x dropped)
    println!("{}", r);     // ❌ r would be invalid - 'a outlives 'b
}
*/

// -----------------------------------------------------------------------------
// 3. GENERIC LIFETIMES IN FUNCTIONS
// -----------------------------------------------------------------------------

// ❌ This WON'T compile - Rust doesn't know how long the return value lives
/*
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() { x } else { y }
}
*/

// ✅ This WORKS - we tell Rust that all references live as long as 'a
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn generic_lifetimes_example() {
    println!("\n📌 GENERIC LIFETIMES IN FUNCTIONS");
    
    let string1 = String::from("abcd");
    let string2 = "xyz";
    
    let result = longest(string1.as_str(), string2);
    println!("  Longest: '{}'", result);
    
    // Example showing why lifetimes matter
    let result2;
    {
        let string3 = String::from("hello");
        // result2 = longest(string1.as_str(), string3.as_str());  // ❌ string3 lives shorter
        // println!("{}", result2);  // Would be invalid!
    }
    println!("  ✅ Lifetimes prevent us from using dropped data");
}

// -----------------------------------------------------------------------------
// 4. LIFETIMES WITH DIFFERENT SPANS
// -----------------------------------------------------------------------------
fn different_spans_example() {
    println!("\n📌 LIFETIMES WITH DIFFERENT SPANS");
    
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        // result = longest(string1.as_str(), string2.as_str());  // ❌ string2 dies too soon
        // println!("{}", result);  // Would be invalid!
    }
    println!("  ✅ Compiler catches lifetime mismatches");
}

// -----------------------------------------------------------------------------
// 5. INCORRECT LIFETIME USAGE (Returning local reference)
// -----------------------------------------------------------------------------
// ❌ This WON'T compile - returning reference to local data
/*
fn longest_with_local<'a>(x: &str, y: &str) -> &'a str {
    let result = String::from("really long string");
    result.as_str()  // ❌ result is dropped here!
}
*/

// -----------------------------------------------------------------------------
// 6. LIFETIME ANNOTATIONS IN STRUCTS
// -----------------------------------------------------------------------------
// Structs can hold references - they need lifetimes!
struct ImportantExcerpt<'a> {
    part: &'a str,  // This reference must live as long as the struct
}

fn struct_lifetime_example() {
    println!("\n📌 LIFETIMES IN STRUCTS");
    
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    
    let excerpt = ImportantExcerpt {
        part: first_sentence,
    };
    
    println!("  Excerpt: {}", excerpt.part);
    // excerpt cannot outlive novel!
}

// -----------------------------------------------------------------------------
// 7. LIFETIME ELISION (Rust's hidden lifetimes)
// -----------------------------------------------------------------------------
// Rust can often figure out lifetimes automatically

// This function's lifetime is inferred by Rule 1 & 2
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]
}

// The compiler sees this as:
// fn first_word<'a>(s: &'a str) -> &'a str

fn lifetime_elision_example() {
    println!("\n📌 LIFETIME ELISION");
    
    let my_string = String::from("hello world");
    let word = first_word(&my_string[..]);
    println!("  First word: '{}'", word);
    
    let literal = "hello world";
    let word = first_word(literal);
    println!("  First word from literal: '{}'", word);
}

// THREE RULES OF LIFETIME ELISION:
// 1. Each parameter gets its own lifetime: fn f<'a>(x: &'a str)
// 2. One input lifetime → same for all outputs: fn f<'a>(x: &'a str) -> &'a str
// 3. Self lifetime in methods → used for outputs

// -----------------------------------------------------------------------------
// 8. LIFETIME ANNOTATIONS IN METHODS
// -----------------------------------------------------------------------------
impl<'a> ImportantExcerpt<'a> {
    // Rule 3 applies: &self lifetime used for return
    fn level(&self) -> i32 {
        3
    }
    
    // Multiple lifetimes: self has 'a, announcement has its own
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("  Attention please: {}", announcement);
        self.part  // Returns 'a lifetime (from self)
    }
}

fn method_lifetime_example() {
    println!("\n📌 LIFETIMES IN METHODS");
    
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    
    let excerpt = ImportantExcerpt {
        part: first_sentence,
    };
    
    println!("  Level: {}", excerpt.level());
    let part = excerpt.announce_and_return_part("Hello!");
    println!("  Part: {}", part);
}

// -----------------------------------------------------------------------------
// 9. GENERICS + TRAIT BOUNDS + LIFETIMES TOGETHER
// -----------------------------------------------------------------------------
fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,  // T must be printable
{
    println!("  📢 ANNOUNCEMENT: {}", ann);
    if x.len() > y.len() { x } else { y }
}

fn all_together_example() {
    println!("\n📌 GENERICS + TRAITS + LIFETIMES");
    
    let string1 = String::from("abcd");
    let string2 = "xyz";
    
    let result = longest_with_an_announcement(
        string1.as_str(),
        string2,
        "Today is someone's birthday!",
    );
    
    println!("  Longest: {}", result);
}

// -----------------------------------------------------------------------------
// MAIN FUNCTION
// -----------------------------------------------------------------------------
fn main() {
    println!("=========================================================");
    println!("LIFETIMES IN RUST - COMPLETE SUMMARY");
    println!("=========================================================");
    
    dangling_reference_example();
    generic_lifetimes_example();
    struct_lifetime_example();
    lifetime_elision_example();
    method_lifetime_example();
    all_together_example();
    
    println!("\n=========================================================");
    println!("📌 LIFETIME RULES QUICK REFERENCE");
    println!("=========================================================");
    println!("| Rule | Description                                      |");
    println!("|------|--------------------------------------------------|");
    println!("| 1    | Each parameter gets its own lifetime            |");
    println!("|      | fn f<'a>(x: &'a str)                            |");
    println!("| 2    | One input → same for all outputs                |");
    println!("|      | fn f<'a>(x: &'a str) -> &'a str                 |");
    println!("| 3    | &self lifetime used for outputs in methods      |");
    println!("|      | fn f(&self, other: &str) -> &str                |");
    println!("=========================================================");
    
    println!("\n📌 LIFETIME NOTATION");
    println!("  • 'a = a lifetime parameter (like T for types)");
    println!("  • &'a T = a reference that lives at least as long as 'a");
    println!("  • struct Name<'a> = struct with a reference");
    println!("  • fn f<'a>(x: &'a str) = function with lifetime");
    
    println!("\n📌 KEY TAKEAWAY");
    println!("  Lifetimes tell Rust: \"These references must be valid");
    println!("  at least as long as each other.\"");
    println!("  They prevent dangling references WITHOUT runtime cost!");
}

// =============================================================================
// SUMMARY - ONE-LINERS
// =============================================================================
/*
🔑 LIFETIMES = "How long does this reference live?"
🔑 'a = a lifetime parameter (like T for types)
🔑 &'a str = "a string slice that lives at least as long as 'a"
🔑 fn f<'a>(x: &'a str) = "x must live at least as long as 'a"

Think of lifetimes as:
- Compiler instructions for memory safety
- A way to connect references that must live together
- Zero-cost abstractions (all checked at compile time!)
*/