// =============================================================================
// Box<T> – THE SIMPLEST SMART POINTER IN RUST
// =============================================================================
//
// Box<T> lets you store data on the heap rather than the stack.
// The box itself (the pointer) lives on the stack, but the data it points to
// lives on the heap.
//
// Boxes have no performance overhead other than the heap allocation.
// They are useful in three main situations:
//
// 1. When you have a type whose size can't be known at compile time,
//    and you want to use a value of that type in a context that requires an
//    exact size (e.g., recursive types).
// 2. When you have a large amount of data and you want to transfer ownership
//    but ensure the data won't be copied when you do so (moving a Box is cheap).
// 3. When you want to own a value that implements a particular trait,
//    rather than a concrete type (trait objects, e.g., Box<dyn Trait>).
//
// =============================================================================

// -----------------------------------------------------------------------------
// RECURSIVE TYPE WITH Box<T> – CONS LIST EXAMPLE
// -----------------------------------------------------------------------------
// A cons list is a data structure from Lisp: a pair (element, rest of list).
// Without indirection, the compiler would not be able to compute the size of
// the enum because it would contain itself infinitely.
// By using Box<List>, we store the next element on the heap, so the enum
// variant has a known size (pointer).

enum List {
    Cons(i32, Box<List>), // The rest of the list is on the heap
    Nil,
}

// To avoid writing `List::Cons` and `List::Nil` every time, we can import them.
use List::{Cons, Nil};

fn main() {
    // -------------------------------------------------------------------------
    // CORRECT WAY TO BUILD A CONS LIST
    // -------------------------------------------------------------------------
    // Each Box::new allocates the next node on the heap.
    // The list structure is: (1, (2, (3, Nil)))
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    // We can access elements by pattern matching (not shown here).
    println!("List created successfully!");

    // -------------------------------------------------------------------------
    // COMMON MISTAKE (like in the image)
    // -------------------------------------------------------------------------
    // The following would NOT work:
    // let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Nil)))));
    //                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    // The inner Cons(3, Nil) is of type List, but Box::new expects its argument
    // to be something to box. However, the error says "expected Box<List>, found List"
    // because the third element is Nil, which is a List, but we need to wrap it in Box.
    // Always put the recursive part in Box::new.

    // -------------------------------------------------------------------------
    // SITUATION 2: TRANSFERRING OWNERSHIP OF LARGE DATA WITHOUT COPYING
    // -------------------------------------------------------------------------
    let large_data = Box::new([0u8; 1024]); // 1KB array on the heap
    let moved_data = large_data; // Only the pointer is copied, not the data

    // -------------------------------------------------------------------------
    // SITUATION 3: TRAIT OBJECTS
    // -------------------------------------------------------------------------
    trait Animal {
        fn speak(&self);
    }

    struct Dog;
    struct Cat;

    impl Animal for Dog {
        fn speak(&self) {
            println!("Woof!");
        }
    }

    impl Animal for Cat {
        fn speak(&self) {
            println!("Meow!");
        }
    }

    // Using Box<dyn Animal> to store different types that implement Animal
    let animals: Vec<Box<dyn Animal>> = vec![Box::new(Dog), Box::new(Cat)];
    for animal in animals {
        animal.speak(); // dynamic dispatch
    }
}

// =============================================================================
// WHY Box FIXES THE RECURSIVE TYPE PROBLEM
// =============================================================================
// Without Box, the compiler would try to compute the size of List:
//
//   enum List {
//       Cons(i32, List),
//       Nil,
//   }
//
// To compute the size of Cons, it needs the size of List, which leads to
// infinite recursion. Box<List> is a pointer of known size (usize),
// breaking the cycle.
//
// =============================================================================
// FURTHER READING
// =============================================================================
// - https://doc.rust-lang.org/book/ch15-01-box.html
// - https://doc.rust-lang.org/rust-by-example/std/box.html
// - https://doc.rust-lang.org/std/box/index.html
// =============================================================================