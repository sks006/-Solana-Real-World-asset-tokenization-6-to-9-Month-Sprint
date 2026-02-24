// =============================================================================
// ADVANCED TESTING IN RUST - CONTROLLING TEST EXECUTION
// =============================================================================

// -----------------------------------------------------------------------------
// 1. TEST FUNCTION WITH PRINTING
// -----------------------------------------------------------------------------

fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }

    #[test]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(5, value);  // This will fail - expects 5 but gets 10
    }
}

// -----------------------------------------------------------------------------
// 2. CONTROLLING TEST OUTPUT
// -----------------------------------------------------------------------------

/*
COMMAND: cargo test -- --show-output
------------------------------------
Shows println! output even for passing tests

$ cargo test -- --show-output

Running unittests
test tests::this_test_will_pass ... ok
--- stdout: I got the value 4

test tests::this_test_will_fail ... FAILED

failures:
---- tests::this_test_will_fail stdout ----
I got the value 8
thread 'tests::this_test_will_fail' panicked at 'assertion failed: `(left == right)`
  left: `5`,
 right: `10`', src/lib.rs:...

Without --show-output, println! only shows for failing tests
*/

// -----------------------------------------------------------------------------
// 3. RUNNING TESTS IN SINGLE THREAD
// -----------------------------------------------------------------------------

/*
COMMAND: cargo test -- --test-threads=1
---------------------------------------
Runs tests one at a time (not in parallel)
Useful for debugging or when tests share state

$ cargo test -- --test-threads=1

Running tests sequentially...
test tests::this_test_will_pass ... ok
test tests::this_test_will_fail ... FAILED
*/

// -----------------------------------------------------------------------------
// 4. RUNNING SPECIFIC TESTS BY NAME
// -----------------------------------------------------------------------------

pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod math_tests {
    use super::*;

    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }

    #[test]
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }
}

/*
COMMAND: cargo test one_hundred
-------------------------------
Runs only tests that contain "one_hundred" in the name

$ cargo test one_hundred

running 1 test
test math_tests::one_hundred ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out
*/

/*
COMMAND: cargo test two
-----------------------
Runs all tests that contain "two" in the name

$ cargo test two

running 2 tests
test math_tests::add_three_and_two ... ok
test math_tests::add_two_and_two ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 1 filtered out
*/

// -----------------------------------------------------------------------------
// 5. IGNORING TESTS
// -----------------------------------------------------------------------------

#[cfg(test)]
mod ignored_tests {
    use super::*;

    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    #[ignore]  // This test will be skipped by default
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }

    #[test]
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }
    
    #[test]
    #[ignore = "not implemented yet"]  // With reason
    fn future_feature() {
        // Test for something not yet implemented
    }
}

/*
COMMAND: cargo test (without --ignored)
--------------------------------------
Skipped ignored tests

$ cargo test

running 2 tests
test ignored_tests::add_two_and_two ... ok
test ignored_tests::one_hundred ... ok
test ignored_tests::add_three_and_two ... ignored

test result: ok. 2 passed; 0 failed; 1 ignored; 0 measured
*/

/*
COMMAND: cargo test -- --ignored
--------------------------------
Runs ONLY the ignored tests

$ cargo test -- --ignored

running 1 test
test ignored_tests::add_three_and_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out
*/

// -----------------------------------------------------------------------------
// 6. COMPLETE EXAMPLE WITH ALL FEATURES
// -----------------------------------------------------------------------------

pub fn complex_function(input: i32) -> i32 {
    println!("Processing input: {}", input);
    input * 2
}

#[cfg(test)]
mod complete_tests {
    use super::*;

    // Regular test
    #[test]
    fn test_normal_case() {
        assert_eq!(10, complex_function(5));
    }

    // Test that will fail (for demonstration)
    #[test]
    fn test_failing_case() {
        assert_eq!(20, complex_function(5));  // Fails: 10 != 20
    }

    // Ignored test
    #[test]
    #[ignore]
    fn test_slow_operation() {
        // Simulate slow test
        std::thread::sleep(std::time::Duration::from_secs(5));
        assert_eq!(100, complex_function(50));
    }

    // Test with custom message
    #[test]
    fn test_with_message() {
        let result = complex_function(7);
        assert!(
            result == 14,
            "Expected 14 but got {} when processing 7",
            result
        );
    }
}

// -----------------------------------------------------------------------------
// 7. TEST COMMANDS CHEAT SHEET
// -----------------------------------------------------------------------------

/*
╔══════════════════════════════╦════════════════════════════════════════════╗
║ COMMAND                      ║ DESCRIPTION                                ║
╠══════════════════════════════╬════════════════════════════════════════════╣
║ cargo test                   ║ Run all tests                              ║
║ cargo test -- --show-output  ║ Show println! output for all tests         ║
║ cargo test -- --test-threads=1║ Run tests sequentially (not parallel)     ║
║ cargo test test_name         ║ Run tests containing "test_name"           ║
║ cargo test -- --ignored       ║ Run only ignored tests                     ║
║ cargo test -- --include-ignored║ Run all tests including ignored           ║
║ cargo test -- --nocapture     ║ Show output even for passing tests        ║
╚══════════════════════════════╩════════════════════════════════════════════╝
*/

// -----------------------------------------------------------------------------
// MAIN FUNCTION
// -----------------------------------------------------------------------------
fn main() {
    println!("=========================================================");
    println!("ADVANCED TESTING IN RUST");
    println!("=========================================================");
    
    println!("\n📌 TEST COMMANDS:");
    println!("  • cargo test                    # Run all tests");
    println!("  • cargo test -- --show-output    # Show println! output");
    println!("  • cargo test -- --test-threads=1 # Run sequentially");
    println!("  • cargo test one_hundred         # Run specific test");
    println!("  • cargo test -- --ignored         # Run ignored tests");
    
    println!("\n📌 TEST ATTRIBUTES:");
    println!("  • #[test]        - Mark as test");
    println!("  • #[ignore]      - Skip test by default");
    println!("  • #[ignore = \"reason\"] - Skip with reason");
    
    println!("\n📌 EXAMPLE OUTPUTS:");
    println!("  running 3 tests");
    println!("  test tests::add_two_and_two ... ok");
    println!("  test tests::add_three_and_two ... ignored");
    println!("  test tests::one_hundred ... ok");
    println!();
    println!("  test result: ok. 2 passed; 0 failed; 1 ignored");
}

// =============================================================================
// QUICK REFERENCE CARD
// =============================================================================
/*
🔑 KEY CONCEPTS:
---------------
1. Filter tests by name: cargo test <pattern>
2. Show all output: --show-output
3. Control parallelism: --test-threads=1
4. Skip tests: #[ignore]
5. Run ignored: --ignored

📊 TEST RUN EXAMPLES:
-------------------
$ cargo test
→ Runs all non-ignored tests

$ cargo test add
→ Runs tests with "add" in name (add_two_and_two, add_three_and_two)

$ cargo test -- --ignored
→ Runs only #[ignore] tests

$ cargo test -- --show-output
→ Shows println! statements from all tests

💡 TIPS:
------
• Use #[ignore] for slow or broken tests
• Use --test-threads=1 when tests share state
• Use test names to focus on specific functionality
• println! only shows for failed tests unless --show-output
*/