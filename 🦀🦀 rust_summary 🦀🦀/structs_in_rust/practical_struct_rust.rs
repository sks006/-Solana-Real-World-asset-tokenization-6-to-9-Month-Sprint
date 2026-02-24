

// The main function - entry point of every Rust program
fn main(){
    // Create a tuple with two elements: width (30) and height (50)
    // In Rust, tuples are written as (value1, value2, ...)
    // Here, both values are of type u32 (unsigned 32-bit integer)
    let rect1 = (30, 50);  // rect1 is a tuple: (u32, u32)

    // println! is a macro that prints text to the console
    // The { } are placeholders that get filled with values
    // The first { } will be replaced by the result of area(rect1)
    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect1)  // This function call provides the value for { }
    );
    // When this runs, it becomes: "The area... is 1500 square pixels."
}

// Function that calculates the area of a rectangle from a tuple
// Parameters:
//   - dimensions: a tuple containing (width, height) both u32
// Returns:
//   - u32: the calculated area (width × height)
fn area(dimensions: (u32, u32)) -> u32 {
    // Access tuple elements with .0, .1, .2, etc.
    // .0 accesses the first element (width)
    // .1 accesses the second element (height)
    // Multiply them together to get the area
    
    dimensions.0 * dimensions.1  // width × height = area
    
    // Note: No semicolon means this expression is the return value
    // This is equivalent to: return dimensions.0 * dimensions.1;
}

//----------------------------------------------------------


// =====================================================
// RUST TUTORIAL: CALCULATING RECTANGLE AREA WITH STRUCTS
// =====================================================
// This example shows how to use structs to group related data
// Structs give names to fields, making code more readable and maintainable
// =====================================================

// Define a custom struct type named Rectangle
// Structs are like blueprints for creating values with named fields
struct Rectangle {
    width: u32,   // field name: width, type: unsigned 32-bit integer
    height: u32,  // field name: height, type: unsigned 32-bit integer
}

fn main() {
    // Create an instance of the Rectangle struct
    // We specify values for each field using the field names
    let rect1 = Rectangle {
        width: 30,   // assign 30 to the width field
        height: 50,  // assign 50 to the height field
    };
    // rect1 is now a Rectangle with width = 30, height = 50

    // Print the calculated area
    // Note: The original code had a typo: `are("rect1)` should be `area(&rect1)`
    // We fixed it below:
    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)  // Pass a reference to rect1 to avoid moving ownership
    );
    // Expected output: "The area of the rectangle is 1500 square pixels."
}

// Function that calculates the area of a Rectangle
// Parameters:
//   - rectangle: a reference (&) to a Rectangle
//     Using a reference allows us to borrow the struct without taking ownership
// Returns:
//   - u32: the product of width and height
fn area(rectangle: &Rectangle) -> u32 {
    // Access struct fields using dot notation: rectangle.field_name
    rectangle.width * rectangle.height  // Multiply width × height
    // No semicolon means this expression is the return value
}

// ----------------------------------------------------------

// =====================================================
// RUST TUTORIAL: CALCULATING RECTANGLE AREA WITH STRUCTS
// =====================================================
// This example shows how to use structs to group related data
// Structs give names to fields, making code more readable and maintainable
// =====================================================

// Derive Debug so we can print the struct with {:?} and {:#?}
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    // Create an instance of the Rectangle struct
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    // Print the calculated area
    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    // Print the entire struct using debug formatting
    println!("Compact debug: {:?}", rect1);
    println!("Pretty debug:\n{:#?}", rect1);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}