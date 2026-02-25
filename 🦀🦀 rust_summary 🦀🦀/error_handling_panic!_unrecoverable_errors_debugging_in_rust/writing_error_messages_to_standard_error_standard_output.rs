// =============================================================================
// STDOUT AND STDERR IN RUST - COMPLETE GUIDE
// =============================================================================
// STDOUT: Standard output - for normal program output
// STDERR: Standard error - for error messages and diagnostics
// =============================================================================

use std::fs::read_to_string;
use colored::*;  // For colored output (add to Cargo.toml)

// -----------------------------------------------------------------------------
// BASIC STDOUT AND STDERR
// -----------------------------------------------------------------------------
fn basic_stdout_stderr() {
    println!("\n📌 BASIC STDOUT AND STDERR");
    
    // STDOUT - normal program output
    println!("This goes to STDOUT");
    print!("This also goes to STDOUT (without newline)");
    
    // STDERR - error messages and diagnostics
    eprintln!("This goes to STDERR");
    eprint!("This also goes to STDERR (without newline)");
}

// -----------------------------------------------------------------------------
// EXAMPLE WITH FILE OPERATIONS
// -----------------------------------------------------------------------------
fn file_operation_example() {
    println!("\n📌 FILE OPERATION EXAMPLE");
    
    let filename = "non_existent_file.txt";
    
    // Try to read a file that doesn't exist
    match read_to_string(filename) {
        Ok(contents) => {
            // Success - print to STDOUT
            println!("✅ File contents: {}", contents);
        }
        Err(e) => {
            // Error - print to STDERR
            eprintln!("❌ Error reading file '{}': {}", filename, e);
        }
    }
}

// -----------------------------------------------------------------------------
// COLORIZED OUTPUT (requires colored crate)
// -----------------------------------------------------------------------------
fn colorized_output() {
    println!("\n📌 COLORIZED OUTPUT");
    
    // Normal output
    println!("Normal message");
    
    // Colored output to STDERR
    eprintln!("{}", "This is a RED error message".red());
    eprintln!("{}", "This is a BOLD RED error".red().bold());
    eprintln!("{}", "⚠️  Warning: This is YELLOW".yellow());
    eprintln!("{}", "✅ Success: This is GREEN".green());
    eprintln!("{}", "ℹ️  Info: This is BLUE".blue());
}

// -----------------------------------------------------------------------------
// REDIRECTION DEMONSTRATION
// -----------------------------------------------------------------------------
fn redirection_demo() {
    println!("\n📌 REDIRECTION DEMONSTRATION");
    println!("This line will go to STDOUT");
    eprintln!("This line will go to STDERR");
    println!("This is another STDOUT message");
    eprintln!("This is another STDERR message");
}

/*
TERMINAL COMMANDS FOR REDIRECTION:
---------------------------------
$ cargo run                    # Both STDOUT and STDERR show in terminal
$ cargo run > output.txt       # STDOUT goes to file, STDERR shows in terminal
$ cargo run 2> error.txt       # STDERR goes to file, STDOUT shows in terminal
$ cargo run > all.txt 2>&1     # Both STDOUT and STDERR go to same file
$ cargo run &> all.txt         # Both STDOUT and STDERR go to same file (bash)
*/

// -----------------------------------------------------------------------------
// PRACTICAL LOGGING WITH SEPARATE STREAMS
// -----------------------------------------------------------------------------
fn logging_example() {
    println!("\n📌 LOGGING EXAMPLE");
    
    // Application status - always show
    println!("Application started");
    
    // Detailed logs - could be redirected to a file
    eprintln!("[DEBUG] Loading configuration...");
    eprintln!("[DEBUG] Connecting to database...");
    eprintln!("[DEBUG] Processing request...");
    
    // Results - show to user
    println!("Operation completed successfully");
}

// -----------------------------------------------------------------------------
// PROGRESS INDICATOR (STDERR for status, STDOUT for results)
// -----------------------------------------------------------------------------
fn progress_example() {
    println!("\n📌 PROGRESS INDICATOR");
    
    eprint!("Processing: [");
    for i in 1..=10 {
        std::thread::sleep(std::time::Duration::from_millis(100));
        eprint!("=");
    }
    eprintln!("] Done!");
    
    // Final result goes to STDOUT
    println!("Result: 42");
}

// -----------------------------------------------------------------------------
// ERROR HANDLING WITH CONTEXT
// -----------------------------------------------------------------------------
fn error_handling_example() {
    println!("\n📌 ERROR HANDLING WITH CONTEXT");
    
    let filename = "config.txt";
    
    match read_to_string(filename) {
        Ok(content) => {
            println!("Config loaded: {}", content);
        }
        Err(e) => {
            // Use STDERR for detailed error information
            eprintln!("{}", "╔══════════════════════════════════════╗".red());
            eprintln!("{}", "║          ERROR DETECTED              ║".red().bold());
            eprintln!("{}", "╠══════════════════════════════════════╣".red());
            eprintln!("{}", format!("║ File: {}", filename).red());
            eprintln!("{}", format!("║ Error: {}", e).red());
            eprintln!("{}", "╚══════════════════════════════════════╝".red());
            
            // Normal output continues to STDOUT
            println!("Application continuing with defaults...");
        }
    }
}

// -----------------------------------------------------------------------------
// CARGO.TOML DEPENDENCIES
// -----------------------------------------------------------------------------
/*
[dependencies]
colored = "2.0"  # For colored output
*/

// -----------------------------------------------------------------------------
// MAIN FUNCTION
// -----------------------------------------------------------------------------
fn main() {
    println!("=========================================================");
    println!("STDOUT AND STDERR IN RUST");
    println!("=========================================================");
    
    // Run examples
    basic_stdout_stderr();
    file_operation_example();
    colorized_output();
    redirection_demo();
    logging_example();
    progress_example();
    error_handling_example();
    
    println!("\n=========================================================");
    println!("📌 REDIRECTION COMMANDS");
    println!("=========================================================");
    println!("| Command                          | Effect                           |");
    println!("|----------------------------------|----------------------------------|");
    println!("| cargo run                        | Both to terminal                 |");
    println!("| cargo run > output.txt           | STDOUT to file, STDERR to term   |");
    println!("| cargo run 2> error.txt           | STDERR to file, STDOUT to term   |");
    println!("| cargo run > all.txt 2>&1         | Both to same file                |");
    println!("| cargo run &> all.txt              | Both to same file (bash)         |");
    println!("| ./program | grep \"pattern\"       | Pipe only STDOUT                  |");
    println!("=========================================================");
    
    println!("\n📌 RUST MACROS:");
    println!("  • println!  → STDOUT (with newline)");
    println!("  • print!    → STDOUT (no newline)");
    println!("  • eprintln! → STDERR (with newline)");
    println!("  • eprint!   → STDERR (no newline)");
}

// =============================================================================
// COMPLETE EXAMPLE WITH ALL FEATURES
// =============================================================================
/*
use std::fs::read_to_string;
use colored::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filename = std::env::args().nth(1).unwrap_or("config.txt".to_string());
    
    // Status message to STDERR
    eprintln!("{}", format!("📂 Reading file: {}", filename).cyan());
    
    match read_to_string(&filename) {
        Ok(content) => {
            // Success - output to STDOUT
            println!("{}", content);
            eprintln!("{}", "✅ File read successfully".green());
            Ok(())
        }
        Err(e) => {
            // Error - output to STDERR with formatting
            eprintln!("{}", "╔════════════════════════════════════╗".red());
            eprintln!("{}", "║         ERROR: FILE ERROR          ║".red().bold());
            eprintln!("{}", "╠════════════════════════════════════╣".red());
            eprintln!("{}", format!("║ File: {}", filename).red());
            eprintln!("{}", format!("║ Error: {}", e).red());
            eprintln!("{}", "╚════════════════════════════════════╝".red());
            Err(Box::new(e))
        }
    }
}
*/

// =============================================================================
// QUICK REFERENCE
// =============================================================================
/*
🔑 KEY CONCEPTS:
---------------
• STDOUT: Normal program output (println!)
• STDERR: Error messages and diagnostics (eprintln!)
• They can be redirected independently
• STDERR is buffered, STDOUT is line-buffered

💡 BEST PRACTICES:
----------------
• Use STDOUT for actual program output/results
• Use STDERR for status, progress, errors, and debug info
• Colorize STDERR to make errors stand out
• Let users redirect streams as needed
• In scripts, use STDERR for messages that shouldn't be piped

📦 CRATES FOR ENHANCED OUTPUT:
----------------------------
• colored - Add colors to terminal output
• termion - Terminal control
• console - Better terminal interactions
• log - Logging facade
• env_logger - Environment-based logging
*/