// =============================================================================
// RUST MODULES & PRIVACY - Simple Explanation
// =============================================================================
// In Rust:
// - `pub` = public (can be used outside the module)
// - No `pub` = private (only usable inside the module)
// =============================================================================

// Declaring a public module named 'network'
pub mod network {
    // PRIVATE function - only usable inside this module
    // (no 'pub' keyword means it's private)
    fn connect() {
        println!("Connection established.");
    }

    // PUBLIC function - usable from outside the module
    // (has 'pub' keyword)
    pub fn initiate_connection() {
        println!("Initiating connection...");
        connect(); // ✅ OK: private function called from inside the module
    }
}

fn main() {
    // ✅ THIS WORKS: calling a PUBLIC function
    // We can call initiate_connection() because it's marked 'pub'
    println!("--- Calling public function ---");
    network::initiate_connection();
    
    // ❌ THIS WOULD FAIL: calling a PRIVATE function
    // The line below would cause a compile error if uncommented
    // network::connect(); // ERROR: function `connect` is private
    
    println!("\n--- Privacy Rules ---");
    println!("✅ Public functions (with 'pub') can be called from anywhere");
    println!("❌ Private functions (no 'pub') can only be called inside their module");
}

// =============================================================================
// OUTPUT:
// --- Calling public function ---
// Initiating connection...
// Connection established.
//
// --- Privacy Rules ---
// ✅ Public functions (with 'pub') can be called from anywhere
// ❌ Private functions (no 'pub') can only be called inside their module
// =============================================================================

// =============================================================================
// MORE EXAMPLES TO UNDERSTAND MODULES
// =============================================================================

// Example 2: Multiple functions with different visibility
pub mod calculator {
    // Private helper function
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }
    
    // Private helper function
    fn multiply(a: i32, b: i32) -> i32 {
        a * b
    }
    
    // Public function that uses private helpers
    pub fn calculate_sum_and_product(x: i32, y: i32) {
        let sum = add(x, y);           // ✅ private function called inside module
        let product = multiply(x, y);   // ✅ private function called inside module
        println!("Sum: {}, Product: {}", sum, product);
    }
    
    // Public function - just returns sum
    pub fn get_sum(x: i32, y: i32) -> i32 {
        add(x, y)  // ✅ using private helper
    }
}

// Example 3: Nested modules
pub mod outer {
    // Private function in outer module
    fn outer_private() {
        println!("Outer private function");
    }
    
    pub mod inner {
        // Private function in inner module
        fn inner_private() {
            println!("Inner private function");
        }
        
        // Public function in inner module
        pub fn inner_public() {
            println!("Inner public function");
            inner_private(); // ✅ OK: calling private from same module
            // outer_private(); // ❌ ERROR: can't call outer's private function
        }
    }
    
    pub fn outer_public() {
        println!("Outer public function");
        outer_private();     // ✅ OK: calling private from same module
        inner::inner_public(); // ✅ OK: calling public from inner module
        // inner::inner_private(); // ❌ ERROR: inner_private is private
    }
}

fn more_examples() {
    println!("\n=== MORE EXAMPLES ===");
    
    // Using calculator module
    calculator::calculate_sum_and_product(5, 3);
    let sum = calculator::get_sum(10, 20);
    println!("Sum only: {}", sum);
    // calculator::add(1,2); // ❌ ERROR: add is private!
    
    // Using nested modules
    outer::outer_public();
    outer::inner::inner_public(); // ✅ Can call public inner function
    // outer::inner::inner_private(); // ❌ ERROR: private!
}

// Uncomment to run more examples
// fn main() {
//     more_examples();
// }

// =============================================================================
// QUICK REFERENCE
// =============================================================================
/*
VISIBILITY RULES:
----------------
pub fn name()      → Can be called from anywhere
fn name()          → Can ONLY be called inside its own module

INSIDE A MODULE:
- Can call any function (public or private) from the SAME module
- Can call PUBLIC functions from OUTSIDE the module
- Can NOT call PRIVATE functions from OUTSIDE the module

NESTED MODULES:
- Child modules can call parent's PUBLIC functions
- Child modules CANNOT call parent's PRIVATE functions
- Parent modules can call child's PUBLIC functions
- Parent modules CANNOT call child's PRIVATE functions

KEY POINT:
Private = "For internal use only within this module"
Public = "Available for everyone to use"
*/