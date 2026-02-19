// =====================================================
// RUST SLICES - COMPLETE BEGINNER'S GUIDE
// =====================================================
// This tutorial covers all aspects of slices in Rust
// Slices are references to a contiguous sequence of elements
// =====================================================

// =====================================================
// PART 1: BASIC SLICES WITH DIFFERENT DATA TYPES
// =====================================================
// Uncomment this section to run the examples


fn main() {
    // -------------------------------------------------
    // EXAMPLE 1: Slices with Arrays
    // -------------------------------------------------
    // An array of characters stored on the stack
    let arr: [char; 5] = ['a', 'c', 's', 'd', 'f'];
    
    // Creating a slice: &arr[start_index..end_index]
    // This takes elements from index 1 up to (but not including) index 4
    // So it includes indices: 1, 2, 3
    let slice: &[char] = &arr[1..4];  // Results in: ['c', 's', 'd']
    
    // {:?} is debug formatter - prints the slice content
    println!("Array slice (indices 1-3): {:?}", slice);
    // Output: ['c', 's', 'd']
    
    // -------------------------------------------------
    // EXAMPLE 2: Slices with Vectors
    // -------------------------------------------------
    // A vector (heap-allocated, growable array)
    let vec: Vec<i32> = vec![10, 20, 30, 40, 50];
    
    // Creating a slice of just the element at index 3
    // &vec[3..4] gives a slice containing only index 3
    let slice: &[i32] = &vec[3..4];  // Results in: [40]
    
    println!("Vector slice (index 3 only): {:?}", slice);
    // Output: [40]
    
    // -------------------------------------------------
    // EXAMPLE 3: Slices with Strings
    // -------------------------------------------------
    // A String (heap-allocated, growable UTF-8 text)
    let s: String = String::from("hello world");
    
    // String slices are of type &str (pronounced "string slice")
    let hello: &str = &s[0..5];   // Takes characters 0-4: "hello"
    let world: &str = &s[6..11];  // Takes characters 6-10: "world"
    
    println!("First word: {:?}", hello);   // Output: "hello"
    println!("Second word: {:?}", world);  // Output: "world"
    
    // -------------------------------------------------
    // EXAMPLE 4: Slice Shorthand Notation
    // -------------------------------------------------
    let s = String::from("shihabkabir");
    
    // You can omit the starting index - it defaults to 0
    let slice_from_start = &s[0..3];  // Explicit: indices 0-2
    let slice_shorthand = &s[..3];    // Same thing, shorter!
    
    // You can also omit the ending index - it goes to the end
    let slice_to_end = &s[3..];        // From index 3 to the end
    
    println!("Full string: shihabkabir");
    println!("First 3 chars: {}", slice_shorthand);     // Output: "shi"
    println!("From index 3 to end: {}", slice_to_end);  // Output: "habkabir"
    
    // The whole string as a slice: &s[..] or just &s
    let whole = &s[..];  // Slice of the entire string
}


// =====================================================
// PART 2: BUILDING A WORD FINDER - EVOLUTION OF CODE
// =====================================================
// This section shows how slices improve code step by step

// -----------------------------------------------------
// VERSION 1: WITHOUT SLICES (THE PROBLEM)
// -----------------------------------------------------
// This function returns the INDEX where the first space is found
// Problem: It gives you a number, but you still have the original string
// What if the string changes? Your index might point to wrong data!

fn first_word_index(s: &String) -> usize {
    // Convert string to bytes to check each character
    let bytes = s.as_bytes();
    
    // enumerate() gives us (index, value) pairs
    for (i, &byte) in bytes.iter().enumerate() {
        // b' ' is the byte value for space character
        if byte == b' ' {
            return i;  // Return position of first space
        }
    }
    
    // If no space found, the whole string is one word
    s.len()  // Return the length (end of string)
}


// -----------------------------------------------------
// VERSION 2: WITH SLICES (THE SOLUTION)
// -----------------------------------------------------
// This function returns the actual word, not just an index
// Much safer because the word is tied to the original data

fn first_word_slice(s: &String) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            // Return a slice from start up to the space
            return &s[0..i];  // This is the actual word!
        }
    }
    
    // No space found? Return the whole string as a slice
    &s[..]  // Shorthand for &s[0..s.len()]
}


// -----------------------------------------------------
// VERSION 3: THE BEST VERSION (MOST FLEXIBLE)
// -----------------------------------------------------
// This works with BOTH String and &str types!
// &str is more general and can accept any string-like input
fn first_word(s: &str) -> &str {
    // Convert to bytes to check for spaces
    let bytes = s.as_bytes();
    
    // Iterate through each byte with its index
    for (i, &byte) in bytes.iter().enumerate() {
        // Check if current byte is a space character
        if byte == b' ' {
            // Found a space! Return everything before it as a slice
            // &s[0..i] means: from start up to (but not including) space
            return &s[0..i];  // This is the first word!
        }
    }
    
    // If we get here, no space was found
    // The entire string is just one word - return all of it
    // &s[..] is shorthand for &s[0..s.len()]
    &s[..]  // Return the whole string as a slice
}

// -----------------------------------------------------
// MAIN FUNCTION - TESTING OUR WORD FINDER
// -----------------------------------------------------
fn main() {
    // =================================================
    // TEST 1: Working with a String (heap-allocated)
    // =================================================
    let s = String::from("hello world");
    
    // Get the first word as a slice
    let word = first_word(&s);  // We can pass &String because &String can become &str
    
    println!("=== TEST 1: Working with String ===");
    println!("Original string: \"{}\"", s);     // Output: "hello world"
    println!("First word: \"{}\"", word);        // Output: "hello"
    println!("The slice points to data inside the original string!");
    println!();
    
    // =================================================
    // TEST 2: Working with string literal (&str)
    // =================================================
    // String literals are already slices (&str)
    // They're stored directly in the binary
    let s2: &str = "second world";  // This is already a slice!
    
    // Pass the string literal directly - no & needed because it's already &str
    let word2 = first_word(s2);
    
    println!("=== TEST 2: Working with string literal ===");
    println!("Original literal: \"{}\"", s2);     // Output: "second world"
    println!("First word: \"{}\"", word2);        // Output: "second"
    println!("String literals are slices too!");
    println!();
    
    // =================================================
    // TEST 3: Single word strings (no spaces)
    // =================================================
    let single = String::from("RustProgramming");
    let word3 = first_word(&single);
    
    println!("=== TEST 3: Single word ===");
    println!("Original: \"{}\"", single);          // Output: "RustProgramming"
    println!("First word: \"{}\"", word3);         // Output: "RustProgramming"
    println!("When no space, returns entire string!");
    println!();
    
    // =================================================
    // KEY CONCEPT: WHY SLICES ARE SAFE
    // =================================================
    println!("=== KEY RUST SAFETY FEATURE ===");
    println!("1. Slices are REFERENCES - they don't own data");
    println!("2. Rust ensures original data lives as long as the slice");
    println!("3. If you try to use a slice after data is gone, Rust stops you!");
    println!("4. The borrow checker prevents many common bugs");
}

// =====================================================
// SUMMARY: SLICES AT A GLANCE
// =====================================================
/*
WHAT ARE SLICES?
- A slice is a reference to a portion of a collection
- They don't own data - they just borrow it
- Syntax: &collection[start..end] (end is exclusive)

TYPES OF SLICES:
- Array slices: &[T]
- Vector slices: &[T] (same as array slices!)
- String slices: &str

SLICE NOTATION:
- &arr[0..5]  - indices 0,1,2,3,4
- &arr[..5]    - from start to index 4
- &arr[3..]    - from index 3 to end
- &arr[..]     - entire collection

WHY USE SLICES?
- Safety: They're tied to original data lifetime
- Efficiency: No copying, just borrowing
- Flexibility: Work with different types (&str accepts both String and &str)
- Clarity: Code expresses intent (return word, not index)
*/