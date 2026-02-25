// =============================================================================
// ITERATORS IN RUST - COMPLETE GUIDE
// =============================================================================
// Iterators are lazy, zero-cost abstractions for processing sequences.
// They allow you to transform, filter, and aggregate data efficiently.
// =============================================================================

// -----------------------------------------------------------------------------
// 1. THE ITERATOR TRAIT (Core concept)
// -----------------------------------------------------------------------------
pub trait Iterator {
    type Item;  // The type of elements being iterated over
    fn next(&mut self) -> Option<Self::Item>;  // Advance iterator and return next element
    // many other methods provided by default (map, filter, fold, etc.)
}

// -----------------------------------------------------------------------------
// 2. CREATING ITERATORS (Three ways)
// -----------------------------------------------------------------------------
fn creating_iterators() {
    let numbers = vec![1, 2, 3, 4, 5];
    
    // a) .iter() - Borrows elements immutably (yields &T)
    let iter = numbers.iter();
    for val in iter {
        println!("  iter: {}", val);  // val: &i32
    }
    println!("  numbers still usable: {:?}", numbers);  // ✅ still owned
    
    // b) .iter_mut() - Borrows elements mutably (yields &mut T)
    let mut numbers_mut = vec![1, 2, 3];
    for val in numbers_mut.iter_mut() {
        *val *= 2;  // modify in place
    }
    println!("  after iter_mut: {:?}", numbers_mut);
    
    // c) .into_iter() - Consumes the collection, takes ownership (yields T)
    let numbers_owned = vec![10, 20, 30];
    for val in numbers_owned.into_iter() {  // or just `for val in numbers_owned`
        println!("  into_iter: {}", val);   // val: i32 (owned)
    }
    // println!("{:?}", numbers_owned);  // ❌ numbers_owned moved
}

// -----------------------------------------------------------------------------
// 3. MANUAL ITERATION WITH .next()
// -----------------------------------------------------------------------------
fn manual_next() {
    let v = vec![10, 20, 30];
    let mut iter = v.iter();
    
    assert_eq!(iter.next(), Some(&10));
    assert_eq!(iter.next(), Some(&20));
    assert_eq!(iter.next(), Some(&30));
    assert_eq!(iter.next(), None);  // end of iteration
}

// -----------------------------------------------------------------------------
// 4. ADAPTERS: map, filter, fold (lazy transformations)
// -----------------------------------------------------------------------------
fn adapters() {
    let numbers = vec![1, 2, 3, 4, 5];
    
    // map: transform each element
    let squares: Vec<i32> = numbers.iter()
        .map(|&x| x * x)
        .collect();  // collect consumes the iterator
    println!("  squares: {:?}", squares);
    
    // filter: keep only elements satisfying a condition
    let evens: Vec<&i32> = numbers.iter()
        .filter(|&&x| x % 2 == 0)
        .collect();
    println!("  evens: {:?}", evens);
    
    // fold: accumulate (like reduce)
    let sum = numbers.iter()
        .fold(0, |acc, &x| acc + x);
    println!("  sum: {}", sum);
    
    // chain adapters (lazy, single pass)
    let result: Vec<i32> = numbers.iter()
        .map(|&x| x * 2)
        .filter(|&x| x > 5)
        .collect();
    println!("  chained: {:?}", result);
}

// -----------------------------------------------------------------------------
// 5. CONSUMING ADAPTERS (force iteration)
// -----------------------------------------------------------------------------
fn consuming() {
    let v = vec![1, 2, 3];
    
    // .collect() - gathers into a collection
    let doubled: Vec<i32> = v.iter().map(|&x| x * 2).collect();
    
    // .sum() - sums all elements
    let total: i32 = v.iter().sum();
    
    // .for_each() - apply closure to each element
    v.iter().for_each(|&x| println!("  {}", x));
    
    // .any(), .all() - check conditions
    let has_even = v.iter().any(|&x| x % 2 == 0);
    let all_positive = v.iter().all(|&x| x > 0);
}

// -----------------------------------------------------------------------------
// 6. OWNERSHIP AND ITERATORS (Key differences)
// -----------------------------------------------------------------------------
/*
| Method      | Returns          | Effect on original               |
|-------------|------------------|----------------------------------|
| .iter()     | &T               | Original remains usable          |
| .iter_mut() | &mut T           | Original remains usable, modified|
| .into_iter()| T                | Original consumed (moved)        |
*/

// -----------------------------------------------------------------------------
// 7. LAZY NATURE (No work until consumed)
// -----------------------------------------------------------------------------
fn lazy_demo() {
    let v = vec![1, 2, 3];
    
    let mapped = v.iter().map(|&x| {
        println!("  mapping {}", x);
        x * 2
    });
    // Nothing printed yet! map is lazy.
    
    let collected: Vec<_> = mapped.collect();  // NOW mapping happens
    println!("  collected: {:?}", collected);
}

// -----------------------------------------------------------------------------
// 8. COMMON ITERATOR METHODS QUICK REFERENCE
// -----------------------------------------------------------------------------
/*
| Method        | Purpose                                      |
|---------------|----------------------------------------------|
| .next()       | Advances iterator, returns Option<Item>      |
| .map()        | Transform each element                       |
| .filter()     | Keep elements satisfying predicate            |
| .fold()       | Accumulate over elements                     |
| .collect()    | Gather into collection (Vec, HashMap, etc.)  |
| .sum()        | Sum numeric elements                         |
| .any()        | Check if any element satisfies predicate     |
| .all()        | Check if all elements satisfy predicate      |
| .take(n)      | Limit to first n elements                    |
| .skip(n)      | Skip first n elements                        |
| .enumerate()  | Pair elements with their indices             |
| .zip()        | Combine two iterators pairwise               |
| .chain()      | Concatenate two iterators                    |
| .rev()        | Reverse iterator                             |
*/

// -----------------------------------------------------------------------------
// 9. PERFORMANCE: Iterators are zero-cost abstractions
// -----------------------------------------------------------------------------
// Rust's iterators compile down to hand-written loop code.
// No runtime overhead – often faster than manual loops due to optimizations.

// -----------------------------------------------------------------------------
// 10. MAIN FUNCTION TO DEMO
// -----------------------------------------------------------------------------
fn main() {
    println!("=========================================================");
    println!("ITERATORS IN RUST - COMPLETE GUIDE");
    println!("=========================================================");
    
    creating_iterators();
    manual_next();
    adapters();
    consuming();
    lazy_demo();
    
    println!("\n📌 QUICK REFERENCE");
    println!("  .iter()    → &T (borrow immutably)");
    println!("  .iter_mut()→ &mut T (borrow mutably)");
    println!("  .into_iter()→ T (take ownership)");
    println!("  Iterators are LAZY – no work until consumed");
    println!("  Adapters: .map(), .filter(), .fold(), ...");
    println!("  Consumers: .collect(), .sum(), .for_each(), ...");
}

// =============================================================================
// RECAP FROM IMAGES
// =============================================================================
/*
📌 KEY TAKEAWAYS:
----------------
• Iterators provide a functional, lazy way to process sequences.
• Three ways to get an iterator: .iter(), .iter_mut(), .into_iter().
• Adaptors (map, filter) are lazy; consumers (collect, sum) force execution.
• Iterators are zero-cost – they optimize as well as hand-written loops.
• Ownership is explicit: choose the right method based on whether you need to
  borrow (read), borrow mutably (modify), or own (consume) the data.
*/