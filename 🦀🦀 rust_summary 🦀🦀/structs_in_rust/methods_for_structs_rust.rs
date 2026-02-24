// =====================================================
// RECTANGLE STRUCT WITH MULTIPLE IMPLEMENTATIONS
// =====================================================

// Define a simple Rectangle struct
struct Rectangle {
    width: u32,
    height: u32,
}

// =====================================================
// FIRST IMPLEMENTATION BLOCK - Basic methods
// =====================================================
impl Rectangle {
    // ASSOCIATED FUNCTION (constructor)
    // Creates a new rectangle with given width and height
    // Called as: Rectangle::new(10, 20)
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }  // field init shorthand
    }
    
    // ASSOCIATED FUNCTION (constructor for square)
    // Creates a square (equal width and height)
    // Called as: Rectangle::square(10)
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
    
    // METHOD - calculates area
    // Uses &self to borrow the instance
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    // METHOD - checks if rectangle is square
    fn is_square(&self) -> bool {
        self.width == self.height
    }
}

// =====================================================
// SECOND IMPLEMENTATION BLOCK - More methods
// You can have multiple impl blocks for the same struct!
// =====================================================
impl Rectangle {
    // METHOD - calculates perimeter
    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }
    
    // METHOD - checks if this rectangle can hold another
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    
    // METHOD - prints rectangle details
    fn print_info(&self) {
        println!("Rectangle: {} x {}", self.width, self.height);
        println!("Area: {}, Perimeter: {}", self.area(), self.perimeter());
        if self.is_square() {
            println!("This is a square!");
        }
    }
}

// =====================================================
// THIRD IMPLEMENTATION BLOCK - Getters and setters
// =====================================================
impl Rectangle {
    // GETTER methods - return field values
    fn get_width(&self) -> u32 {
        self.width
    }
    
    fn get_height(&self) -> u32 {
        self.height
    }
    
    // SETTER methods - modify field values
    // Note: needs &mut self to modify
    fn set_width(&mut self, width: u32) {
        self.width = width;
    }
    
    fn set_height(&mut self, height: u32) {
        self.height = height;
    }
}

// =====================================================
// MAIN FUNCTION - Testing all implementations
// =====================================================
fn main() {
    // =================================================
    // Using the CONSTRUCTORS (associated functions)
    // =================================================
    println!("=== CREATING RECTANGLES ===");
    
    // Create using new() constructor
    let rect1 = Rectangle::new(30, 50);
    println!("Created rect1 with new(): {}x{}", rect1.get_width(), rect1.get_height());
    
    // Create using square() constructor
    let square1 = Rectangle::square(25);
    println!("Created square1 with square(): {}x{}", square1.get_width(), square1.get_height());
    
    // =================================================
    // Using the METHODS
    // =================================================
    println!("\n=== USING METHODS ===");
    
    // Area method
    println!("rect1 area: {}", rect1.area());           // 30 * 50 = 1500
    println!("square1 area: {}", square1.area());       // 25 * 25 = 625
    
    // is_square method
    println!("rect1 is square? {}", rect1.is_square());   // false
    println!("square1 is square? {}", square1.is_square()); // true
    
    // Perimeter method (from second impl block)
    println!("rect1 perimeter: {}", rect1.perimeter());   // 2*(30+50) = 160
    
    // Print info method (from second impl block)
    println!("\n=== PRINTING INFO ===");
    rect1.print_info();
    println!();
    square1.print_info();
    
    // =================================================
    // Using GETTERS and SETTERS
    // =================================================
    println!("\n=== USING GETTERS & SETTERS ===");
    
    // Create a mutable rectangle
    let mut rect2 = Rectangle::new(10, 20);
    println!("Initial rect2: {}x{}", rect2.get_width(), rect2.get_height());
    
    // Modify using setters
    rect2.set_width(15);
    rect2.set_height(25);
    println!("After setters: {}x{}", rect2.get_width(), rect2.get_height());
    println!("New area: {}", rect2.area());  // 15 * 25 = 375
    
    // =================================================
    // Using can_hold method
    // =================================================
    println!("\n=== CAN HOLD TEST ===");
    
    let small = Rectangle::new(10, 15);
    let medium = Rectangle::new(20, 25);
    let large = Rectangle::new(30, 35);
    
    println!("Can large hold medium? {}", large.can_hold(&medium));  // true
    println!("Can medium hold large? {}", medium.can_hold(&large));  // false
    println!("Can medium hold small? {}", medium.can_hold(&small));  // true
}

// =====================================================
// SUMMARY - Key Concepts Demonstrated
// =====================================================
/*
1. ASSOCIATED FUNCTIONS (no self):
   - Called with :: syntax (Rectangle::new)
   - Used as constructors
   - Don't need an instance to call

2. METHODS (with self):
   - Called with . syntax (rect1.area)
   - Operate on an instance
   - Can be &self (read-only), &mut self (modify), or self (consume)

3. MULTIPLE IMPL BLOCKS:
   - You can have many impl blocks for the same struct
   - Helps organize related methods together
   - All methods are available regardless of which block they're in

4. GETTERS & SETTERS:
   - Control access to fields
   - Can add validation logic
   - Maintain API stability if internal structure changes
*/