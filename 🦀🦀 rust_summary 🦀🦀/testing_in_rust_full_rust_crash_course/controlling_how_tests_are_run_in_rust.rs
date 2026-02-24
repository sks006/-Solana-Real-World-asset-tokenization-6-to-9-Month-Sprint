// =============================================================================
// TESTING IN RUST - UNIT TESTS vs INTEGRATION TESTS
// =============================================================================

// -----------------------------------------------------------------------------
// FILE: src/lib.rs - Library Code with Unit Tests
// -----------------------------------------------------------------------------

// Public function that uses a private helper
pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

// Private function - only accessible within this module
fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

// Unit tests - placed in the same file as the code
#[cfg(test)]  // Only compile when running tests
mod tests {
    use super::*;  // Import everything from parent module
    
    #[test]
    fn internal() {
        // Unit tests CAN test private functions!
        assert_eq!(4, internal_adder(2, 2));
        println!("Internal test passed!");
    }
    
    #[test]
    fn add_two_works() {
        assert_eq!(4, add_two(2));
    }
}

/*
📦 PROJECT STRUCTURE:
--------------------
adder/
├── Cargo.toml
├── src/
│   └── lib.rs              # Library code + unit tests
└── tests/
    ├── common/
    │   └── mod.rs          # Shared test utilities
    └── integration_test.rs  # Integration tests
*/

// -----------------------------------------------------------------------------
// FILE: tests/integration_test.rs - Integration Tests
// -----------------------------------------------------------------------------

// In integration tests, we need to import the library
use adder::add_two;  // Note: using the crate name, not file path

#[test]
fn it_adds_two() {
    // Integration tests can ONLY test public API
    assert_eq!(4, add_two(2));
}

#[test]
fn it_adds_two_again() {
    assert_eq!(6, add_two(4));
}

// We CANNOT test private functions here:
// fn test_private() {
//     internal_adder(2, 2);  // ❌ ERROR: private function
// }

// -----------------------------------------------------------------------------
// FILE: tests/common/mod.rs - Shared Test Utilities
// -----------------------------------------------------------------------------

// This file contains helper functions shared across multiple integration tests
pub fn setup() {
    // Setup code specific to your library's tests
    println!("Setting up tests...");
    // Initialize database, create temp files, etc.
}

// Another helper function
pub fn cleanup() {
    println!("Cleaning up after tests...");
}

// -----------------------------------------------------------------------------
// FILE: tests/integration_test.rs - Using shared utilities
// -----------------------------------------------------------------------------

use adder::add_two;

// Import the common module
mod common;  // This looks for common/mod.rs or common.rs

#[test]
fn it_adds_two_with_setup() {
    common::setup();  // Call shared setup function
    
    let result = add_two(2);
    assert_eq!(4, result);
    
    common::cleanup();  // Call shared cleanup
}

#[test]
fn another_test_with_setup() {
    common::setup();
    assert_eq!(6, add_two(4));
    common::cleanup();
}

// -----------------------------------------------------------------------------
// CARGO.TOML - Project Configuration
// -----------------------------------------------------------------------------

/*
[package]
name = "adder"
version = "0.1.0"
edition = "2021"

[dependencies]
# No dependencies for this simple example
*/

// -----------------------------------------------------------------------------
// RUNNING TESTS - COMMANDS AND OUTPUT
// -----------------------------------------------------------------------------

/*
📌 COMMAND: cargo test
---------------------
Runs ALL tests (unit + integration)

$ cargo test
Compiling adder v0.1.0
Finished test [unoptimized + debuginfo] target(s) in 0.20s

=== Running unit tests ===
Running unittests src/lib.rs
running 2 tests
test tests::internal ... ok
test tests::add_two_works ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured

=== Running integration tests ===
Running tests/integration_test.rs
running 2 tests
test it_adds_two ... ok
test it_adds_two_again ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured

=== Doc tests ===
Doc-tests adder
running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored
*/

/*
📌 COMMAND: cargo test -- --show-output
---------------------------------------
Shows println! output from tests

$ cargo test -- --show-output
... shows "Setting up tests..." messages
*/

/*
📌 COMMAND: cargo test --test integration_test
-----------------------------------------------
Run only a specific integration test file

$ cargo test --test integration_test
Running tests/integration_test.rs
running 2 tests
test it_adds_two ... ok
test it_adds_two_again ... ok
*/

/*
📌 COMMAND: cargo test it_adds_two
----------------------------------
Run tests matching a specific name

$ cargo test it_adds_two
running 1 test
test it_adds_two ... ok
*/

// -----------------------------------------------------------------------------
// COMPLETE WORKING EXAMPLE
// -----------------------------------------------------------------------------

// In real code, this would be in lib.rs
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn subtract(left: usize, right: usize) -> usize {
    private_subtract(left, right)
}

fn private_subtract(left: usize, right: usize) -> usize {
    left - right
}

#[cfg(test)]
mod unit_tests {
    use super::*;
    
    #[test]
    fn test_add() {
        assert_eq!(4, add(2, 2));
    }
    
    #[test]
    fn test_private() {
        // Unit tests CAN test private functions
        assert_eq!(3, private_subtract(5, 2));
    }
}

// In tests/integration_test.rs
/*
use my_crate::add;
use my_crate::subtract;

mod common;

#[test]
fn integration_test_add() {
    common::setup();
    assert_eq!(4, add(2, 2));
}

#[test]
fn integration_test_subtract() {
    assert_eq!(3, subtract(5, 2));
}
*/

// In tests/common/mod.rs
/*
pub fn setup() {
    println!("Setting up test environment...");
    // Initialize database connections, temp files, etc.
}

pub fn teardown() {
    println!("Cleaning up test environment...");
}
*/

// -----------------------------------------------------------------------------
// MAIN FUNCTION
// -----------------------------------------------------------------------------
fn main() {
    println!("=========================================================");
    println!("UNIT TESTS vs INTEGRATION TESTS");
    println!("=========================================================");
    
    println!("\n📌 UNIT TESTS (src/lib.rs):");
    println!("  • Live in the same file as code");
    println!("  • Can test private functions");
    println!("  • Marked with #[cfg(test)]");
    println!("  • Use 'use super::*' to import");
    
    println!("\n📌 INTEGRATION TESTS (tests/ folder):");
    println!("  • Live in separate files under tests/");
    println!("  • Can ONLY test public API");
    println!("  • Need to 'use crate_name::function'");
    println!("  • Each file is a separate crate");
    
    println!("\n📌 SHARED UTILITIES (tests/common/mod.rs):");
    println!("  • Common setup/helper functions");
    println!("  • Not compiled as separate tests");
    println!("  • Import with 'mod common;'");
    
    println!("\n📌 TEST COMMANDS:");
    println!("  cargo test                    # Run all tests");
    println!("  cargo test -- --show-output    # Show println! output");
    println!("  cargo test --test integration  # Run specific test file");
    println!("  cargo test test_name           # Run tests by name");
}

// =============================================================================
// QUICK REFERENCE
// =============================================================================
/*
📁 PROJECT STRUCTURE:
-------------------
my_crate/
├── Cargo.toml
├── src/
│   └── lib.rs              # Library code + unit tests
└── tests/
    ├── common/              # Shared test utilities
    │   └── mod.rs
    └── integration_test.rs  # Integration tests

🔑 KEY DIFFERENCES:
-----------------
| Aspect              | Unit Tests                    | Integration Tests           |
|---------------------|-------------------------------|----------------------------|
| Location            | Same file as code             | tests/ directory           |
| Private functions   | ✅ Yes                         | ❌ No                       |
| Import              | use super::*                   | use crate_name::*          |
| When to use         | Testing individual functions  | Testing public API         |
| Number per project  | Many                          | Fewer, more comprehensive  |

💡 BEST PRACTICES:
----------------
• Unit tests for private logic
• Integration tests for public API
• Shared setup in common/mod.rs
• One integration test file per feature/module
*/