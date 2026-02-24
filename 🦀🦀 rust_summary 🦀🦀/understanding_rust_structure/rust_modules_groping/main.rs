// =============================================================================
// RUST MODULES - COMPLETE RESTAURANT EXAMPLE (ALL CASES)
// =============================================================================
// This shows EVERY combination of public/private modules and functions
// =============================================================================

mod restaurant {
    // =========================================================================
    // CASE 1: PUBLIC MODULE with PUBLIC FUNCTIONS (Fully accessible)
    // =========================================================================
    pub mod front_of_house {
        pub mod hosting {
            pub fn add_to_waitlist() {  // ✅ pub mod + pub fn = ACCESSIBLE
                println!("[CASE 1] ✅ Public module + Public function: Host adds to waitlist");
            }
        }
        
        // CASE 2: PUBLIC MODULE with PRIVATE FUNCTION
        fn private_serving() {  // ❌ pub mod + private fn = NOT ACCESSIBLE
            println!("[CASE 2] ❌ Public module + Private function: Secret serving method");
        }
        
        pub fn call_private_serving() {  // Public wrapper to access private function
            println!("  → Calling private function from inside the module:");
            private_serving();  // ✅ Works because we're INSIDE the module
        }
    }
    
    // =========================================================================
    // CASE 3: PRIVATE MODULE with PUBLIC FUNCTION
    // =========================================================================
    mod back_of_house {
        pub fn kitchen_operation() {  // ❌ private mod + pub fn = NOT ACCESSIBLE
            println!("[CASE 3] ❌ Private module + Public function: Kitchen running");
        }
        
        // CASE 4: PRIVATE MODULE with PRIVATE FUNCTION
        fn clean_kitchen() {  // ❌ private mod + private fn = NOT ACCESSIBLE
            println!("[CASE 4] ❌ Private module + Private function: Cleaning kitchen");
        }
        
        // Public function inside private module - still not accessible from outside!
        pub fn show_all_back_operations() {
            println!("  → Inside private module, calling all functions:");
            kitchen_operation();  // ✅ Works inside same module
            clean_kitchen();      // ✅ Works inside same module
        }
    }
    
    // =========================================================================
    // CASE 5: PUBLIC WRAPPER that accesses PRIVATE MODULE
    // =========================================================================
    pub fn access_kitchen() {  // ✅ Public function that can access private module
        println!("[CASE 5] ✅ Public wrapper accessing private module:");
        // Can call private module because we're INSIDE the restaurant module
        back_of_house::kitchen_operation();  // ✅ Works!
        // back_of_house::clean_kitchen();   // ❌ Still can't call private function!
    }
    
    // =========================================================================
    // CASE 6: NESTED MODULES - Different visibility combinations
    // =========================================================================
    pub mod outer {
        pub fn outer_public() {
            println!("[CASE 6a] ✅ outer::outer_public() - Public function in public module");
        }
        
        fn outer_private() {
            println!("[CASE 6b] ❌ outer::outer_private() - Private function");
        }
        
        pub mod inner {
            pub fn inner_public() {
                println!("[CASE 6c] ✅ outer::inner::inner_public() - Nested public function");
            }
            
            fn inner_private() {
                println!("[CASE 6d] ❌ outer::inner::inner_private() - Nested private function");
            }
            
            pub fn access_all_inner() {
                println!("  → Inside inner module, calling all functions:");
                inner_public();   // ✅ Works
                inner_private();  // ✅ Works (inside same module)
            }
        }
        
        pub fn access_all_outer() {
            println!("  → Inside outer module, calling functions:");
            outer_public();        // ✅ Works
            outer_private();       // ✅ Works (inside same module)
            inner::inner_public(); // ✅ Works (public nested)
            // inner::inner_private(); // ❌ Can't call private nested function
        }
    }
}

// =============================================================================
// MAIN FUNCTION - Testing ALL CASES
// =============================================================================
fn main() {
    println!("=================================================================");
    println!("RESTAURANT MODULE PRIVACY - TESTING ALL CASES");
    println!("=================================================================\n");

    // =========================================================================
    // CASE 1: Public module + Public function = WORKS
    // =========================================================================
    println!("--- CASE 1: Public module + Public function ---");
    restaurant::front_of_house::hosting::add_to_waitlist();
    
    // =========================================================================
    // CASE 2: Public module + Private function = FAILS
    // =========================================================================
    println!("\n--- CASE 2: Public module + Private function ---");
    println!("❌ Cannot call: restaurant::front_of_house::private_serving()");
    println!("   → Error: function `private_serving` is private");
    
    // But we CAN access it through a public wrapper inside the module
    println!("   ✅ But can access through public wrapper:");
    restaurant::front_of_house::call_private_serving();
    
    // =========================================================================
    // CASE 3: Private module + Public function = FAILS
    // =========================================================================
    println!("\n--- CASE 3: Private module + Public function ---");
    println!("❌ Cannot call: restaurant::back_of_house::kitchen_operation()");
    println!("   → Error: module `back_of_house` is private");
    
    // =========================================================================
    // CASE 4: Private module + Private function = FAILS
    // =========================================================================
    println!("\n--- CASE 4: Private module + Private function ---");
    println!("❌ Cannot call: restaurant::back_of_house::clean_kitchen()");
    println!("   → Error: module `back_of_house` is private");
    
    // But inside the private module, both work:
    println!("   ✅ Inside the private module, all functions work:");
    // Can't show directly, but CASE 5 demonstrates this
    
    // =========================================================================
    // CASE 5: Public wrapper accessing private module = WORKS
    // =========================================================================
    println!("\n--- CASE 5: Public wrapper accessing private module ---");
    restaurant::access_kitchen();
    
    // =========================================================================
    // CASE 6: Nested modules - testing all combinations
    // =========================================================================
    println!("\n--- CASE 6: Nested modules ---");
    
    // 6a: Public outer function
    restaurant::outer::outer_public();
    
    // 6b: Private outer function - FAILS
    println!("❌ Cannot call: restaurant::outer::outer_private()");
    
    // 6c: Public inner function
    restaurant::outer::inner::inner_public();
    
    // 6d: Private inner function - FAILS
    println!("❌ Cannot call: restaurant::outer::inner::inner_private()");
    
    // Access all from inside outer
    println!("\n   ✅ From inside outer module:");
    restaurant::outer::access_all_outer();
    
    // Access all from inside inner
    println!("\n   ✅ From inside inner module:");
    restaurant::outer::inner::access_all_inner();
    
    // =========================================================================
    // SUMMARY TABLE
    // =========================================================================
    println!("\n=================================================================");
    println!("PRIVACY RULES - SUMMARY TABLE");
    println!("=================================================================");
    println!("| Case | Module      | Function    | Access from main |");
    println!("|------|-------------|-------------|------------------|");
    println!("| 1    | pub mod     | pub fn      | ✅ YES           |");
    println!("| 2    | pub mod     | fn          | ❌ NO            |");
    println!("| 3    | mod         | pub fn      | ❌ NO            |");
    println!("| 4    | mod         | fn          | ❌ NO            |");
    println!("| 5    | pub wrapper | (access private) | ✅ YES    |");
    println!("| 6a   | pub mod     | pub fn (nested) | ✅ YES    |");
    println!("| 6b   | pub mod     | fn (nested)     | ❌ NO     |");
    println!("| 6c   | nested pub  | pub fn          | ✅ YES    |");
    println!("| 6d   | nested pub  | fn              | ❌ NO     |");
    println!("=================================================================");
    
    println!("\n🔑 KEY RULE: Privacy is checked at the MODULE level first!");
    println!("   Even if a function is 'pub', if its module is private,");
    println!("   you CANNOT access it from outside!");
}

// =============================================================================
// SIMPLE RULES TO REMEMBER
// =============================================================================
/*
1. MODULE PRIVACY (checked FIRST):
   - `pub mod` = module is public (can be accessed)
   - `mod` = module is private (cannot be accessed from outside)

2. FUNCTION PRIVACY (checked SECOND):
   - `pub fn` = function is public (can be called if module is accessible)
   - `fn` = function is private (only callable inside same module)

3. INSIDE THE SAME MODULE:
   - Any function can call any other function (public or private)
   - Like family members can access all rooms in the house

4. OUTSIDE THE MODULE:
   - Can only access if BOTH module AND function are public
   - Like customers can only access public areas with open counters

5. NESTED MODULES:
   - Each level checks privacy independently
   - Must be public ALL the way down to access from main!
*/