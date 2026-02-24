// =============================================================================
// RUST VECTORS - COMPLETE GUIDE
// =============================================================================
// Vectors are like arrays that can grow/shrink. They store data on the heap.
// =============================================================================

fn main() {
    println!("=========================================================");
    println!("RUST VECTORS (Vec<T>) - ALL OPERATIONS");
    println!("=========================================================");

    // -------------------------------------------------------------------------
    // 1. CREATING VECTORS
    // -------------------------------------------------------------------------
    println!("\n📌 1. CREATING VECTORS");
    
    // Empty vector (need to specify type)
    let vec1: Vec<i32> = Vec::new();
    println!("  Empty vector: {:?}", vec1);
    
    // Using vec! macro (with initial values)
    let vec2 = vec![1, 2, 3, 4, 5];
    println!("  With initial values: {:?}", vec2);
    
    // Create with capacity (pre-allocate space)
    let mut vec3: Vec<i32> = Vec::with_capacity(10);
    println!("  With capacity 10: capacity = {}", vec3.capacity());

    // -------------------------------------------------------------------------
    // 2. ADDING AND REMOVING ELEMENTS
    // -------------------------------------------------------------------------
    println!("\n📌 2. ADDING AND REMOVING ELEMENTS");
    
    let mut numbers: Vec<i32> = Vec::new();
    
    // push() - add to end
    numbers.push(10);
    numbers.push(20);
    numbers.push(30);
    println!("  After pushes: {:?}", numbers);
    
    // pop() - remove from end (returns Option<T>)
    let last = numbers.pop();
    println!("  Popped value: {:?}", last);
    println!("  After pop: {:?}", numbers);
    
    // insert() - at specific position
    numbers.insert(1, 15);  // Insert 15 at index 1
    println!("  After insert at index 1: {:?}", numbers);
    
    // remove() - at specific position
    let removed = numbers.remove(0);  // Remove at index 0
    println!("  Removed: {}, remaining: {:?}", removed, numbers);

    // -------------------------------------------------------------------------
    // 3. READING ELEMENTS (Two ways)
    // -------------------------------------------------------------------------
    println!("\n📌 3. READING ELEMENTS");
    
    let letters = vec!['a', 'b', 'c', 'd', 'e'];
    
    // Method 1: Direct indexing (panics if out of bounds)
    let third = &letters[2];
    println!("  Direct index [2]: {}", third);
    
    // This would PANIC:
    // let tenth = &letters[9];  // ❌ panic! index out of bounds
    
    // Method 2: get() method (returns Option - safe)
    match letters.get(2) {
        Some(value) => println!("  get(2): Some({})", value),
        None => println!("  get(2): None"),
    }
    
    match letters.get(10) {
        Some(value) => println!("  get(10): Some({})", value),
        None => println!("  get(10): None (safe!)"),
    }

    // -------------------------------------------------------------------------
    // 4. ITERATING OVER VECTORS
    // -------------------------------------------------------------------------
    println!("\n📌 4. ITERATING OVER VECTORS");
    
    // Immutable iteration (read-only)
    let vec4 = vec![1, 2, 3, 4, 5];
    print!("  Immutable iteration: ");
    for i in &vec4 {
        print!("{} ", i);
    }
    println!();
    
    // Mutable iteration (change values)
    let mut vec5 = vec![10, 20, 30, 40, 50];
    for i in &mut vec5 {
        *i += 1;  // * to dereference and modify
    }
    println!("  After mutable iteration: {:?}", vec5);
    
    // Iterate with index
    println!("  With index:");
    for (index, value) in vec5.iter().enumerate() {
        println!("    [{}] = {}", index, value);
    }

    // -------------------------------------------------------------------------
    // 5. VECTOR METHODS
    // -------------------------------------------------------------------------
    println!("\n📌 5. COMMON VECTOR METHODS");
    
    let mut scores = vec![5, 3, 8, 1, 9, 4];
    
    println!("  Original: {:?}", scores);
    println!("  Length: {}", scores.len());
    println!("  Is empty? {}", scores.is_empty());
    println!("  Capacity: {}", scores.capacity());
    
    scores.sort();
    println!("  After sort: {:?}", scores);
    
    scores.reverse();
    println!("  After reverse: {:?}", scores);
    
    scores.dedup();  // Remove consecutive duplicates
    println!("  Contains 8? {}", scores.contains(&8));
    println!("  First element: {:?}", scores.first());
    println!("  Last element: {:?}", scores.last());

    // -------------------------------------------------------------------------
    // 6. STORING DIFFERENT TYPES USING ENUMS
    // -------------------------------------------------------------------------
    println!("\n📌 6. STORING MULTIPLE TYPES WITH ENUMS");
    
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    
    // Create vector with different types
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Int(6),
        SpreadsheetCell::Int(9),
    ];
    
    // Iterate and process based on type
    for (i, cell) in row.iter().enumerate() {
        print!("  Cell {}: ", i);
        match cell {
            SpreadsheetCell::Int(val) => println!("Integer {}", val),
            SpreadsheetCell::Float(val) => println!("Float {}", val),
            SpreadsheetCell::Text(val) => println!("Text '{}'", val),
        }
    }

    // -------------------------------------------------------------------------
    // 7. SLICING VECTORS
    // -------------------------------------------------------------------------
    println!("\n📌 7. SLICING VECTORS");
    
    let numbers = vec![10, 20, 30, 40, 50, 60];
    
    let slice1 = &numbers[1..4];      // indices 1,2,3
    println!("  slice [1..4]: {:?}", slice1);
    
    let slice2 = &numbers[..3];        // indices 0,1,2
    println!("  slice [..3]: {:?}", slice2);
    
    let slice3 = &numbers[3..];        // indices 3 to end
    println!("  slice [3..]: {:?}", slice3);
    
    let slice4 = &numbers[..];          // entire vector
    println!("  slice [..]: {:?}", slice4);

    // -------------------------------------------------------------------------
    // 8. CONVERTING BETWEEN VECTORS AND OTHER TYPES
    // -------------------------------------------------------------------------
    println!("\n📌 8. CONVERSIONS");
    
    // Vec to array (if you know the size)
    let vec = vec![1, 2, 3];
    let array: [i32; 3] = vec.try_into().unwrap();
    println!("  Vector to array: {:?}", array);
    
    // Vec to slice
    let slice = &vec[..];
    println!("  Vector to slice: {:?}", slice);
    
    // Iterator to Vec
    let vec_from_range: Vec<i32> = (0..5).collect();
    println!("  From range (0..5): {:?}", vec_from_range);

    // -------------------------------------------------------------------------
    // SUMMARY TABLE
    // -------------------------------------------------------------------------
    println!("\n=========================================================");
    println!("📌 VECTOR OPERATIONS QUICK REFERENCE");
    println!("=========================================================");
    println!("| Operation          | Example                          |");
    println!("|-------------------|----------------------------------|");
    println!("| Create empty      | let v: Vec<i32> = Vec::new();   |");
    println!("| Create with values| let v = vec![1, 2, 3];          |");
    println!("| Add to end        | v.push(4);                      |");
    println!("| Remove from end   | v.pop();                        |");
    println!("| Read by index     | v[2] (may panic)                |");
    println!("| Read safely       | v.get(2) (returns Option)       |");
    println!("| Iterate           | for i in &v {}                   |");
    println!("| Modify            | for i in &mut v { *i += 1; }    |");
    println!("| Length            | v.len()                         |");
    println!("| Capacity          | v.capacity()                    |");
    println!("| Check if empty    | v.is_empty()                    |");
    println!("| Sort              | v.sort();                       |");
    println!("| Reverse           | v.reverse();                    |");
    println!("| Contains          | v.contains(&5)                  |");
    println!("| Clear all         | v.clear();                      |");
    println!("=========================================================");
}

// =============================================================================
// BONUS: ERROR HANDLING WITH VECTORS
// =============================================================================
fn error_handling_example() {
    let v = vec![1, 2, 3];
    
    // Safe way to access elements
    match v.get(5) {
        Some(val) => println!("Value: {}", val),
        None => println!("Index out of bounds!"),
    }
    
    // Or use if let
    if let Some(val) = v.get(1) {
        println!("Got value: {}", val);
    }
}

// =============================================================================
// BONUS: PERFORMANCE TIPS
// =============================================================================
/*
📌 VECTOR PERFORMANCE TIPS:
--------------------------
1. Use `Vec::with_capacity(n)` if you know approximate size
2. Access is O(1) by index
3. Insert/remove at end is fast (amortized O(1))
4. Insert/remove at beginning is slow (O(n)) - consider VecDeque
5. Use `v.shrink_to_fit()` to reduce capacity after removing elements
*/