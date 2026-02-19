struct User {
    name: String,
    email: String,
    is_active: bool,
    age: u8
}

fn main() {
    let user1 = User {
        name: String::from("John Doe"),
        email: String::from("doe@mail.com"),
        is_active: true,
        age: 25
    };

    //print all the values using debug formatting
    println!("{} {} {}", user1.name,user1.email,user1.is_active);  // {:?} is for debug output
}


//---------------------------------------------------------------


#[derive(Debug)]  // This automatically implements the Debug trait
struct User {
    name: String,
    email: String,
    is_active: bool,
    age: u8
}

fn main() {
    let user1 = User {
        name: String::from("John Doe"),
        email: String::from("doe@mail.com"),
        is_active: true,
        age: 25
    };

    //print all the values using debug formatting
    println!("{:?}", user1);  // {:?} is for debug output
}

//---------------------------------------------------------------

// Define a struct named User with four fields
struct User {
    name: String,      // String type for the user's name
    email: String,     // String type for the user's email
    is_active: bool,   // boolean to track if user is active
    age: u8            // unsigned 8-bit integer for age (0-255)
}

fn main() {
    // Create a MUTABLE instance of the User struct
    // The 'mut' keyword allows us to modify its fields later
    let mut user1 = User {
        name: String::from("John Doe"),    // Initialize name
        email: String::from("doe@mail.com"), // Initialize email
        is_active: true,                    // Initialize active status
        age: 25                              // Initialize age
    };

    // MODIFY the name field of the user1 instance
    // This is possible because user1 was declared as 'mut'
    user1.name = String::from("Francesco");
    
    // Print only the name field using dot notation
    // This accesses and displays the modified name
    println!("Name: {}", user1.name);
}


//---------------------------------------------------------------

struct User {
    name: String,
    email: String,
    is_active: bool,
    age: u8
}

fn main() {
    // Create a user by calling the build_user function
    // The function takes two String parameters and returns a User struct
    let user1 = build_user(
        String::from("John Doe"),    // First argument: name
        String::from("doe@mail.com") // Second argument: email
    );
    
    // Print all the values of the user struct
    // Note: This line was incomplete in the original code
    println!("Name: {}, Email: {}, Active: {}, Age: {}", 
             user1.name, user1.email, user1.is_active, user1.age);
}

// Function that builds and returns a User struct
// Takes name and email as parameters (both String type)
// Returns a User instance with the provided values and default values for other fields
fn build_user(name: String, email: String) -> User {
    User {
        name,           // Field init shorthand: same as "name: name"
        email,          // Field init shorthand: same as "email: email"
        is_active: true, // Default value: all users start as active
        age: 25          // Default value: all users start at age 25
    }
}

//----------------------------------------------------------------------------------------

struct User {
    name: String,
    email: String,
    is_active: bool,
    age: u8
}

fn main() {
    // First, create the original user (user1)
    let user1 = User {
        name: String::from("John Doe"),
        email: String::from("doe@mail.com"),
        is_active: true,
        age: 25
    };
    
    // Creating a new instance (user2) by copying some fields from user1
    let user2 = User {
        // New name field - completely new value, not from user1
        name: String::from("Francesco"),
        
        // email field is MOVED from user1 (not copied!)
        // After this, user1.email is no longer valid!
        //String does NOT implement the Copy trait
        //Ownership of the String data is transferred from user1 to user2
        //After this line, user1.email is no longer valid - you cannot use it
        
        email: user1.email,
        
        // is_active is copied from user1 (bool implements Copy trait)
        is_active: user1.is_active,
        
        // age is copied from user1 (u8 implements Copy trait)
        // u8 (and other simple types like i32, bool, f64) implements the Copy trait
        // The value is duplicated - a complete copy is made
        // Both user1.age and user2.age are valid and independent
        age: user1.age,
    };

    // Print all the values of user2
    println!(
        "Name: {} Email: {} Active: {} Age: {}",
        user2.name,      // "Francesco" - the new name we set
        user2.email,     // "doe@mail.com" - moved from user1
        user2.is_active, // true - copied from user1
        user2.age        // 25 - copied from user1
    );
    
    // WARNING: user1.email is now INVALID because ownership was moved!
    // The following line would cause a compile error if uncommented:
    // println!("{}", user1.email); // ERROR! value borrowed here after move
    
    // But user1.name, user1.is_active, and user1.age are still valid!
    println!("User1's name is still accessible: {}", user1.name); // Works!
    println!("User1's age is still accessible: {}", user1.age);   // Works!
}

//----------------------------------------------------------------------------------------
// TUPLE STRUCTS
// These are structs that look like tuples but have their own type names
// They're useful for creating simple wrappers around values

// Define a tuple struct named Color with three i32 fields
// Even though it has the same structure as Point, Color and Point are DIFFERENT types!
struct Color(i32, i32, i32);

// Define a tuple struct named Point with three i32 fields
// This creates a completely separate type from Color
struct Point(i32, i32, i32);

fn main() {
    // Create an instance of Color tuple struct
    // Fields are accessed by index, just like tuples
    let black = Color(0, 0, 0);
    
    // Create an instance of Point tuple struct
    // Even though the values are the same (0,0,0), this is a different type!
    let origin = Point(0, 0, 0);
    
    // Access tuple struct fields using dot notation with indices
    // black.0 accesses the first field, black.1 the second, etc.
    println!("black: {}, {}, {}", black.0, black.1, black.2);
    
    // origin.0, origin.1, origin.2 access the Point's fields
    println!("origin: {}, {}, {}", origin.0, origin.1, origin.2);
    
    // TYPE SAFETY EXAMPLE - This would NOT compile:
    // let error: Color = origin; // ERROR! mismatched types: expected Color, found Point
}

//-----------------------------------------------------------------------------
// UNIT-LIKE STRUCT (also called unit struct)
// This is a struct with no fields!
// The name comes from similarity to the unit type () which also has no data
#[derive(Debug)]  // Automatically implements the Debug trait so we can print it
struct User;      // Note: no curly braces or parentheses - just the name and semicolon

fn main() {
    // Create an instance of the unit-like struct
    // No need for parentheses or curly braces - just use the name
    let user = User;
    
    // Print the unit-like struct using debug formatting
    // This will output just "User" since there's no data to display
    println!("{:?}", user);  // Output: User
    
    // Unit-like structs occupy no memory - they're zero-sized types
    println!("Size of User: {} bytes", std::mem::size_of::<User>()); // Output: 0 bytes
}

