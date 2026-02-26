// =============================================================================
// SMART POINTERS IN RUST - COMPLETE GUIDE
// =============================================================================
//
// Smart pointers are data structures that act like pointers (hold a memory address)
// but also have additional metadata and capabilities. They implement the Deref and
// Drop traits to behave like references and manage resources automatically.
//
// Main smart pointers covered:
// - Box<T>        : heap allocation, single ownership
// - Rc<T>         : reference counting, multiple ownership (single-threaded)
// - RefCell<T>    : interior mutability with runtime borrow checking
//
// We'll also discuss interior mutability, reference cycles, and Weak<T>.
// =============================================================================

#![allow(dead_code, unused_variables)]

use std::rc::Rc;
use std::cell::RefCell;
use std::fmt::Display;

// -----------------------------------------------------------------------------
// 1. THE DEREF AND DROP TRAITS (Foundation of smart pointers)
// -----------------------------------------------------------------------------

/*
   Deref allows a smart pointer to be treated like a regular reference.
   Drop customizes cleanup when the pointer goes out of scope.
*/

// A minimal custom smart pointer (for illustration)
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> std::ops::Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("Dropping MyBox with data...");
    }
}

fn deref_drop_demo() {
    let x = MyBox::new(5);
    println!("*x = {}", *x); // deref works
    // x goes out of scope here, drop is called automatically
}

// -----------------------------------------------------------------------------
// 2. Box<T> – HEAP ALLOCATION
// -----------------------------------------------------------------------------

/*
   Box<T> stores data on the heap. It's the simplest smart pointer.
   Use cases:
   - When you have a type whose size can't be known at compile time (e.g., recursive types).
   - To transfer ownership of large data without copying.
   - For trait objects (dyn Trait).
*/

fn box_demo() {
    // Basic usage
    let b = Box::new(5);
    println!("b = {}", b); // prints 5

    // Recursive type example (cons list)
    #[derive(Debug)]
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }
    use List::{Cons, Nil};

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("{:?}", list);

    // Box<T> implements Deref, so we can use * to dereference
    let x = Box::new(42);
    assert_eq!(42, *x);
}

// -----------------------------------------------------------------------------
// 3. Rc<T> – REFERENCE COUNTING (SINGLE-THREADED MULTIPLE OWNERSHIP)
// -----------------------------------------------------------------------------

/*
   Rc<T> enables multiple ownership by keeping a reference count.
   Data is deallocated only when the count reaches zero.
   Use only in single-threaded scenarios.
*/

fn rc_demo() {
    // Creating an Rc
    let a = Rc::new(5);
    println!("reference count after creating a: {}", Rc::strong_count(&a));

    let b = Rc::clone(&a); // increases count
    println!("reference count after cloning to b: {}", Rc::strong_count(&a));

    {
        let c = Rc::clone(&a);
        println!("reference count inside inner scope: {}", Rc::strong_count(&a));
    } // c goes out of scope, count decreases

    println!("reference count after c drops: {}", Rc::strong_count(&a));
}

// -----------------------------------------------------------------------------
// 4. RefCell<T> – INTERIOR MUTABILITY
// -----------------------------------------------------------------------------

/*
   RefCell<T> allows mutation even when the RefCell itself is immutable.
   Borrowing rules are enforced at RUNTIME (if you break them, the program panics).
   Use when you know the borrowing rules are followed but the compiler can't prove it.
*/

fn refcell_demo() {
    let value = RefCell::new(5);

    // Borrow mutably through an immutable RefCell
    *value.borrow_mut() += 1;

    // Borrow immutably
    let borrowed = value.borrow();
    println!("value is now: {}", *borrowed);
}

// -----------------------------------------------------------------------------
// 5. COMBINING Rc AND RefCell – MULTIPLE OWNERS WITH MUTABILITY
// -----------------------------------------------------------------------------

/*
   Rc<RefCell<T>> is a common pattern: multiple owners can mutate the inner value.
*/

fn rc_refcell_demo() {
    let shared = Rc::new(RefCell::new(42));

    let alice = Rc::clone(&shared);
    let bob = Rc::clone(&shared);

    // Both Alice and Bob can mutate the same value
    *alice.borrow_mut() += 10;
    *bob.borrow_mut() += 5;

    println!("Final value: {}", shared.borrow()); // 57
}

// -----------------------------------------------------------------------------
// 6. INTERIOR MUTABILITY PATTERN (DETAIL)
// -----------------------------------------------------------------------------

/*
   The interior mutability pattern allows you to mutate data even when there are
   immutable references to it. RefCell is the enabler. This pattern is useful in
   cases where you need to modify data but the API only gives you immutable access.
*/

// Example: mock objects in testing
#[derive(Debug)]
struct MockMessenger {
    sent_messages: RefCell<Vec<String>>,
}

impl MockMessenger {
    fn new() -> Self {
        MockMessenger {
            sent_messages: RefCell::new(vec![]),
        }
    }

    fn send(&self, message: String) {
        // Even though self is immutable, we can modify the RefCell interior
        self.sent_messages.borrow_mut().push(message);
    }
}

fn interior_mutability_demo() {
    let mock = MockMessenger::new();
    mock.send("hello".to_string());
    mock.send("world".to_string());
    println!("{:?}", mock.sent_messages.borrow());
}

// -----------------------------------------------------------------------------
// 7. REFERENCE CYCLES AND Weak<T>
// -----------------------------------------------------------------------------

/*
   Using Rc can create reference cycles where two Rc values point to each other,
   causing memory leaks because their reference counts never drop to zero.
   Weak<T> is a version of Rc that doesn't own the value; it prevents cycles.
*/

use std::rc::Weak;

#[derive(Debug)]
enum CycleList {
    Cons(i32, RefCell<Rc<CycleList>>),
    Nil,
}

use CycleList::{Cons, Nil};

impl CycleList {
    fn tail(&self) -> Option<&RefCell<Rc<CycleList>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

fn cycle_demo() {
    // This creates a cycle: a -> b -> a
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    // Now a and b point to each other – a reference cycle!
    // Their reference counts are 2 each, never drop.
    // To avoid this, use Weak<T> (see next).
}

// Using Weak to break cycles
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn weak_demo() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    // No cycle because leaf.parent is Weak
}

// -----------------------------------------------------------------------------
// 8. SUMMARY TABLE
// -----------------------------------------------------------------------------
/*
| Smart pointer | Multiple owners? | Mutability   | Check time |
|---------------|------------------|--------------|------------|
| Box<T>        | No (single)      | immutable or | compile    |
|               |                  | mutable via  |            |
|               |                  | Box<mut T>   |            |
| Rc<T>         | Yes              | immutable    | compile    |
| RefCell<T>    | No (single)      | interior     | runtime    |
| Rc<RefCell<T>>| Yes              | interior     | runtime    |
*/

// -----------------------------------------------------------------------------
// MAIN – DEMO EVERYTHING
// -----------------------------------------------------------------------------
fn main() {
    println!("=== DEREF & DROP ===");
    deref_drop_demo();

    println!("\n=== Box<T> ===");
    box_demo();

    println!("\n=== Rc<T> ===");
    rc_demo();

    println!("\n=== RefCell<T> ===");
    refcell_demo();

    println!("\n=== Rc<RefCell<T>> ===");
    rc_refcell_demo();

    println!("\n=== Interior Mutability Pattern ===");
    interior_mutability_demo();

    println!("\n=== Reference Cycles (commented out, would leak) ===");
    // cycle_demo(); // uncomment to create a cycle (memory leak)

    println!("\n=== Weak<T> breaks cycles (no leak) ===");
    weak_demo();

    println!("\nAll examples finished.");
}

// =============================================================================
// FURTHER NOTES
// =============================================================================
/*
- `Box<T>` is the simplest; use it when you need heap allocation.
- `Rc<T>` gives shared ownership; clone it to increase reference count.
- `RefCell<T>` moves borrow checking to runtime; use with caution.
- Combine `Rc` and `RefCell` for shared, mutable data.
- Avoid reference cycles with `Weak<T>`.
- The interior mutability pattern is powerful but should be used sparingly.
*/