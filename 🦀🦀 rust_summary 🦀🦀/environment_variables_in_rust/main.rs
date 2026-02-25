// =============================================================================
// ENVIRONMENT VARIABLES IN RUST - COMPLETE GUIDE
// =============================================================================
// Environment variables are key-value pairs stored in the OS
// Used for configuration, secrets, and system information
// =============================================================================

use std::env;
use dotenv::dotenv;

// -----------------------------------------------------------------------------
// EXAMPLE 1: BASIC ENVIRONMENT VARIABLE OPERATIONS
// -----------------------------------------------------------------------------
fn basic_env_example() {
    println!("\n📌 BASIC ENVIRONMENT VARIABLES");
    
    let key = "MY_TEST_VAR";
    
    // 1. SET an environment variable (only affects current process)
    env::set_var(key, "12345");
    println!("  ✅ Set {} = 12345", key);
    
    // 2. GET an environment variable (returns Result)
    match env::var(key) {
        Ok(value) => println!("  ✅ Got {} = {}", key, value),
        Err(e) => println!("  ❌ Error reading {}: {}", key, e),
    }
    
    // 3. CHECK if variable exists
    if env::var(key).is_ok() {
        println!("  ✅ {} exists", key);
    }
    
    // 4. REMOVE an environment variable
    env::remove_var(key);
    println!("  🗑️ Removed {}", key);
    
    // Verify it's gone
    match env::var(key) {
        Ok(value) => println!("  Got {} = {}", key, value),
        Err(_) => println!("  ✅ {} successfully removed", key),
    }
}

// -----------------------------------------------------------------------------
// EXAMPLE 2: READING SYSTEM ENVIRONMENT VARIABLES
// -----------------------------------------------------------------------------
fn system_env_example() {
    println!("\n📌 SYSTEM ENVIRONMENT VARIABLES");
    
    // Common system environment variables
    let common_vars = ["PATH", "HOME", "USER", "TEMP", "OS"];
    
    for var_name in common_vars.iter() {
        match env::var(var_name) {
            Ok(value) => println!("  {} = {}", var_name, value),
            Err(_) => println!("  {} not set", var_name),
        }
    }
}

// -----------------------------------------------------------------------------
// EXAMPLE 3: COMMAND LINE ARGUMENTS
// -----------------------------------------------------------------------------
fn command_line_args_example() {
    println!("\n📌 COMMAND LINE ARGUMENTS");
    
    // Collect all command line arguments
    let args: Vec<String> = env::args().collect();
    
    println!("  Program name: {}", args[0]);
    println!("  Number of arguments: {}", args.len() - 1);
    
    if args.len() > 1 {
        println!("  Arguments:");
        for (i, arg) in args.iter().enumerate().skip(1) {
            println!("    [{}]: {}", i, arg);
        }
    }
}

// -----------------------------------------------------------------------------
// EXAMPLE 4: USING DOTENV FOR CONFIGURATION FILES
// -----------------------------------------------------------------------------
fn dotenv_example() {
    println!("\n📌 DOTENV EXAMPLE");
    
    // Load environment variables from .env file
    // This looks for a .env file in the current directory
    dotenv().ok();  // .ok() ignores errors if file doesn't exist
    
    // Try to read API_KEY from .env file
    match env::var("API_KEY") {
        Ok(value) => println!("  ✅ API_KEY = {}", value),
        Err(_) => println!("  ⚠️  API_KEY not set in .env file"),
    }
    
    // Try to read DATABASE_URL from .env file
    match env::var("DATABASE_URL") {
        Ok(value) => println!("  ✅ DATABASE_URL = {}", value),
        Err(_) => println!("  ⚠️  DATABASE_URL not set in .env file"),
    }
}

// -----------------------------------------------------------------------------
// EXAMPLE 5: PRACTICAL CONFIGURATION PATTERN
// -----------------------------------------------------------------------------
struct Config {
    database_url: String,
    api_key: String,
    port: u16,
    debug_mode: bool,
}

impl Config {
    fn from_env() -> Result<Self, String> {
        println!("\n📌 LOADING CONFIG FROM ENVIRONMENT");
        
        // Load .env file if it exists (for development)
        dotenv().ok();
        
        // Get required variables - return error if missing
        let database_url = env::var("DATABASE_URL")
            .map_err(|_| "DATABASE_URL must be set".to_string())?;
        
        let api_key = env::var("API_KEY")
            .map_err(|_| "API_KEY must be set".to_string())?;
        
        // Get optional variables with defaults
        let port = env::var("PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse()
            .unwrap_or(8080);
        
        let debug_mode = env::var("DEBUG")
            .unwrap_or_else(|_| "false".to_string())
            .parse()
            .unwrap_or(false);
        
        Ok(Config {
            database_url,
            api_key,
            port,
            debug_mode,
        })
    }
    
    fn print(&self) {
        println!("  Database URL: {}", self.database_url);
        println!("  Port: {}", self.port);
        println!("  Debug mode: {}", self.debug_mode);
        // Don't print API key in production!
        if self.debug_mode {
            println!("  API Key: {}", self.api_key);
        }
    }
}

// -----------------------------------------------------------------------------
// EXAMPLE 6: ITERATING OVER ALL ENVIRONMENT VARIABLES
// -----------------------------------------------------------------------------
fn list_all_env_vars() {
    println!("\n📌 ALL ENVIRONMENT VARIABLES");
    
    // env::vars() returns an iterator over all environment variables
    for (key, value) in env::vars() {
        println!("  {} = {}", key, value);
    }
}

// -----------------------------------------------------------------------------
// EXAMPLE 7: ENVIRONMENT VARIABLES IN TESTS
// -----------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_with_env_var() {
        // Set environment variable for this test only
        env::set_var("TEST_MODE", "true");
        
        assert_eq!(env::var("TEST_MODE").unwrap(), "true");
        
        // Clean up
        env::remove_var("TEST_MODE");
    }
    
    #[test]
    fn test_with_dotenv() {
        dotenv().ok();
        // Test depends on .env file
    }
}

// -----------------------------------------------------------------------------
// CREATE A .ENV FILE EXAMPLE
// -----------------------------------------------------------------------------
/*
Create a file named `.env` in your project root:

# .env file example
DATABASE_URL=postgres://user:pass@localhost/mydb
API_KEY=abc123def456
PORT=3000
DEBUG=true
*/

// -----------------------------------------------------------------------------
// CARGO.TOML DEPENDENCY
// -----------------------------------------------------------------------------
/*
[dependencies]
dotenv = "0.15.0"
*/

// -----------------------------------------------------------------------------
// MAIN FUNCTION
// -----------------------------------------------------------------------------
fn main() {
    println!("=========================================================");
    println!("ENVIRONMENT VARIABLES IN RUST");
    println!("=========================================================");
    
    // Run examples
    basic_env_example();
    system_env_example();
    command_line_args_example();
    dotenv_example();
    
    // Load config from environment
    match Config::from_env() {
        Ok(config) => {
            config.print();
        }
        Err(e) => {
            println!("\n❌ Configuration error: {}", e);
            println!("   Create a .env file or set environment variables:");
            println!("   DATABASE_URL=postgres://localhost/mydb");
            println!("   API_KEY=your_api_key_here");
        }
    }
    
    println!("\n=========================================================");
    println!("📌 ENVIRONMENT VARIABLES CHEAT SHEET");
    println!("=========================================================");
    println!("| Operation                    | Code                               |");
    println!("|------------------------------|------------------------------------|");
    println!("| Set variable                  | env::set_var(\"KEY\", \"value\");     |");
    println!("| Get variable                  | env::var(\"KEY\") -> Result<String> |");
    println!("| Get with default              | env::var(\"KEY\").unwrap_or(\"default\")|");
    println!("| Check if exists               | env::var(\"KEY\").is_ok()           |");
    println!("| Remove variable               | env::remove_var(\"KEY\");            |");
    println!("| All variables                 | env::vars()                       |");
    println!("| Command line args             | env::args()                       |");
    println!("| Load .env file                | dotenv().ok();                    |");
    println!("=========================================================");
    
    println!("\n📌 COMMAND LINE USAGE:");
    println!("  $ export MY_VAR=123                    # Linux/Mac");
    println!("  $ set MY_VAR=123                        # Windows");
    println!("  $ cargo run -- --my-arg value           # Pass arguments");
    println!("  $ MY_VAR=123 cargo run                  # Set for one command");
}

// =============================================================================
// COMMON ERRORS AND SOLUTIONS
// =============================================================================
/*
❌ ERROR: env::var("KEY").unwrap()
   → Panics if KEY doesn't exist
   ✅ Fix: Use match or unwrap_or()

❌ ERROR: Can't find .env file
   → Make sure .env is in the same directory you run from
   ✅ Fix: Use dotenv().ok() which ignores missing files

❌ ERROR: set_var only affects current process
   → This is normal! Child processes don't inherit changes
   ✅ Fix: Set variables before spawning child processes
*/