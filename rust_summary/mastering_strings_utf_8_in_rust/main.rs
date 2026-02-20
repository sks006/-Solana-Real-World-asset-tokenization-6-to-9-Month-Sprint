// =============================================================================
// RUST STRINGS - COMPLETE GUIDE
// =============================================================================
// Strings in Rust are UTF-8 encoded collections of bytes.
// Two main types: String (owned) and &str (borrowed)
// =============================================================================

fn main() {
    println!("=========================================================");
    println!("RUST STRINGS - ALL OPERATIONS");
    println!("=========================================================");

    // -------------------------------------------------------------------------
    // 1. CREATING STRINGS
    // -------------------------------------------------------------------------
    println!("\n📌 1. CREATING STRINGS");
    
    // Empty string (like Vec::new())
    let mut s1 = String::new();
    println!("  Empty string: '{}'", s1);
    
    // From string literal using to_string()
    let data = "initial contents";
    let s2 = data.to_string();
    println!("  to_string(): '{}'", s2);
    
    // Directly on literal
    let s3 = "initial contents".to_string();
    println!("  literal.to_string(): '{}'", s3);
    
    // Using String::from() (most common)
    let s4 = String::from("initial contents");
    println!("  String::from(): '{}'", s4);
    
    // UTF-8 encoded strings (any language!)
    let hello = String::from("你好");           // Chinese
    println!("  UTF-8 Chinese: '{}'", hello);
    let hello = String::from("안녕하세요");     // Korean
    println!("  UTF-8 Korean: '{}'", hello);
    let hello = String::from("Здравствуйте");  // Russian
    println!("  UTF-8 Russian: '{}'", hello);
    let hello = String::from("नमस्ते");        // Hindi
    println!("  UTF-8 Hindi: '{}'", hello);
    let hello = String::from("😊🚀🦀");          // Emojis!
    println!("  UTF-8 Emojis: '{}'", hello);

    // -------------------------------------------------------------------------
    // 2. UPDATING STRINGS (Adding to them)
    // -------------------------------------------------------------------------
    println!("\n📌 2. UPDATING STRINGS");
    
    let mut s = String::from("hello");
    println!("  Initial: '{}'", s);
    
    // push_str() - add a string slice
    s.push_str(", world");
    println!("  After push_str(): '{}'", s);
    
    // push() - add a single character
    s.push('!');
    println!("  After push('!'): '{}'", s);
    
    // insert() - insert at index (careful with UTF-8!)
    s.insert(5, ',');  // Insert ',' at index 5
    println!("  After insert(5, ','): '{}'", s);

    // -------------------------------------------------------------------------
    // 3. CONCATENATION
    // -------------------------------------------------------------------------
    println!("\n📌 3. CONCATENATION");
    
    // Method 1: + operator (takes ownership of first string)
    let s1 = String::from("Hello ");
    let s2 = String::from("World");
    let s3 = s1 + &s2;  // s1 is MOVED here, can't use after
    println!("  + operator: '{}'", s3);
    // println!("{}", s1); // ❌ s1 is moved!
    
    // Method 2: format! macro (doesn't take ownership)
    let t1 = String::from("one");
    let t2 = String::from("two");
    let t3 = String::from("three");
    
    let combined = format!("{}-{}-{}", t1, t2, t3);
    println!("  format! macro: '{}'", combined);
    // All t1, t2, t3 are still usable!
    println!("  Original strings still available: '{}', '{}', '{}'", t1, t2, t3);
    
    // Method 3: push_str chain
    let mut base = String::from("start");
    base.push_str(" + middle");
    base.push_str(" + end");
    println!("  push_str chain: '{}'", base);

    // -------------------------------------------------------------------------
    // 4. INDEXING INTO STRINGS (Why it's tricky)
    // -------------------------------------------------------------------------
    println!("\n📌 4. INDEXING INTO STRINGS");
    
    // ❌ This does NOT work:
    // let s = String::from("hello");
    // let h = s[0];  // Error: cannot index into String
    
    println!("  ❌ Rust strings don't support direct indexing!");
    println!("  Why? Because UTF-8 is variable-width:");
    println!("    'a' = 1 byte");
    println!("    'ß' = 2 bytes");
    println!("    '中' = 3 bytes");
    println!("    '😊' = 4 bytes");

    // -------------------------------------------------------------------------
    // 5. SLICING STRINGS (Use with caution!)
    // -------------------------------------------------------------------------
    println!("\n📌 5. SLICING STRINGS");
    
    let hello = "안녕하세요";  // Korean: "annyeonghaseyo"
    println!("  Korean: '{}'", hello);
    
    // Slicing works on byte boundaries (can break UTF-8!)
    let slice = &hello[0..3];  // First 3 bytes (first character '안')
    println!("  Safe slice [0..3]: '{}'", slice);
    
    // ❌ This would panic if uncommented (not on character boundary)
    // let bad_slice = &hello[0..2];  // Panics! Not a char boundary
    // println!("  Bad slice: '{}'", bad_slice);
    
    println!("  ⚠️  Slice only at byte boundaries that align with characters!");

    // -------------------------------------------------------------------------
    // 6. ITERATING OVER STRINGS (The Safe Way)
    // -------------------------------------------------------------------------
    println!("\n📌 6. ITERATING OVER STRINGS");
    
    let word = "नमस्ते";  // Hindi "namaste"
    println!("  Word: '{}'", word);
    
    // Method 1: Iterate over characters (Unicode scalar values)
    println!("  chars():");
    for (i, c) in word.chars().enumerate() {
        println!("    [{}] = '{}'", i, c);
    }
    
    // Method 2: Iterate over bytes (raw data)
    println!("  bytes():");
    for (i, b) in word.bytes().enumerate() {
        println!("    [{}] = {}", i, b);
    }
    
    // Count characters vs bytes
    println!("  Length in chars: {}", word.chars().count());
    println!("  Length in bytes: {}", word.len());

    // -------------------------------------------------------------------------
    // 7. COMMON STRING METHODS
    // -------------------------------------------------------------------------
    println!("\n📌 7. COMMON STRING METHODS");
    
    let text = String::from("  Hello, World!  ");
    
    println!("  Original: '{}'", text);
    println!("  Length: {}", text.len());
    println!("  Is empty? {}", text.is_empty());
    println!("  Contains 'World'? {}", text.contains("World"));
    println!("  Starts with '  '? {}", text.starts_with("  "));
    println!("  Ends with '  '? {}", text.ends_with("  "));
    println!("  Trimmed: '{}'", text.trim());
    println!("  To uppercase: '{}'", text.to_uppercase());
    println!("  To lowercase: '{}'", text.to_lowercase());
    println!("  Replace 'World' with 'Rust': '{}'", text.replace("World", "Rust"));

    // -------------------------------------------------------------------------
    // 8. STRING vs &str CONVERSIONS
    // -------------------------------------------------------------------------
    println!("\n📌 8. STRING vs &str");
    
    // &str to String
    let str_slice: &str = "hello";
    let string1: String = str_slice.to_string();
    let string2: String = String::from(str_slice);
    let string3: String = str_slice.to_owned();
    println!("  &str to String: '{}', '{}', '{}'", string1, string2, string3);
    
    // String to &str
    let string = String::from("hello");
    let slice: &str = &string;  // &String coerces to &str
    let slice2: &str = &string[..];  // Full slice
    println!("  String to &str: '{}', '{}'", slice, slice2);

    // -------------------------------------------------------------------------
    // 9. COLLECTING FROM ITERATORS
    // -------------------------------------------------------------------------
    println!("\n📌 9. COLLECTING FROM ITERATORS");
    
    let chars = ['h', 'e', 'l', 'l', 'o'];
    let collected: String = chars.iter().collect();
    println!("  From chars array: '{}'", collected);
    
    let words = vec!["hello", "world"];
    let joined = words.join(" ");
    println!("  Join with space: '{}'", joined);
    
    let numbers = (1..=5).map(|x| x.to_string()).collect::<Vec<String>>();
    println!("  Numbers as strings: {:?}", numbers);

    // -------------------------------------------------------------------------
    // SUMMARY TABLE
    // -------------------------------------------------------------------------
    println!("\n=========================================================");
    println!("📌 STRING OPERATIONS QUICK REFERENCE");
    println!("=========================================================");
    println!("| Operation              | Example                          |");
    println!("|------------------------|----------------------------------|");
    println!("| Create empty           | String::new()                    |");
    println!("| From &str              | String::from(\"hello\")            |");
    println!("| From literal           | \"hello\".to_string()              |");
    println!("| Append string          | s.push_str(\"world\")              |");
    println!("| Append char            | s.push('!')                      |");
    println!("| Concatenate            | s1 + &s2                         |");
    println!("| Format                 | format!(\"{}-{}\", a, b)          |");
    println!("| Length in bytes        | s.len()                          |");
    println!("| Length in chars        | s.chars().count()                |");
    println!("| Check empty            | s.is_empty()                     |");
    println!("| Contains                | s.contains(\"pat\")                |");
    println!("| Replace                | s.replace(\"a\", \"b\")            |");
    println!("| Trim                   | s.trim()                         |");
    println!("| Uppercase              | s.to_uppercase()                 |");
    println!("| Lowercase              | s.to_lowercase()                 |");
    println!("| Iterate chars          | for c in s.chars() {}            |");
    println!("| Iterate bytes          | for b in s.bytes() {}            |");
    println!("=========================================================");
}

// =============================================================================
// BONUS: WHY STRING INDEXING DOESN'T WORK
// =============================================================================
/*
fn why_no_indexing() {
    let s = "नमस्ते";  // Hindi "namaste"
    
    // In memory, it's stored as bytes:
    // [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]
    //  ↑ 'न' 3 bytes   ↑ 'म' 3 bytes   ↑ 'स' 3 bytes   ↑ '्' 3 bytes   ↑ 'त' 3 bytes   ↑ 'े' 3 bytes
    
    // What should s[0] return?
    // - First byte? (224) - not meaningful
    // - First character? ('न') - but that's 3 bytes!
    
    // Rust prevents ambiguity by not allowing indexing at all.
}
*/

// =============================================================================
// BONUS: PERFORMANCE TIPS
// =============================================================================
/*
📌 STRING PERFORMANCE TIPS:
--------------------------
1. Use `String::with_capacity(n)` if you know approximate size
2. Reuse strings with `clear()` instead of creating new ones
3. `push_str` is more efficient than `+` for multiple appends
4. `format!` is convenient but allocates new string each time
5. For heavy text processing, consider `String` vs `&str` carefully
*/