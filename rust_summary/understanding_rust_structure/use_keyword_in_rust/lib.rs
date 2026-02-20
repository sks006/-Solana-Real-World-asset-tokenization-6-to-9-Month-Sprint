// =============================================================================
// RUST 'USE' KEYWORD - ALL PATTERNS SIMPLIFIED
// =============================================================================

// -----------------------------------------------------------------------------
// 1. BASIC USE - Bringing paths into scope
// -----------------------------------------------------------------------------
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() { println!("✅ Added!"); }
    }
}

// Without 'use': need full path every time
fn without_use() {
    crate::front_of_house::hosting::add_to_waitlist();
}

// With 'use': create a shortcut
use crate::front_of_house::hosting;

fn with_use() {
    hosting::add_to_waitlist();  // Much shorter!
}

// -----------------------------------------------------------------------------
// 2. USE INSIDE MODULES
// -----------------------------------------------------------------------------
mod restaurant {
    use crate::front_of_house::hosting;  // 'use' inside module
    
    pub fn eat() {
        hosting::add_to_waitlist();  // Works because of 'use' above
    }
}

// -----------------------------------------------------------------------------
// 3. DIFFERENT WAYS TO USE 'USE'
// -----------------------------------------------------------------------------

// A. Import module (preferred for functions)
use std::collections::HashMap;        // Import type directly
use std::io;                           // Import module

// B. Import specific function (less common)
use std::io::stdin;                    // Can call stdin() directly

// C. Import enum variants
#[derive(Debug)]
enum Color { Red, Green, Blue }

use Color::{Red, Green};               // Now use Red, Green directly

fn enum_example() {
    let c1 = Red;                       // Instead of Color::Red
    let c2 = Green;                      // Instead of Color::Green
}

// D. Import everything (*) - use carefully!
use std::collections::*;                 // Imports HashMap, HashSet, etc.

fn glob_example() {
    let map = HashMap::new();             // Works!
    let set = HashSet::new();             // Works!
}

// E. Rename with 'as' (for conflicts)
use std::fmt::Result as FmtResult;       // Rename to avoid conflicts
use std::io::Result as IoResult;

// F. Re-export with 'pub use' (make internal items public)
mod kitchen {
    pub fn cook() { println!("Cooking..."); }
}
pub use kitchen::cook;                    // Now cook() is public at crate root

// -----------------------------------------------------------------------------
// 4. NESTED PATHS (clean up multiple imports)
// -----------------------------------------------------------------------------

// Instead of:
// use std::io;
// use std::fmt;
// use std::fs::File;

// Do this:
use std::{
    io,
    fmt,
    fs::File,
};

// Even deeper nesting
use std::collections::{
    HashMap,
    HashSet,
    VecDeque,
};

// Mix modules and items
use std::io::{
    self,        // Bring io module
    Read,        // Bring Read trait
    Write,       // Bring Write trait
};

// -----------------------------------------------------------------------------
// 5. HANDLING NAME CONFLICTS
// -----------------------------------------------------------------------------

// Both std::fmt and std::io have a type called 'Result'
// Solution 1: Import modules
use std::fmt;
use std::io;

fn solution1() -> fmt::Result { Ok(()) }     // Clear which Result
fn solution2() -> io::Result<()> { Ok(()) }  // Clear which Result

// Solution 2: Rename with 'as'
use std::fmt::Result as FmtResult;
use std::io::Result as IoResult;

fn solution3() -> FmtResult { Ok(()) }        // fmt::Result
fn solution4() -> IoResult<()> { Ok(()) }     // io::Result

// -----------------------------------------------------------------------------
// 6. RE-EXPORTING (pub use) - Create public API
// -----------------------------------------------------------------------------

// Hide internal structure
mod internal {
    pub mod helpers {
        pub fn do_thing() { println!("Done!"); }
    }
}

// Re-export only what users need
pub use internal::helpers::do_thing;

// Users just do:
// use my_crate::do_thing;  // Simple!

// -----------------------------------------------------------------------------
// 7. COMMON PATTERNS QUICK REFERENCE
// -----------------------------------------------------------------------------

/*
┌─────────────────┬────────────────────────────────────┐
│ Pattern         │ Example                            │
├─────────────────┼────────────────────────────────────┤
│ Import module   │ use std::io;                        │
│ Import type     │ use std::collections::HashMap;      │
│ Import function │ use std::io::stdin;                  │
│ Import enum     │ use Color::{Red, Green};            │
│ Import all *    │ use std::collections::*;            │
│ Rename 'as'     │ use std::fmt::Result as FmtResult;  │
│ Nested {}       │ use std::{io, fmt};                  │
│ Re-export pub   │ pub use path::to::item;             │
│ Self in nested  │ use std::io::{self, Read};          │
└─────────────────┴────────────────────────────────────┘
*/

// -----------------------------------------------------------------------------
// 8. WHERE TO PUT 'USE'
// -----------------------------------------------------------------------------

// At the top of file (most common)
use std::fmt;
use std::io;

// Inside a function (rare, limited scope)
fn example() {
    use std::collections::HashMap;  // Only works inside this function
    let map = HashMap::new();
}

// Inside a module (only affects that module)
mod my_mod {
    use std::fmt;  // Only affects code inside my_mod
}

// -----------------------------------------------------------------------------
// 9. TEST YOUR KNOWLEDGE
// -----------------------------------------------------------------------------
fn main() {
    println!("========================================");
    println!("RUST 'USE' KEYWORD - SIMPLE REFERENCE");
    println!("========================================");
    println!();
    println!("✅ USE = Create a shortcut to a path");
    println!("✅ pub USE = Make imported item public");
    println!("✅ {{}} = Import multiple items at once");
    println!("✅ * = Import everything (use carefully)");
    println!("✅ as = Rename imported item");
    println!();
    println!("📌 REMEMBER:");
    println!("  • Prefer importing modules for functions");
    println!("  • Prefer importing types directly for structs/enums");
    println!("  • Use nested paths {{}} to keep code clean");
    println!("  • Use 'as' to resolve naming conflicts");
    println!("  • Use 'pub use' to create clean public APIs");
}

// =============================================================================
// END - Simple Reference
// =============================================================================