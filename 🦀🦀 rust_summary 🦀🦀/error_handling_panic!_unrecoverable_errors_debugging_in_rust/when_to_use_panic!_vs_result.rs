// =============================================================================
// ADVANCED ERROR HANDLING PATTERNS IN RUST
// =============================================================================
// This combines rate limiting, custom types, and error handling patterns
// =============================================================================

use std::time::Duration;
use std::thread;
use std::fs;

// -----------------------------------------------------------------------------
// PART 1: RATE LIMITING IN HTTP REQUESTS
// -----------------------------------------------------------------------------

#[derive(Debug)]
enum HttpError {
    RateLimited,
    NotFound,
    ServerError,
    NetworkError(String),
    Other(u16),
}

impl std::fmt::Display for HttpError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            HttpError::RateLimited => write!(f, "Rate limited - too many requests"),
            HttpError::NotFound => write!(f, "Resource not found"),
            HttpError::ServerError => write!(f, "Server error"),
            HttpError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            HttpError::Other(code) => write!(f, "HTTP error: {}", code),
        }
    }
}

// Simulated HTTP request function
fn fetch_data(url: &str) -> Result<String, HttpError> {
    println!("  📡 Fetching from: {}", url);
    
    // Simulate different responses based on URL
    match url {
        "https://api.example.com/data" => Ok("Sample data".to_string()),
        "https://api.example.com/rate-limited" => Err(HttpError::RateLimited),
        "https://api.example.com/not-found" => Err(HttpError::NotFound),
        "https://api.example.com/server-error" => Err(HttpError::ServerError),
        _ => Err(HttpError::Other(418)), // I'm a teapot!
    }
}

// Retry mechanism with exponential backoff
fn fetch_data_with_retry(url: &str, max_retries: usize) -> Result<String, HttpError> {
    println!("\n📌 FETCH WITH RETRY (max: {} attempts)", max_retries);
    
    let mut delay = Duration::from_secs(1);
    
    for attempt in 1..=max_retries {
        println!("  Attempt {} of {}", attempt, max_retries);
        
        match fetch_data(url) {
            Ok(data) => return Ok(data),
            Err(HttpError::RateLimited) if attempt < max_retries => {
                println!("    ⏳ Rate limited! Retrying in {} secs...", delay.as_secs());
                thread::sleep(delay);
                delay *= 2; // Exponential backoff
            }
            Err(e) => return Err(e),
        }
    }
    
    Err(HttpError::RateLimited)
}

fn rate_limiting_example() {
    println!("\n=========================================================");
    println!("PART 1: RATE LIMITING EXAMPLE");
    println!("=========================================================");
    
    // Success case
    match fetch_data_with_retry("https://api.example.com/data", 3) {
        Ok(data) => println!("  ✅ Success: {}", data),
        Err(e) => println!("  ❌ Failed: {}", e),
    }
    
    // Rate limited case
    match fetch_data_with_retry("https://api.example.com/rate-limited", 3) {
        Ok(data) => println!("  ✅ Success: {}", data),
        Err(e) => println!("  ❌ Failed: {}", e),
    }
    
    // Not found case (no retry)
    match fetch_data_with_retry("https://api.example.com/not-found", 3) {
        Ok(data) => println!("  ✅ Success: {}", data),
        Err(e) => println!("  ❌ Failed: {}", e),
    }
}

// -----------------------------------------------------------------------------
// PART 2: CUSTOM TYPE WITH VALIDATION (Guess)
// -----------------------------------------------------------------------------

pub struct Guess {
    value: i32,
}

impl Guess {
    // Constructor with validation - returns Result
    pub fn new(value: i32) -> Result<Guess, String> {
        if value < 1 || value > 100 {
            Err(format!(
                "Guess value must be between 1 and 100, got {}",
                value
            ))
        } else {
            Ok(Guess { value })
        }
    }
    
    // Getter method
    pub fn value(&self) -> i32 {
        self.value
    }
    
    // Alternative: Panic version for prototypes
    pub fn new_panic(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}", value);
        }
        Guess { value }
    }
}

fn guess_example() {
    println!("\n=========================================================");
    println!("PART 2: CUSTOM TYPE WITH VALIDATION");
    println!("=========================================================");
    
    // Valid guess - using Result
    match Guess::new(50) {
        Ok(guess) => println!("  ✅ Valid guess: {}", guess.value()),
        Err(e) => println!("  ❌ Error: {}", e),
    }
    
    // Invalid guess - using Result
    match Guess::new(150) {
        Ok(guess) => println!("  ✅ Valid guess: {}", guess.value()),
        Err(e) => println!("  ❌ Error: {}", e),
    }
    
    // Prototype version (panics on invalid)
    // let valid = Guess::new_panic(75);  // Works
    // let invalid = Guess::new_panic(200);  // ❌ PANIC!
}

// -----------------------------------------------------------------------------
// PART 3: COMBINING PATTERNS - Config with Validation
// -----------------------------------------------------------------------------

#[derive(Debug)]
struct ApiConfig {
    url: String,
    max_retries: usize,
    timeout_seconds: u64,
    api_key: String,
}

impl ApiConfig {
    fn new(url: String, max_retries: usize, timeout_seconds: u64, api_key: String) -> Result<Self, String> {
        // Validate URL
        if !url.starts_with("https://") {
            return Err("URL must use HTTPS".to_string());
        }
        
        // Validate retries
        if max_retries > 5 {
            return Err("Max retries cannot exceed 5".to_string());
        }
        
        // Validate timeout
        if timeout_seconds < 1 || timeout_seconds > 60 {
            return Err("Timeout must be between 1 and 60 seconds".to_string());
        }
        
        // Validate API key length
        if api_key.len() < 16 {
            return Err("API key must be at least 16 characters".to_string());
        }
        
        Ok(ApiConfig {
            url,
            max_retries,
            timeout_seconds,
            api_key,
        })
    }
    
    // Load from file with ? operator
    fn from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        // Parse config (simplified)
        let lines: Vec<&str> = content.lines().collect();
        
        if lines.len() < 4 {
            return Err("Invalid config file".into());
        }
        
        Ok(ApiConfig {
            url: lines[0].to_string(),
            max_retries: lines[1].parse()?,
            timeout_seconds: lines[2].parse()?,
            api_key: lines[3].to_string(),
        })
    }
}

fn combined_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=========================================================");
    println!("PART 3: COMBINED EXAMPLE - API Config");
    println!("=========================================================");
    
    // Create valid config
    let config = ApiConfig::new(
        "https://api.example.com".to_string(),
        3,
        30,
        "abcdefghijklmnop".to_string(),
    )?;
    
    println!("  ✅ Valid config: {:?}", config);
    
    // Try invalid config
    match ApiConfig::new(
        "http://insecure.com".to_string(),  // ❌ Not HTTPS
        10,                                  // ❌ Too many retries
        0,                                   // ❌ Timeout too low
        "short".to_string(),                 // ❌ API key too short
    ) {
        Ok(c) => println!("  ✅ Config: {:?}", c),
        Err(e) => println!("  ❌ Invalid config: {}", e),
    }
    
    // Try loading from file (commented - would need config.txt)
    // let file_config = ApiConfig::from_file("config.txt")?;
    // println!("  ✅ Loaded from file: {:?}", file_config);
    
    Ok(())
}

// -----------------------------------------------------------------------------
// PART 4: ERROR HANDLING PROGRESSION
// -----------------------------------------------------------------------------

fn error_handling_progression() {
    println!("\n=========================================================");
    println!("PART 4: ERROR HANDLING PROGRESSION");
    println!("=========================================================");
    
    println!("STAGE 1: Prototype - unwrap/expect");
    println!("  let guess = Guess::new_panic(50);  // Panics on error");
    println!("  let data = fetch_data(url).unwrap();  // Crash on error");
    
    println!("\nSTAGE 2: Basic error handling with match");
    println!("  match Guess::new(50) {{");
    println!("      Ok(g) => use_guess(g),");
    println!("      Err(e) => eprintln!(\"Error: {{}}\", e),");
    println!("  }}");
    
    println!("\nSTAGE 3: Propagate errors with ?");
    println!("  fn process() -> Result<(), Error> {{");
    println!("      let guess = Guess::new(50)?;");
    println!("      let data = fetch_data(url)?;");
    println!("      Ok(())");
    println!("  }}");
    
    println!("\nSTAGE 4: Custom error types with context");
    println!("  enum ApiError {{ RateLimited, NotFound, Validation(String) }}");
    println!("  impl std::fmt::Display for ApiError {{ ... }}");
}

// -----------------------------------------------------------------------------
// MAIN FUNCTION
// -----------------------------------------------------------------------------
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=========================================================");
    println!("ADVANCED ERROR HANDLING PATTERNS IN RUST");
    println!("=========================================================");
    
    // Part 1: Rate Limiting
    rate_limiting_example();
    
    // Part 2: Custom Types
    guess_example();
    
    // Part 3: Combined Example
    combined_example()?;
    
    // Part 4: Progression
    error_handling_progression();
    
    println!("\n=========================================================");
    println!("📌 KEY PATTERNS SUMMARY");
    println!("=========================================================");
    println!("| Pattern              | Use Case                          | Example                  |");
    println!("|----------------------|-----------------------------------|--------------------------|");
    println!("| Retry with backoff   | Rate limiting, transient errors  | fetch_data_with_retry() |");
    println!("| Custom error types   | Domain-specific errors           | HttpError, ApiError      |");
    println!("| Validation in new()  | Ensure type invariants           | Guess::new()             |");
    println!("| FromFile with ?      | Load and validate config         | ApiConfig::from_file()   |");
    println!("| ? operator           | Propagate errors upward          | func()?                  |");
    println!("=========================================================");
    
    Ok(())
}

// =============================================================================
// BONUS: TEST EXAMPLES
// =============================================================================
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_guess_valid() {
        assert!(Guess::new(50).is_ok());
    }
    
    #[test]
    fn test_guess_invalid() {
        assert!(Guess::new(150).is_err());
    }
    
    #[test]
    #[should_panic(expected = "between 1 and 100")]
    fn test_guess_panic() {
        Guess::new_panic(200);
    }
    
    #[test]
    fn test_retry_success() {
        let result = fetch_data_with_retry("https://api.example.com/data", 3);
        assert!(result.is_ok());
    }
}

// =============================================================================
// QUICK REFERENCE - guess.rs
// =============================================================================
/*
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Result<Guess, String> {
        if value < 1 || value > 100 {
            Err(format!("Guess must be 1-100, got {}", value))
        } else {
            Ok(Guess { value })
        }
    }
    
    pub fn value(&self) -> i32 {
        self.value
    }
}

fn main() {
    match Guess::new(50) {
        Ok(guess) => println!("Valid: {}", guess.value()),
        Err(e) => println!("Error: {}", e),
    }
}
*/