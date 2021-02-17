/**
 * Prints usage notes for the applications
 */
pub fn print_usage() {
    println!("Usage: ttable <file script | pass directly option>")
}

/**
 * prints an error
 */
pub fn print_error(e: std::io::Error) {
    println!("Error: {}", e);
}

/**
 * enum for the input method
 */
pub enum In {
    Cli(String),
    Stdin,
    File(String)
}