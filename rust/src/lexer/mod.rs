use std::io::{self, Read};

/// The lexer returns tokens [0-255] if it is an unknown character,
/// otherwise one of the following enum variants for known things.
pub enum Token {
    TokEof = -1,

    /// Comands.
    TokDef = -2,
    TokExtern = -3,

    /// Primary.
    TokIdentifier = -4,
    TokNumber = -5,
}

static mut IDENTIFIER_STR: String = String::new();
static mut NUM_VAL: f64 = 0.0;

/**
 * 
 * Each token returned by our lexer will either be one of the 
 * Token enum values or it will be an ‘unknown’ character like ‘+’, 
 * which is returned as its ASCII value. If the current token is an 
 * identifier, the IdentifierStr global variable holds the name of 
 * the identifier. If the current token is a numeric literal (like 1.0), 
 * NUM_VAL holds its value. We use global variables for simplicity, but 
 * this is not the best choice for a real language implementation :).
 * 
 */
pub fn get_token() -> &'static str {
    static mut LAST_CHAR: char = ' ';

    unsafe {
        // Read from stdin until a non-whitespace character
        // is found.
        let stdin = io::stdin();
        let mut stdin_lock = stdin.lock();
        // Store each character.
        let mut buffer = [0; 1];

        // Skip whitespace characters.
        while LAST_CHAR.is_whitespace() {
            
        }
    }

    todo!()
}