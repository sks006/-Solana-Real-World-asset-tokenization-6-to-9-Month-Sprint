// =============================================================================
// CLOSURES IN RUST - COMPLETE GUIDE
// =============================================================================
// Closures = anonymous functions that can capture variables from their scope
// They are like small, flexible, nameless functions
// =============================================================================

// -----------------------------------------------------------------------------
// 1. BASIC CLOSURE SYNTAX
// -----------------------------------------------------------------------------
fn basic_syntax() {
    println!("\n📌 BASIC CLOSURE SYNTAX");
    
    // Simplest closure: no parameters, returns &str
    let hello = || "Hello, World!";
    println!("  {}", hello());
    
    // Closure with parameters and type inference
    let add = |a: i32, b: i32| a + b;
    println!("  2 + 3 = {}", add(2, 3));
    
    // Closure with explicit return type (rarely needed)
    let multiply = |a: i32, b: i32| -> i32 { a * b };
    println!("  4 * 5 = {}", multiply(4, 5));
    
    // Multi-line closure with curly braces
    let complex = |x: i32| {
        let y = x * 2;
        let z = y + 10;
        z
    };
    println!("  complex(5) = {}", complex(5));
}

// -----------------------------------------------------------------------------
// 2. CLOSURES CAPTURING VARIABLES
// -----------------------------------------------------------------------------
fn capturing_examples() {
    println!("\n📌 CAPTURING VARIABLES FROM ENVIRONMENT");
    
    let x = 50;                     // i32 implements Copy
    let s = String::from("Hello");  // String does NOT implement Copy
    
    // Example 1: Immutable borrow (Fn)
    let print_x = || println!("  x = {}", x);  // x is borrowed immutably
    print_x();
    println!("  x still accessible: {}", x);    // ✅ Works
    
    // Example 2: Mutable borrow (FnMut)
    let mut y = 100;
    let mut increment_y = || {
        y += 1;                       // y is borrowed mutably
        println!("  y = {}", y);
    };
    increment_y();  // y becomes 101
    increment_y();  // y becomes 102
    // println!("{}", y);  // ❌ Can't use y here because it's mutably borrowed
    // But after last use of closure, borrow ends
    println!("  y after closure: {}", y);  // ✅ Works now
    
    // Example 3: Taking ownership (FnOnce) with move keyword
    let z = String::from("World");
    let print_z = move || {
        println!("  z = {}", z);   // z is moved into closure
        // drop(z);                 // we could consume it
    };
    print_z();
    // println!("{}", z);  // ❌ z is moved, can't use here
}

// -----------------------------------------------------------------------------
// 3. CLOSURE TRAITS: Fn, FnMut, FnOnce
// -----------------------------------------------------------------------------
/*
CLOSURE TRAITS:
---------------
- FnOnce: can be called once, may move captured values out of the closure.
- FnMut: can mutate captured values, can be called multiple times.
- Fn: can be called multiple times without mutating state.

Each closure automatically implements the most restrictive trait it needs.
*/

fn trait_examples() {
    println!("\n📌 CLOSURE TRAITS");
    
    let text = String::from("Rust");
    
    // Fn: only reads captured variables
    let fn_closure = || println!("  Fn closure: {}", text);
    fn_closure();
    fn_closure();  // can be called multiple times
    
    // FnMut: modifies captured variables
    let mut counter = 0;
    let mut fn_mut_closure = || {
        counter += 1;
        println!("  FnMut counter: {}", counter);
    };
    fn_mut_closure();
    fn_mut_closure();
    
    // FnOnce: consumes captured variables
    let data = String::from("important");
    let fn_once_closure = move || {
        println!("  FnOnce: {}", data);
        // data is dropped here
    };
    fn_once_closure();
    // fn_once_closure();  // ❌ can't call twice
}

// -----------------------------------------------------------------------------
// 4. CLOSURES AS FUNCTION PARAMETERS
// -----------------------------------------------------------------------------
// Generic function that accepts any Fn closure
fn apply<F>(f: F, value: i32) -> i32
where
    F: Fn(i32) -> i32,
{
    f(value)
}

// Accepts FnMut
fn apply_mut<F>(mut f: F, value: i32) -> i32
where
    F: FnMut(i32) -> i32,
{
    f(value)
}

// Accepts FnOnce
fn apply_once<F>(f: F, value: i32) -> i32
where
    F: FnOnce(i32) -> i32,
{
    f(value)
}

fn closure_as_parameter() {
    println!("\n📌 CLOSURES AS FUNCTION PARAMETERS");
    
    let double = |x| x * 2;
    println!("  apply(double, 10) = {}", apply(double, 10));
    
    let mut factor = 3;
    let multiply = |x| x * factor;  // captures factor immutably
    println!("  apply(multiply, 5) = {}", apply(multiply, 5));
    
    let mut accumulator = 0;
    let mut accumulate = |x| {
        accumulator += x;
        accumulator
    };
    // Need to use apply_mut because accumulate is FnMut
    println!("  apply_mut(accumulate, 4) = {}", apply_mut(accumulate, 4));
}

// -----------------------------------------------------------------------------
// 5. RETURNING CLOSURES
// -----------------------------------------------------------------------------
fn returns_closure() -> impl Fn(i32) -> i32 {
    // Closure that captures nothing (can be returned as impl Fn)
    |x| x + 1
}

fn returns_closure_with_capture() -> impl Fn(i32) -> i32 {
    let offset = 10;
    // Must use move to transfer ownership of offset to the closure
    move |x| x + offset
}

fn returning_closures() {
    println!("\n📌 RETURNING CLOSURES");
    
    let f = returns_closure();
    println!("  returns_closure()(5) = {}", f(5));
    
    let g = returns_closure_with_capture();
    println!("  returns_closure_with_capture()(5) = {}", g(5));
}

// -----------------------------------------------------------------------------
// 6. CLOSURES VS FUNCTIONS
// -----------------------------------------------------------------------------
fn regular_function(x: i32) -> i32 {
    x * 2
}

fn vs_functions() {
    println!("\n📌 CLOSURES vs FUNCTIONS");
    
    let closure = |x| x * 2;
    
    println!("  Function: {}", regular_function(5));
    println!("  Closure:  {}", closure(5));
    
    // Functions can be used where a closure is expected (if signature matches)
    let func_ptr: fn(i32) -> i32 = regular_function;
    println!("  Function pointer: {}", func_ptr(5));
    
    // But closures can capture environment, functions cannot
    let y = 10;
    let closure_captures = |x| x * y;
    println!("  Closure with capture: {}", closure_captures(5));
    // fn regular_capture(x: i32) -> i32 { x * y }  // ❌ ERROR: can't capture
}

// -----------------------------------------------------------------------------
// 7. PRACTICAL EXAMPLE: ITERATORS
// -----------------------------------------------------------------------------
fn practical_example() {
    println!("\n📌 PRACTICAL: CLOSURES WITH ITERATORS");
    
    let numbers = vec![1, 2, 3, 4, 5];
    
    // map with closure
    let doubled: Vec<i32> = numbers.iter().map(|&x| x * 2).collect();
    println!("  doubled: {:?}", doubled);
    
    // filter with closure
    let evens: Vec<&i32> = numbers.iter().filter(|&&x| x % 2 == 0).collect();
    println!("  evens: {:?}", evens);
    
    // fold with closure
    let sum = numbers.iter().fold(0, |acc, &x| acc + x);
    println!("  sum: {}", sum);
    
    // Capturing environment in iterator
    let multiplier = 3;
    let multiplied: Vec<i32> = numbers
        .iter()
        .map(|&x| x * multiplier)
        .collect();
    println!("  multiplied by {}: {:?}", multiplier, multiplied);
}

// -----------------------------------------------------------------------------
// MAIN FUNCTION
// -----------------------------------------------------------------------------
fn main() {
    println!("=========================================================");
    println!("CLOSURES IN RUST - COMPLETE GUIDE");
    println!("=========================================================");
    
    basic_syntax();
    capturing_examples();
    trait_examples();
    closure_as_parameter();
    returning_closures();
    vs_functions();
    practical_example();
    
    println!("\n=========================================================");
    println!("📌 CLOSURE TRAITS QUICK REFERENCE");
    println!("=========================================================");
    println!("| Trait   | Captures      | Called     | Example               |");
    println!("|---------|---------------|------------|-----------------------|");
    println!("| Fn      | &T            | many times | || println!(\"{}\", x) |");
    println!("| FnMut   | &mut T        | many times | || *x += 1            |");
    println!("| FnOnce  | T             | once       | move || drop(x)       |");
    println!("=========================================================");
    
    println!("\n📌 SYNTAX EXAMPLES:");
    println!("  let add = |a, b| a + b;                      // type inference");
    println!("  let add = |a: i32, b: i32| -> i32 { a + b }; // explicit types");
    println!("  let greet = || println!(\"Hi\");               // no parameters");
    println!("  let move_me = move || { /* takes ownership */ };");
}

// =============================================================================
// COMPLETE REFERENCE - CLOSURE PATTERNS
// =============================================================================
/*
🔑 KEY DIFFERENCES FROM FUNCTIONS:
---------------------------------
1. Closures can capture variables from their environment; functions cannot.
2. Closures have a more concise syntax (|args| body).
3. Closures are anonymous (can be stored in variables).
4. Closure types are inferred and implement one of Fn/FnMut/FnOnce.
5. Closures may have a runtime size cost if they capture environment.

📦 WHERE CLOSURES ARE COMMON:
----------------------------
• Iterator adapters (map, filter, fold)
• Thread spawning (move || ...)
• Callbacks and event handlers
• Configuration builders
• Lazy initialization

💡 BEST PRACTICES:
-----------------
• Use `move` when the closure needs to outlive the current scope.
• Prefer `||` without types unless inference fails.
• Use `Fn` when possible, `FnMut` when mutation needed, `FnOnce` for ownership.
• Closures are zero-cost; they usually inline.
*/