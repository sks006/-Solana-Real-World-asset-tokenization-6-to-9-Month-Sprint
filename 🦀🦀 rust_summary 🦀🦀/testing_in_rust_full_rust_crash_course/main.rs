// =============================================================================
// TESTING IN RUST - COMPLETE SUMMARY
// =============================================================================
// Three steps of testing:
// 1. Set up any needed data or state
// 2. Run the code you want to test
// 3. Assert the results are what you expect
// =============================================================================

// -----------------------------------------------------------------------------
// 1. BASIC TEST FUNCTION
// -----------------------------------------------------------------------------

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]  // Only compile when running tests
mod tests {
    use super::*;  // Import code from parent module
    
    #[test]  // This attribute marks a test function
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);  // Assert equality
    }
}

// -----------------------------------------------------------------------------
// 2. TESTING STRUCT METHODS
// -----------------------------------------------------------------------------

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod rectangle_tests {
    use super::*;
    
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        
        // assert! checks that the condition is true
        assert!(larger.can_hold(&smaller));
    }
    
    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        
        assert!(!smaller.can_hold(&larger));  // Check for false
    }
}

// -----------------------------------------------------------------------------
// 3. TESTING EQUALITY WITH assert_eq! AND assert_ne!
// -----------------------------------------------------------------------------

pub fn add_two(a: i32) -> i32 {
    a + 2  // Fixed from original (was a + 3)
}

#[cfg(test)]
mod equality_tests {
    use super::*;
    
    #[test]
    fn it_adds_two() {
        // assert_eq! checks that two values are equal
        assert_eq!(4, add_two(2));
    }
    
    #[test]
    fn it_does_not_add_three() {
        // assert_ne! checks that two values are NOT equal
        assert_ne!(5, add_two(2));
    }
}

// -----------------------------------------------------------------------------
// 4. CUSTOM FAILURE MESSAGES
// -----------------------------------------------------------------------------

pub fn greeting(name: &str) -> String {
    // This function has a bug - it doesn't use the name!
    String::from("Hello!")  // Should be: format!("Hello {name}")
}

#[cfg(test)]
mod greeting_tests {
    use super::*;
    
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        
        // Custom error message when assertion fails
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name. Expected 'Carol', got '{}'",
            result
        );
    }
}

// -----------------------------------------------------------------------------
// 5. TESTING PANICS WITH #[should_panic]
// -----------------------------------------------------------------------------

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {value}.");
        }
        
        Guess { value }
    }
}

#[cfg(test)]
mod guess_tests {
    use super::*;
    
    #[test]
    #[should_panic]  // Test passes ONLY if the code panics
    fn greater_than_100() {
        Guess::new(200);
    }
    
    #[test]
    #[should_panic]
    fn less_than_1() {
        Guess::new(0);
    }
    
    #[test]
    #[should_panic(expected = "between 1 and 100")]  // Check panic message
    fn invalid_value() {
        Guess::new(200);
    }
}

// -----------------------------------------------------------------------------
// 6. USING RESULT<T, E> IN TESTS
// -----------------------------------------------------------------------------

#[cfg(test)]
mod result_tests {
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())  // Test passes
        } else {
            Err(String::from("two plus two does not equal four"))  // Test fails
        }
    }
    
    #[test]
    fn it_fails() -> Result<(), String> {
        if 2 + 2 == 5 {
            Ok(())
        } else {
            Err(String::from("Math is broken!"))
        }
    }
}

// -----------------------------------------------------------------------------
// 7. MULTIPLE TEST FUNCTIONS TOGETHER
// -----------------------------------------------------------------------------

#[cfg(test)]
mod comprehensive_tests {
    use super::*;
    
    // Test 1: Basic equality
    #[test]
    fn test_add() {
        assert_eq!(4, add(2, 2));
        assert_eq!(10, add(5, 5));
        assert_eq!(0, add(0, 0));
    }
    
    // Test 2: Test with multiple cases
    #[test]
    fn test_rectangle() {
        let rect1 = Rectangle { width: 10, height: 10 };
        let rect2 = Rectangle { width: 5, height: 5 };
        let rect3 = Rectangle { width: 15, height: 15 };
        
        assert!(rect1.can_hold(&rect2));
        assert!(!rect2.can_hold(&rect1));
        assert!(!rect1.can_hold(&rect3));
    }
    
    // Test 3: Test with custom message
    #[test]
    fn test_greeting() {
        let result = greeting("Alice");
        assert!(
            result.contains("Alice"),
            "Greeting '{}' should contain 'Alice'",
            result
        );
    }
    
    // Test 4: Test panics
    #[test]
    #[should_panic(expected = "between 1 and 100")]
    fn test_guess_panic() {
        Guess::new(150);
    }
}

// -----------------------------------------------------------------------------
// 8. IGNORING TESTS
// -----------------------------------------------------------------------------

#[cfg(test)]
mod ignored_tests {
    #[test]
    fn test_always_passes() {
        assert_eq!(2 + 2, 4);
    }
    
    #[test]
    #[ignore]  // This test won't run unless specifically requested
    fn test_expensive_computation() {
        // Long-running test that we want to skip normally
        assert_eq!(1 + 1, 2);
    }
    
    #[test]
    #[ignore = "not implemented yet"]  // With reason
    fn test_future_feature() {
        // Test for something we haven't implemented
    }
}

// -----------------------------------------------------------------------------
// 9. RUNNING TESTS - COMMANDS
// -----------------------------------------------------------------------------
/*
TERMINAL COMMANDS:
-----------------
cargo test                    # Run all tests
cargo test -- --nocapture     # Show println! output
cargo test test_name          # Run specific test by name
cargo test -- --ignored        # Run only ignored tests
cargo test -- --test-threads=1 # Run tests in single thread (for debugging)
*/

// -----------------------------------------------------------------------------
// MAIN FUNCTION (Not needed for tests)
// -----------------------------------------------------------------------------
fn main() {
    println!("=========================================================");
    println!("TESTING IN RUST - SUMMARY");
    println!("=========================================================");
    println!("This file contains test examples.");
    println!("Run tests with: cargo test");
    
    // Demonstrate the functions
    println!("\n📌 FUNCTION OUTPUTS:");
    println!("  2 + 2 = {}", add(2, 2));
    
    let rect1 = Rectangle { width: 8, height: 7 };
    let rect2 = Rectangle { width: 5, height: 1 };
    println!("  Larger can hold smaller? {}", rect1.can_hold(&rect2));
    
    println!("\n📌 TO RUN TESTS:");
    println!("  $ cargo test");
}

// =============================================================================
// COMPLETE TEST FILE EXAMPLE (lib.rs)
// =============================================================================
/*
// In lib.rs or main.rs

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_add() {
        assert_eq!(add(2, 2), 4);
    }
    
    #[test]
    fn test_add_negative() {
        assert_eq!(add(-2, 2), 0);
    }
    
    #[test]
    #[should_panic]
    fn test_panic() {
        panic!("This test should panic");
    }
    
    #[test]
    fn test_result() -> Result<(), String> {
        if add(2, 2) == 4 {
            Ok(())
        } else {
            Err(String::from("add doesn't work"))
        }
    }
}
*/

// =============================================================================
// QUICK REFERENCE
// =============================================================================
/*
📌 TEST ATTRIBUTES:
------------------
#[test]              // Mark function as test
#[should_panic]      // Test passes if code panics
#[ignore]            // Skip this test
#[cfg(test)]         // Only compile when testing

📌 ASSERTION MACROS:
------------------
assert!(condition)                    // Panic if false
assert_eq!(a, b)                       // Panic if a != b
assert_ne!(a, b)                       // Panic if a == b
assert!(cond, "message {}", value)     // With custom message
assert_eq!(a, b, "message {}", value)  // With custom message

📌 TEST ORGANIZATION:
------------------
// Unit tests: in same file with #[cfg(test)]
// Integration tests: in tests/ directory

📌 TEST OUTPUT:
-------------
ok - test passed
FAILED - test failed
ignored - test skipped
*/