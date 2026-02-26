// =============================================================================
// REFERENCE COUNTED SMART POINTER: Rc<T>
// =============================================================================
//
// In most cases, each value in Rust has a single owner. When that owner goes
// out of scope, the value is dropped. But sometimes we need multiple parts of
// our code to share ownership of the same data. For these situations, Rust
// provides the `Rc<T>` type (Reference Counted).
//
// `Rc<T>` keeps track of the number of references to a value. When the count
// drops to zero, the value is cleaned up. It is only for single‑threaded use;
// for multithreaded code, use `Arc<T>` (Atomic Reference Counted).
//
// =============================================================================

// Suppress warnings about unused code (for demonstration purposes).
#![allow(dead_code)]

use std::rc::Rc;

// -----------------------------------------------------------------------------
// A recursive list (cons list) – perfect to illustrate the problem and solution
// -----------------------------------------------------------------------------
enum List {
    Cons(i32, Rc<List>),   // Use Rc so multiple lists can share a tail
    Nil,
}

// Bring the variants into scope for convenience
use List::{Cons, Nil};

// -----------------------------------------------------------------------------
// DEMONSTRATION: THE PROBLEM (move error)
// -----------------------------------------------------------------------------
// If we try to use `Box<List>` here, we cannot share a sublist because
// ownership is moved. This code would fail to compile:
/*
fn move_problem() {
    let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    let b = Cons(3, Box::new(a));   // `a` is moved into `b`
    let c = Cons(4, Box::new(a));   // ❌ ERROR: `a` used again after move
}
*/

// -----------------------------------------------------------------------------
// SOLUTION: USE Rc<T>
// -----------------------------------------------------------------------------
fn rc_demo() {
    // Create a list `a`: 5 -> 10 -> Nil
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));

    // Create `b` that shares `a` as its tail
    let b = Cons(3, Rc::clone(&a));   // Rc::clone only increments the reference count
    println!("count after creating b = {}", Rc::strong_count(&a));

    {
        // Create `c` that also shares `a`
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));

        // `c` goes out of scope here, reference count decreases
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}

// -----------------------------------------------------------------------------
// WHAT HAPPENS BEHIND THE SCENES?
// -----------------------------------------------------------------------------
// - `Rc::clone(&a)` does NOT deep copy the list; it only increments the
//   reference counter inside `a`. All clones point to the same heap data.
// - When an `Rc` is dropped, the counter is decremented. Only when the count
//   reaches zero is the actual data deallocated.
// - This is perfect for read‑only shared ownership in a single‑threaded context.

// -----------------------------------------------------------------------------
// NOTE: Rc IS FOR SINGLE‑THREADED USE
// -----------------------------------------------------------------------------
// `Rc` does not use atomic operations, so it is not thread‑safe. If you need
// to share ownership across threads, use `Arc<T>` instead. `Arc` has the same
// API, but uses atomic counters (slightly more expensive).

// -----------------------------------------------------------------------------
// MAIN – RUN THE DEMO
// -----------------------------------------------------------------------------
fn main() {
    println!("=== Rc<T> Demo ===");
    rc_demo();

    println!("\nThe reference count goes up with each clone and down when a clone is dropped.");
    println!("Once the last reference is gone, the list is deallocated.");
}

// =============================================================================
// EXPECTED OUTPUT:
// =============================================================================
// === Rc<T> Demo ===
// count after creating a = 1
// count after creating b = 2
// count after creating c = 3
// count after c goes out of scope = 2
//
// The reference count goes up with each clone and down when a clone is dropped.
// Once the last reference is gone, the list is deallocated.
// =============================================================================
//
// FURTHER READING:
// - `Rc<T>`: https://doc.rust-lang.org/std/rc/struct.Rc.html
// - `Arc<T>`: https://doc.rust-lang.org/std/sync/struct.Arc.html
// =============================================================================