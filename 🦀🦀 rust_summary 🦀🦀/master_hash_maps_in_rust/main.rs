// =============================================================================
// RUST HASHMAPS - COMPLETE GUIDE
// =============================================================================
// HashMap<K, V> stores key-value pairs for fast lookup by key.
// Keys are unique, values can be duplicated.
// =============================================================================

use std::collections::HashMap;

fn main() {
    println!("=========================================================");
    println!("RUST HASHMAPS - ALL OPERATIONS");
    println!("=========================================================");

    // -------------------------------------------------------------------------
    // 1. CREATING HASHMAPS
    // -------------------------------------------------------------------------
    println!("\n📌 1. CREATING HASHMAPS");
    
    // Method 1: new() then insert
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("  scores: {:?}", scores);
    
    // Method 2: from iterators (collect)
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores2: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    println!("  from iterators: {:?}", scores2);
    
    // Method 3: with capacity
    let mut with_cap: HashMap<String, i32> = HashMap::with_capacity(10);
    println!("  with capacity: capacity = {}", with_cap.capacity());

    // -------------------------------------------------------------------------
    // 2. ACCESSING VALUES
    // -------------------------------------------------------------------------
    println!("\n📌 2. ACCESSING VALUES");
    
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    
    // Method 1: get() - returns Option<&V>
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);  // Returns Option<&i32>
    println!("  get('Blue'): {:?}", score);
    
    // Common pattern: get with copied() and unwrap_or
    let score_value = scores.get(&team_name).copied().unwrap_or(0);
    println!("  Blue team score: {}", score_value);
    
    // Non-existent key
    let red_score = scores.get(&String::from("Red")).copied().unwrap_or(0);
    println!("  Red team score: {}", red_score);
    
    // Method 2: Direct indexing (panics if key missing!)
    // let blue = scores[&String::from("Blue")];  // Works
    // let red = scores[&String::from("Red")];    // ❌ PANIC!

    // -------------------------------------------------------------------------
    // 3. ITERATING OVER HASHMAPS
    // -------------------------------------------------------------------------
    println!("\n📌 3. ITERATING OVER HASHMAPS");
    
    let mut languages = HashMap::new();
    languages.insert(String::from("Rust"), 1);
    languages.insert(String::from("Python"), 2);
    languages.insert(String::from("Java"), 3);
    languages.insert(String::from("C++"), 4);
    languages.insert(String::from("JavaScript"), 5);
    
    println!("  All languages (order not guaranteed):");
    for (key, value) in &languages {
        println!("    {}: {}", key, value);
    }

    // -------------------------------------------------------------------------
    // 4. OWNERSHIP AND HASHMAPS
    // -------------------------------------------------------------------------
    println!("\n📌 4. OWNERSHIP WITH HASHMAPS");
    
    let mut map = HashMap::new();
    
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    
    // CASE 1: Insert owned values (ownership moves)
    map.insert(field_name, field_value);
    // println!("{}", field_name);  // ❌ ERROR: field_name moved!
    // println!("{}", field_value); // ❌ ERROR: field_value moved!
    println!("  After insert (ownership moved): {:?}", map);
    
    // CASE 2: Insert references (borrowing)
    let mut map_ref = HashMap::new();
    let name = String::from("Age");
    let value = 30;
    
    map_ref.insert(&name, &value);  // Store references
    println!("  After insert (references): {:?}", map_ref);
    println!("  Original values still usable: name='{}', value={}", name, value);
    
    // CASE 3: With primitive types (Copy, so ownership not moved)
    let mut map_copy = HashMap::new();
    let num_key = 42;
    let num_val = 100;
    map_copy.insert(num_key, num_val);  // i32 implements Copy
    println!("  With Copy types: {:?}", map_copy);
    println!("  Original still usable: key={}, val={}", num_key, num_val);

    // -------------------------------------------------------------------------
    // 5. UPDATING HASHMAPS
    // -------------------------------------------------------------------------
    println!("\n📌 5. UPDATING HASHMAPS");
    
    let mut map_update = HashMap::new();
    map_update.insert(String::from("Blue"), 10);
    println!("  Initial: {:?}", map_update);
    
    // Overwrite existing value
    map_update.insert(String::from("Blue"), 25);
    println!("  After overwrite: {:?}", map_update);
    
    // entry() + or_insert() - insert only if key doesn't exist
    map_update.entry(String::from("Yellow")).or_insert(50);
    map_update.entry(String::from("Blue")).or_insert(100);  // Won't change
    println!("  After entry().or_insert(): {:?}", map_update);

    // -------------------------------------------------------------------------
    // 6. UPDATE BASED ON OLD VALUE (Word Count Example)
    // -------------------------------------------------------------------------
    println!("\n📌 6. WORD COUNT EXAMPLE");
    
    let text = "I like programming in Rust because Rust is a great programming language!";
    let mut word_count = HashMap::new();
    
    for word in text.split_whitespace() {
        // entry() returns an Entry enum, or_insert() inserts 0 if missing
        // and returns a mutable reference to the value
        let count = word_count.entry(word).or_insert(0);
        *count += 1;  // Dereference to update the value
    }
    
    println!("  Word counts:");
    for (word, count) in &word_count {
        println!("    '{}': {}", word, count);
    }

    // -------------------------------------------------------------------------
    // 7. COMMON HASHMAP METHODS
    // -------------------------------------------------------------------------
    println!("\n📌 7. COMMON METHODS");
    
    let mut book_scores = HashMap::new();
    book_scores.insert(String::from("Book A"), 95);
    book_scores.insert(String::from("Book B"), 87);
    book_scores.insert(String::from("Book C"), 92);
    
    println!("  Original: {:?}", book_scores);
    println!("  Length: {}", book_scores.len());
    println!("  Contains 'Book A'? {}", book_scores.contains_key("Book A"));
    println!("  Contains 'Book D'? {}", book_scores.contains_key("Book D"));
    
    // Remove an entry
    let removed = book_scores.remove("Book B");
    println!("  Removed 'Book B': {:?}", removed);
    println!("  After remove: {:?}", book_scores);
    
    // Check if empty
    println!("  Is empty? {}", book_scores.is_empty());
    
    // Get keys and values separately
    let keys: Vec<&String> = book_scores.keys().collect();
    let values: Vec<&i32> = book_scores.values().collect();
    println!("  Keys: {:?}", keys);
    println!("  Values: {:?}", values);

    // -------------------------------------------------------------------------
    // 8. MERGING HASHMAPS
    // -------------------------------------------------------------------------
    println!("\n📌 8. MERGING HASHMAPS");
    
    let mut map1 = HashMap::new();
    map1.insert("a", 1);
    map1.insert("b", 2);
    
    let mut map2 = HashMap::new();
    map2.insert("b", 3);
    map2.insert("c", 4);
    
    // Extend map1 with map2 (overwrites duplicates)
    map1.extend(map2);
    println!("  After extend: {:?}", map1);

    // -------------------------------------------------------------------------
    // SUMMARY TABLE
    // -------------------------------------------------------------------------
    println!("\n=========================================================");
    println!("📌 HASHMAP OPERATIONS QUICK REFERENCE");
    println!("=========================================================");
    println!("| Operation              | Example                          |");
    println!("|------------------------|----------------------------------|");
    println!("| Create empty           | HashMap::new()                   |");
    println!("| Insert                 | map.insert(key, value)           |");
    println!("| Get value              | map.get(&key)                     |");
    println!("| Get with default       | map.get(&key).copied().unwrap_or(0)|");
    println!("| Check key exists       | map.contains_key(&key)           |");
    println!("| Insert if missing      | map.entry(key).or_insert(value)  |");
    println!("| Update based on old    | *map.entry(key).or_insert(0) += 1|");
    println!("| Remove                 | map.remove(&key)                  |");
    println!("| Number of entries      | map.len()                        |");
    println!("| Iterate                | for (k, v) in &map {}            |");
    println!("| Get all keys           | map.keys()                       |");
    println!("| Get all values         | map.values()                     |");
    println!("| Clear all              | map.clear()                      |");
    println!("=========================================================");
}

// =============================================================================
// BONUS: PERFORMANCE TIPS
// =============================================================================
/*
📌 HASHMAP PERFORMANCE TIPS:
--------------------------
1. Use `with_capacity(n)` if you know approximate size to avoid reallocation
2. Default hasher is secure but slow; for performance, consider FNV or fxhash
3. Access is O(1) on average, but can degrade with poor hash function
4. Use `entry()` API to avoid double lookup
5. For small key sets, consider BTreeMap (sorted, but slower access)

📌 COMMON USE CASES:
-------------------
• Counting occurrences (word count)
• Caching/lookup tables
• Grouping data by key
• Implementing dictionaries/maps
*/