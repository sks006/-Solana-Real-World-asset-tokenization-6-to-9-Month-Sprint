// =============================================================================
// RUST TRAITS - COMPLETE SUMMARY
// =============================================================================
// Traits = defining shared behavior (like interfaces in other languages)
// =============================================================================

// -----------------------------------------------------------------------------
// 1. DEFINING A TRAIT
// -----------------------------------------------------------------------------
pub trait Summary {
    fn summarize(&self) -> String;  // Method signature (no implementation)
    
    // Can also provide default implementation
    fn summarize_author(&self) -> String {
        String::from("(Unknown author)")
    }
}

// -----------------------------------------------------------------------------
// 2. IMPLEMENTING A TRAIT ON TYPES
// -----------------------------------------------------------------------------

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// Implement Summary for NewsArticle
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
    
    // Override default implementation
    fn summarize_author(&self) -> String {
        format!("Author: {}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// Implement Summary for Tweet
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
    
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// -----------------------------------------------------------------------------
// 3. TRAITS AS PARAMETERS (Different syntaxes)
// -----------------------------------------------------------------------------

// Syntax 1: impl Trait (simplest)
pub fn notify1(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// Syntax 2: Generic with trait bound (more explicit)
pub fn notify2<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// Multiple parameters of same type
pub fn notify_same<T: Summary>(item1: &T, item2: &T) {
    println!("1: {}", item1.summarize());
    println!("2: {}", item2.summarize());
}

// Multiple parameters of possibly different types
pub fn notify_different(item1: &impl Summary, item2: &impl Summary) {
    println!("1: {}", item1.summarize());
    println!("2: {}", item2.summarize());
}

// -----------------------------------------------------------------------------
// 4. MULTIPLE TRAIT BOUNDS
// -----------------------------------------------------------------------------

use std::fmt::Display;

// Require type to implement BOTH Summary and Display
pub fn notify_display(item: &(impl Summary + Display)) {
    println!("Display: {}", item);
    println!("Summary: {}", item.summarize());
}

// With generic syntax
pub fn notify_display_gen<T: Summary + Display>(item: &T) {
    println!("Display: {}", item);
    println!("Summary: {}", item.summarize());
}

// Where clause (cleaner for complex bounds)
pub fn notify_complex<T>(item: &T)
where
    T: Summary + Display + Clone,
{
    let cloned = item.clone();
    println!("Original: {}", item.summarize());
    println!("Cloned: {}", cloned.summarize());
}

// -----------------------------------------------------------------------------
// 5. RETURNING TYPES THAT IMPLEMENT TRAITS
// -----------------------------------------------------------------------------

// Can return any type that implements Summary
fn returns_summarizable(switch: bool) -> impl Summary {
    if switch {
        NewsArticle {
            headline: String::from("Penguins win the Stanley Cup Championship!"),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from("The Pittsburgh Penguins are the best!"),
        }
    } else {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        }
    }
    // ❌ ERROR: Can only return ONE concrete type!
    // if and else must return the same type
}

// This works - returns same type
fn returns_tweet() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("Hello world"),
        reply: false,
        retweet: false,
    }
}

// -----------------------------------------------------------------------------
// 6. CONDITIONAL IMPLEMENTATIONS (Trait Bounds)
// -----------------------------------------------------------------------------

struct Pair<T> {
    x: T,
    y: T,
}

// Implement for ALL T
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// Implement ONLY for T that implements Display + PartialOrd
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// -----------------------------------------------------------------------------
// 7. BLANKET IMPLEMENTATIONS (Implement trait for any type that satisfies bounds)
// -----------------------------------------------------------------------------

// This is how Rust implements ToString for any type that implements Display
// impl<T: Display> ToString for T {
//     fn to_string(&self) -> String {
//         // ...
//     }
// }

// -----------------------------------------------------------------------------
// 8. MAIN FUNCTION - TESTING ALL CONCEPTS
// -----------------------------------------------------------------------------
fn main() {
    println!("=========================================================");
    println!("RUST TRAITS - COMPLETE SUMMARY");
    println!("=========================================================");
    
    // Create instances
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins are the best!"),
    };
    
    // Call trait methods
    println!("\n📌 TRAIT METHODS");
    println!("  Tweet: {}", tweet.summarize());
    println!("  Article: {}", article.summarize());
    println!("  Tweet author: {}", tweet.summarize_author());
    println!("  Article author: {}", article.summarize_author());
    
    // Functions with trait parameters
    println!("\n📌 TRAITS AS PARAMETERS");
    notify1(&tweet);
    notify2(&article);
    notify_same(&tweet, &tweet);
    notify_different(&tweet, &article);
    
    // Returning impl Trait
    println!("\n📌 RETURNING IMPL TRAIT");
    let returned = returns_tweet();
    println!("  Returned: {}", returned.summarize());
    
    // Conditional implementations
    println!("\n📌 CONDITIONAL IMPLEMENTATIONS");
    let pair_int = Pair::new(5, 10);
    let pair_str = Pair::new("apple", "zebra");
    
    pair_int.cmp_display();  // Works because i32 implements Display + PartialOrd
    pair_str.cmp_display();  // Works because &str implements Display + PartialOrd
    
    // Pair with type that doesn't implement Display would NOT have cmp_display
    // struct NoDisplay;
    // let pair_no = Pair::new(NoDisplay, NoDisplay);
    // pair_no.cmp_display();  // ❌ ERROR: NoDisplay doesn't implement Display
    
    println!("\n=========================================================");
    println!("📌 TRAITS QUICK REFERENCE");
    println!("=========================================================");
    println!("| Concept                | Syntax                          |");
    println!("|------------------------|----------------------------------|");
    println!("| Define trait           | trait Name {{ fn method(&self); }} |");
    println!("| Implement trait        | impl Name for Type {{ ... }}     |");
    println!("| Trait parameter (impl) | fn f(item: &impl Trait)         |");
    println!("| Trait parameter (gen)  | fn f<T: Trait>(item: &T)        |");
    println!("| Multiple bounds        | fn f(item: &(impl A + B))       |");
    println!("| Where clause           | fn f<T>(item: &T) where T: Trait|");
    println!("| Return impl Trait      | fn f() -> impl Trait            |");
    println!("| Conditional impl       | impl<T: Bound> Type<T> {{ ... }} |");
    println!("=========================================================");
}

// =============================================================================
// SUMMARY - ONE-LINERS
// =============================================================================
/*
🔑 TRAIT = "This type CAN DO these things"
🔑 IMPL TRAIT = "I don't care what type it is, as long as it can do THIS"
🔑 TRAIT BOUND = "T must be able to do THIS"

Think of traits as:
- Like interfaces in other languages
- A way to say "any type that has this behavior"
- A contract between the type and the caller
*/