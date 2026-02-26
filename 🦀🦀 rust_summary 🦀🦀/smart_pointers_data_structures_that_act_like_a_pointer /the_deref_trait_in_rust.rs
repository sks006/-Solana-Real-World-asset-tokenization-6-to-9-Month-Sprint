// =============================================================================
// THE DEREF TRAIT – MAKING SMART POINTERS BEHAVE LIKE REGULAR REFERENCES
// =============================================================================
//
// The Deref trait allows you to customize the behavior of the dereference
// operator `*`. By implementing Deref for a smart pointer, you can make it
// usable in the same contexts as a regular reference.
//
// This is a key part of Rust’s ergonomics: you can write code that works with
// references and seamlessly use smart pointers instead.
//
// The standard library provides the Deref trait; you need to import it to
// implement it for your own types.
//
// =============================================================================

use std::ops::Deref;  // Required to implement the Deref trait

// -----------------------------------------------------------------------------
// BASICS: FOLLOWING A REFERENCE
// -----------------------------------------------------------------------------
// Before we look at smart pointers, recall how references and dereferencing work.
fn reference_basics() {
    let x = 5;
    let y = &x;               // y is a reference to x

    assert_eq!(5, x);
    assert_eq!(5, *y);         // *y follows the reference to get the value
    println!("Reference basics: x = {}, *y = {}", x, *y);
}

// -----------------------------------------------------------------------------
// DEFINING A CUSTOM SMART POINTER: MyBox<T>
// -----------------------------------------------------------------------------
// A tuple struct that holds one value of type T.
// This is a very simple smart pointer – it just stores its data.
struct MyBox<T>(T);

impl<T> MyBox<T> {
    // Associated function to create a new MyBox.
    // It takes a value and stores it directly (here, on the stack – but for
    // a real smart pointer like Box, it would allocate on the heap).
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// -----------------------------------------------------------------------------
// IMPLEMENTING THE DEREF TRAIT FOR MyBox<T>
// -----------------------------------------------------------------------------
// By implementing Deref, we tell Rust how to get a reference to the inner value
// when someone uses `*` on a MyBox. The `target` associated type specifies the
// type we are dereferencing to (here, `T`).
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0   // return a reference to the inner value
    }
}

// -----------------------------------------------------------------------------
// DEMONSTRATION: USING MyBox WITH DEREF
// -----------------------------------------------------------------------------
fn deref_demo() {
    let x = 5;
    let y = MyBox::new(x);   // y is a MyBox<i32> containing 5

    assert_eq!(5, x);
    assert_eq!(5, *y);        // *y works because MyBox implements Deref!
    println!("MyBox deref: x = {}, *y = {}", x, *y);

    // Under the hood, `*y` is actually `*(y.deref())` – Rust calls the deref
    // method and then dereferences the returned reference.
}

// -----------------------------------------------------------------------------
// DEREF COERCION
// -----------------------------------------------------------------------------
// Deref coercion is a convenience that Rust performs on arguments to functions
// and methods. It automatically converts a reference to a type that implements
// Deref into a reference to the target type.
//
// This happens automatically when a reference of type &T is passed to a function
// that expects a parameter of type &U, and T: Deref<Target=U>.

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

fn deref_coercion_demo() {
    // Create a MyBox<String> containing a String.
    let m = MyBox::new(String::from("Francesco"));

    // &m is of type &MyBox<String>. But hello expects &str.
    // Because MyBox<String> implements Deref<Target = String>, and String
    // implements Deref<Target = str>, Rust applies deref coercion:
    //   &MyBox<String> -> &String -> &str
    hello(&m);
    // Without deref coercion, we'd have to write:
    // hello(&(*m)[..]);  // extremely ugly
    println!("Deref coercion allowed us to call hello(&m) directly.");
}

// -----------------------------------------------------------------------------
// DEREF COERCION AND MUTABILITY
// -----------------------------------------------------------------------------
// Rust also has a DerefMut trait for mutable dereferencing. The coercion rules
// are expanded with mutability:
//
// 1. From &T to &U when T: Deref<Target=U>
// 2. From &mut T to &mut U when T: DerefMut<Target=U>
// 3. From &mut T to &U when T: Deref<Target=U>  (immutable coercion from mutable)
//
// This third case is useful because you can pass a mutable reference to a
// function that expects an immutable reference, provided the type implements Deref.

// Example: a mutable version of our smart pointer (we won't implement DerefMut here,
// but the concept is the same).

// -----------------------------------------------------------------------------
// MAIN – RUN ALL DEMOS
// -----------------------------------------------------------------------------
fn main() {
    println!("=== Reference Basics ===");
    reference_basics();

    println!("\n=== Deref on MyBox ===");
    deref_demo();

    println!("\n=== Deref Coercion ===");
    deref_coercion_demo();

    println!("\nAll demos finished.");
}

// =============================================================================
// SUMMARY
// =============================================================================
// - Deref allows smart pointers to be used like references.
// - Implementing Deref requires defining `Target` and `deref()`.
// - Deref coercion automatically converts &SmartPtr to &Inner when appropriate.
// - The three coercion rules ensure flexibility with mutability.
// - Deref coercion makes Rust ergonomic – you rarely need explicit dereferencing.
// =============================================================================