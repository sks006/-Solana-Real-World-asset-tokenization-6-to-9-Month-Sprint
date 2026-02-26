// =============================================================================
// REFERENCE CYCLES AND MEMORY LEAKS – USING Weak TO BREAK THE CYCLE
// =============================================================================
//
// This file demonstrates:
// 1. How to accidentally create a reference cycle using Rc + RefCell,
//    which causes a memory leak (nodes never get dropped).
// 2. How printing a cyclic structure leads to stack overflow.
// 3. How to fix the cycle by replacing one of the strong references
//    with a Weak pointer.
//
// =============================================================================

use std::rc::{Rc, Weak};
use std::cell::RefCell;

// -----------------------------------------------------------------------------
// COMMON STRUCTURE: a node that can point to another node
// -----------------------------------------------------------------------------
#[derive(Debug)]
struct Node {
    name: String,
    next: RefCell<Option<Weak<Node>>>, // we'll use Weak to avoid cycles
}

// -----------------------------------------------------------------------------
// VERSION 1: LEAKY CYCLE (using strong references both ways)
// -----------------------------------------------------------------------------
fn leaky_cycle() {
    println!("=== LEAKY CYCLE (strong references both ways) ===");

    // Create two nodes, initially with no next pointers.
    let a = Rc::new(Node {
        name: "A".to_string(),
        next: RefCell::new(None),
    });

    let b = Rc::new(Node {
        name: "B".to_string(),
        next: RefCell::new(Some(Rc::clone(&a))), // b points strongly to a
    });

    // Make a point strongly to b – this creates a cycle:
    // a -> b -> a (both strong)
    *a.next.borrow_mut() = Some(Rc::clone(&b));

    println!("a strong count = {}", Rc::strong_count(&a)); // 2
    println!("b strong count = {}", Rc::strong_count(&b)); // 2

    // The cycle means these nodes will never be dropped when they go out of scope.
    // (They would be leaked.)

    // Trying to print the structure would cause stack overflow:
    // println!("a.next = {:?}", a.next.borrow()); // ❌ stack overflow
}

// -----------------------------------------------------------------------------
// VERSION 2: FIXED WITH WEAK (break the cycle)
// -----------------------------------------------------------------------------
fn fixed_with_weak() {
    println!("\n=== FIXED WITH WEAK (one direction weakened) ===");

    let a = Rc::new(Node {
        name: "A".to_string(),
        next: RefCell::new(None),
    });

    let b = Rc::new(Node {
        name: "B".to_string(),
        next: RefCell::new(Some(Rc::clone(&a))), // b still points strongly to a
    });

    // Now make a point to b with a WEAK reference – no cycle!
    *a.next.borrow_mut() = Some(Rc::downgrade(&b));

    println!("a strong count = {}", Rc::strong_count(&a)); // 2 (one from b)
    println!("b strong count = {}", Rc::strong_count(&b)); // 1 (only itself)
    println!("a weak count   = {}", Rc::weak_count(&a));   // 0
    println!("b weak count   = {}", Rc::weak_count(&b));   // 1 (from a's next)

    // We can safely print because there is no cycle (weak reference doesn't keep b alive).
    // But we need to upgrade the Weak to a strong reference before dereferencing.
    if let Some(weak_ref) = a.next.borrow().as_ref() {
        if let Some(strong_b) = weak_ref.upgrade() {
            println!("a points to b: {:?}", strong_b.name);
        } else {
            println!("b has already been dropped");
        }
    }

    // When a and b go out of scope, they will be dropped cleanly.
}

// -----------------------------------------------------------------------------
// DEMONSTRATION OF PROPER DROPPING (adding Drop to see when nodes are freed)
// -----------------------------------------------------------------------------
impl Drop for Node {
    fn drop(&mut self) {
        println!("Dropping Node {}", self.name);
    }
}

fn main() {
    // Run the leaky version (commented because it leaks, but we can still run it)
    leaky_cycle();

    // Run the fixed version
    fixed_with_weak();

    println!("\nAll done. Check the output to see that nodes are dropped only in the fixed version.");
    // In leaky_cycle, you won't see any "Dropping Node" messages.
    // In fixed_with_weak, you'll see both nodes dropped when the function ends.
}

// =============================================================================
// EXPLANATION
// =============================================================================
// - Rc provides shared ownership via strong references. When the strong count
//   reaches zero, the value is dropped.
// - If two Rc nodes point to each other with strong references, their counts
//   never drop to zero – a memory leak occurs.
// - Weak references do not affect the strong count; they are created with
//   `Rc::downgrade` and must be upgraded to a strong reference (which may fail)
//   before use.
// - Using Weak in at least one direction breaks the cycle and allows proper
//   deallocation.
//
// =============================================================================