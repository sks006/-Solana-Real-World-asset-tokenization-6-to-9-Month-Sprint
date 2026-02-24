// =============================================================================
// GENERICS, TRAITS & LIFETIMES - SIMPLIFIED
// =============================================================================

// -----------------------------------------------------------------------------
// PART 1: GENERICS - "Work with ANY type"
// -----------------------------------------------------------------------------
// You've already seen generics everywhere!

fn generics_example() {
    println!("\n📌 GENERICS - YOU ALREADY KNOW THESE!");
    
    // Option<T> - T can be ANY type
    let opt_number: Option<i32> = Some(5);
    let opt_text: Option<String> = Some(String::from("hello"));
    println!("  Option<i32>: {:?}", opt_number);
    println!("  Option<String>: {:?}", opt_text);
    
    // Vec<T> - T can be ANY type
    let vec_numbers: Vec<i32> = vec![1, 2, 3];
    let vec_text: Vec<String> = vec![
        String::from("a"),
        String::from("b"),
    ];
    println!("  Vec<i32>: {:?}", vec_numbers);
    println!("  Vec<String>: {:?}", vec_text);
    
    // HashMap<K, V> - K and V can be ANY types
    use std::collections::HashMap;
    let mut map: HashMap<String, i32> = HashMap::new();
    map.insert(String::from("score"), 100);
    println!("  HashMap<String, i32>: {:?}", map);
    
    // Result<T, E> - T and E can be ANY types
    let success: Result<i32, String> = Ok(42);
    let failure: Result<i32, String> = Err(String::from("error"));
    println!("  Result<i32, String>: {:?}, {:?}", success, failure);
}

// Generic function - works with ANY type
fn print_anything<T: std::fmt::Debug>(thing: T) {
    println!("  Generic function got: {:?}", thing);
}

// -----------------------------------------------------------------------------
// PART 2: TRAITS - "SHARED BEHAVIOR"
// -----------------------------------------------------------------------------
// Traits = defining what methods a type MUST have

// Define a trait (like an interface)
trait Summarizable {
    fn summary(&self) -> String;
}

// Define some structs
struct NewsArticle {
    headline: String,
    location: String,
}

struct Tweet {
    username: String,
    content: String,
}

// Implement the trait for NewsArticle
impl Summarizable for NewsArticle {
    fn summary(&self) -> String {
        format!("{} - {}", self.headline, self.location)
    }
}

// Implement the trait for Tweet
impl Summarizable for Tweet {
    fn summary(&self) -> String {
        format!("@{}: {}", self.username, self.content)
    }
}

// Function that works with ANY type that implements Summarizable
fn print_summary<T: Summarizable>(item: T) {
    println!("  {}", item.summary());
}

fn traits_example() {
    println!("\n📌 TRAITS - SHARED BEHAVIOR");
    
    let article = NewsArticle {
        headline: String::from("Rust 1.0 Released!"),
        location: String::from("Internet"),
    };
    
    let tweet = Tweet {
        username: String::from("rustlang"),
        content: String::from("Welcome to Rust 1.0!"),
    };
    
    // Both have .summary() even though they're different types!
    println!("  Article: {}", article.summary());
    println!("  Tweet: {}", tweet.summary());
    
    // Function works with BOTH types
    print_summary(article);
    print_summary(tweet);
}

// -----------------------------------------------------------------------------
// PART 3: LIFETIMES - "HOW LONG DOES THIS LIVE?"
// -----------------------------------------------------------------------------
// Lifetimes = preventing dangling references

// WITHOUT lifetimes (won't compile):
// fn longest(x: &str, y: &str) -> &str {  // ERROR!
//     if x.len() > y.len() { x } else { y }
// }

// WITH lifetimes - tells Rust "x and y must live as long as the return value"
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn lifetimes_example() {
    println!("\n📌 LIFETIMES - PREVENTING DANGLING REFERENCES");
    
    let string1 = String::from("long string is long");
    let string2 = "xyz";  // &str
    
    let result = longest(string1.as_str(), string2);
    println!("  Longest: '{}'", result);
    
    // What lifetimes prevent:
    // let result;
    // {
    //     let short = String::from("short");
    //     result = longest(string1.as_str(), &short);  // ERROR!
    // } // short is dropped here
    // println!("{}", result);  // result would point to dropped data!
    
    println!("  ✅ Lifetimes ensure references are always valid!");
}

// Lifetime in structs - struct can't outlive its references
struct Book<'a> {
    title: &'a str,  // Book must not outlive the title string
}

impl<'a> Book<'a> {
    fn get_title(&self) -> &str {
        self.title
    }
}

// -----------------------------------------------------------------------------
// PART 4: PUTTING IT ALL TOGETHER
// -----------------------------------------------------------------------------

use std::fmt::Display;

// Generic function with trait bound and lifetime
fn longest_with_summary<'a, T>(x: &'a str, y: &'a str, announcer: T) -> &'a str
where
    T: Display,  // T must be printable
{
    println!("ANNOUNCEMENT: {}", announcer);
    if x.len() > y.len() { x } else { y }
}

fn combined_example() {
    println!("\n📌 COMBINED: GENERICS + TRAITS + LIFETIMES");
    
    let s1 = "Rust";
    let s2 = "Programming";
    let announcement = "Comparing strings!";
    
    let result = longest_with_summary(s1, s2, announcement);
    println!("  Longest: {}", result);
}

// -----------------------------------------------------------------------------
// MAIN FUNCTION
// -----------------------------------------------------------------------------
fn main() {
    println!("=========================================================");
    println!("GENERICS, TRAITS & LIFETIMES - SIMPLIFIED");
    println!("=========================================================");
    
    generics_example();
    print_anything(42);
    print_anything("hello");
    
    traits_example();
    lifetimes_example();
    combined_example();
    
    println!("\n=========================================================");
    println!("📌 QUICK SUMMARY");
    println!("=========================================================");
    println!("| Concept    | What it does                    | Example                  |");
    println!("|------------|----------------------------------|--------------------------|");
    println!("| GENERICS   | Work with ANY type              | Vec<T>, Option<T>        |");
    println!("| TRAITS     | Define SHARED behavior          | .summary() on multiple   |");
    println!("| LIFETIMES  | Ensure references are VALID     | &'a str prevents dangling|");
    println!("=========================================================");
}

// =============================================================================
// ONE-LINER SUMMARY
// =============================================================================
/*
🔑 GENERICS = "I don't care what type this is"
🔑 TRAITS   = "I care WHAT it can DO, not what it IS"
🔑 LIFETIMES = "I need to know HOW LONG this lives"

Think of it this way:
- Generics = "Any type works here"
- Traits = "Any type that can do THIS works here"
- Lifetimes = "These references must be valid as long as needed"
*/