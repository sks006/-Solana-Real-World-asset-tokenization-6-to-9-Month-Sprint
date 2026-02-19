// =====================================================
// RUST TUTORIAL: METHODS VS FUNCTIONS
// =====================================================
// Methods are functions defined inside an 'impl' block
// They are called on instances of a struct using dot notation
// =====================================================

// Define a struct to represent a rectangle
struct Rectangle {
    width: u32,
    height: u32,
}

// =====================================================
// IMPLEMENTATION BLOCK - Where methods are defined
// =====================================================
impl Rectangle {
    // METHOD: area
    // - First parameter is always 'self' (or &self, &mut self)
    // - &self is shorthand for self: &Self (where Self = Rectangle)
    // - &self means we BORROW the instance (don't take ownership)
    // - This allows us to read data without modifying it
    fn area(&self) -> u32 {
        // Access fields using self.field_name
        self.width * self.height
    }

    // Another method example: Check if rectangle is square
    fn is_square(&self) -> bool {
        self.width == self.height
    }

    // Method with parameters (other than self)
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// =====================================================
// MAIN FUNCTION - Using our methods
// =====================================================
fn main() {
    // Create an instance of Rectangle
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    // Call methods using DOT NOTATION: instance.method()
    // Rust automatically passes &rect1 as &self
    println!("The area of the rectangle is {} square pixels.", 
             rect1.area());

    // Check if it's a square
    println!("Is it a square? {}", rect1.is_square());

    // Create another rectangle to test can_hold
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
}

// =====================================================
// COMPARISON: STANDALONE FUNCTION (NOT A METHOD)
// =====================================================
// This is a FUNCTION, not a method
// - Not associated with any struct
// - Called as: area(&rect1)
// - More flexible but less organized

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
