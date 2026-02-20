// =============================================================================
// RUST PATHS: ABSOLUTE, RELATIVE, and SUPER
// =============================================================================
// Three ways to call functions in modules:
// 1. ABSOLUTE: crate::module::function    (like /full/path)
// 2. RELATIVE: module::function           (like ./path)
// 3. SUPER:    super::function            (like ../go up)
// =============================================================================

// -----------------------------------------------------------------------------
// BASE MODULE STRUCTURE
// -----------------------------------------------------------------------------
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("✅ Added to waitlist!");
        }
    }
}

// -----------------------------------------------------------------------------
// EXAMPLE 1: ABSOLUTE vs RELATIVE PATHS
// -----------------------------------------------------------------------------
pub fn eat_at_restaurant() {
    println!("\n--- EAT AT RESTAURANT ---");
    
    // ABSOLUTE PATH: starts from crate root (always works)
    println!("  ABSOLUTE: crate::front_of_house::hosting::add_to_waitlist()");
    crate::front_of_house::hosting::add_to_waitlist();
    
    // RELATIVE PATH: starts from current module
    println!("  RELATIVE: front_of_house::hosting::add_to_waitlist()");
    front_of_house::hosting::add_to_waitlist();
}

// -----------------------------------------------------------------------------
// EXAMPLE 2: SUPER KEYWORD (going up)
// -----------------------------------------------------------------------------
fn deliver_order() {
    println!("📦 deliver_order() at ROOT level");
}

fn cook_order() {
    println!("👨‍🍳 cook_order() at ROOT level");
}

mod back_of_house {
    fn cook_order() {
        println!("   👨‍🍳 cook_order() INSIDE back_of_house module");
    }
    
    fn fix_incorrect_order() {
        println!("\n🔧 fix_incorrect_order() called:");
        
        // Calls cook_order from THIS module (same level)
        println!("  → cook_order() (no prefix) = finds in same module");
        cook_order();
        
        // Calls deliver_order from PARENT module using super
        println!("  → super::deliver_order() = goes UP to parent");
        super::deliver_order();
    }
    
    pub fn process_order() {
        fix_incorrect_order();
    }
}

// -----------------------------------------------------------------------------
// EXAMPLE 3: NESTED SUPER (going up multiple levels)
// -----------------------------------------------------------------------------
mod parent {
    pub fn parent_func() {
        println!("   📌 parent_func() at parent level");
    }
    
    pub mod child {
        pub fn child_func() {
            println!("   📌 child_func() at child level");
        }
        
        pub mod grandchild {
            pub fn grandchild_func() {
                println!("\n🔍 Inside grandchild module:");
                
                // super:: goes to child module
                println!("  → super::child_func() = up ONE level");
                super::child_func();
                
                // super::super:: goes to parent module
                println!("  → super::super::parent_func() = up TWO levels");
                super::super::parent_func();
            }
        }
    }
}

// -----------------------------------------------------------------------------
// EXAMPLE 4: PRACTICAL RESTAURANT CHAIN
// -----------------------------------------------------------------------------
mod restaurant {
    pub fn open_restaurant() {
        println!("🏪 Restaurant open for business");
    }
    
    pub mod kitchen {
        pub fn prepare_kitchen() {
            println!("🔪 Kitchen being prepared");
            
            // Call parent (restaurant)
            println!("  → super::open_restaurant() = calling parent");
            super::open_restaurant();
        }
        
        pub mod chef {
            pub fn cook() {
                println!("👨‍🍳 Chef cooking");
                
                // super:: = kitchen, super::super:: = restaurant
                println!("  → super::super::open_restaurant() = up TWO levels");
                super::super::open_restaurant();
                
                println!("  → super::prepare_kitchen() = up ONE level");
                super::prepare_kitchen();
            }
        }
    }
}

// ENUM - one 'pub' makes everything public
pub enum Appetizer {
    Soup,   // ✅ Automatically public
    Salad,  // ✅ Automatically public
}

// STRUCT - each field needs its own 'pub'
pub struct Meal {
    pub soup: Appetizer,  // ✅ Need pub here
    salad: Appetizer,      // ❌ Private even though struct is pub
}


// -----------------------------------------------------------------------------
// MAIN FUNCTION - Run all examples
// -----------------------------------------------------------------------------
fn main() {
    println!("=========================================================");
    println!("RUST PATHS COMPLETE GUIDE");
    println!("=========================================================");
    
    // Example 1: Absolute vs Relative
    println!("\n🔷 EXAMPLE 1: ABSOLUTE vs RELATIVE");
    eat_at_restaurant();
    
    // Example 2: Super keyword
    println!("\n🔷 EXAMPLE 2: SUPER KEYWORD");
    back_of_house::process_order();
    
    // Example 3: Nested super
    println!("\n🔷 EXAMPLE 3: NESTED SUPER");
    parent::child::grandchild::grandchild_func();
    
    // Example 4: Practical restaurant
    println!("\n🔷 EXAMPLE 4: PRACTICAL RESTAURANT");
    restaurant::kitchen::chef::cook();
    
    // Summary
    println!("\n=========================================================");
    println!("📌 QUICK REFERENCE");
    println!("=========================================================");
    println!("crate::module::func    → ABSOLUTE (from root)");
    println!("module::func           → RELATIVE (from current)");
    println!("super::func            → UP ONE level (parent)");
    println!("super::super::func     → UP TWO levels (grandparent)");
    println!("self::func             → CURRENT module (rarely needed)");
    let clean= Appetizer::Soup; // ✅ Enum variants are public if enum is public
    println!("Appetizer variant: {:?}", clean);
}

// =============================================================================
// VISUAL GUIDE
// =============================================================================
/*
FILE SYSTEM    |  RUST MODULES    |  MEANING
---------------+------------------+------------------
/              |  crate::         |  Root
./             |  self::          |  Current directory
../            |  super::         |  Parent directory
../../         |  super::super::  |  Grandparent directory

EXAMPLE TREE:
-------------
crate (root)
├── deliver_order()               ← super::deliver_order()
├── cook_order()
├── front_of_house/
│   └── hosting/
│       └── add_to_waitlist()     ← front_of_house::hosting::add_to_waitlist()
└── back_of_house/
    ├── cook_order()              ← cook_order() (same module)
    ├── fix_incorrect_order()     
    └── process_order()

WHICH PATH TO USE?
-----------------
✅ ABSOLUTE (crate::): When you want to be explicit and safe
✅ RELATIVE: When you're in the right location and want shorter code
✅ SUPER: When you need to access parent module from a child
*/