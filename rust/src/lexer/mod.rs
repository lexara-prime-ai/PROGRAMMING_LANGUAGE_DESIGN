use std::collections::HashMap;
use std::io::{self, Read};

use std::str::FromStr;

/// The lexer returns tokens [0-255] if it is an unknown character,
/// otherwise one of the following enum variants for known things.
pub enum Token {
    EOF,

    /// Comands.
    Def,
    Extern,

    /// Primary.
    Identifier(String),
    Number(f64),
    Character(char),
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
pub fn get_token() -> Token {
    static mut LAST_CHAR: char = ' ';
    // Buffer to store each character.
    let mut buffer = [0; 1];

    unsafe {
        // Skip whitespace characters.
        while LAST_CHAR.is_whitespace() {
            // Read from stdin until a non-whitespace
            // character is found.
            io::stdin().read_exact(&mut buffer).unwrap();
            LAST_CHAR = buffer[0] as char;
        }

        // Identifier: [a-zA-Z][a-zA-Z0-9]*
        if LAST_CHAR.is_alphabetic() {
            IDENTIFIER_STR = LAST_CHAR.to_string();

            while io::stdin().read_exact(&mut buffer).is_ok() {
                LAST_CHAR = buffer[0] as char;

                if LAST_CHAR.is_alphanumeric() {
                    IDENTIFIER_STR.push(LAST_CHAR);
                } else {
                    break;
                }
            }

            if IDENTIFIER_STR == "def" {
                return Token::Def;
            }

            if IDENTIFIER_STR == "extern" {
                return Token::Extern;
            }
            return Token::Identifier(IDENTIFIER_STR.clone());
        }

        if LAST_CHAR.is_digit(10) || LAST_CHAR == '.' {
            let mut num_str = String::new();

            while io::stdin().read_exact(&mut buffer).is_ok() {
                LAST_CHAR = buffer[0] as char;

                if LAST_CHAR.is_digit(10) || LAST_CHAR == '.' {
                    num_str.push(LAST_CHAR);
                } else {
                    break;
                }
            }

            NUM_VAL = f64::from_str(&num_str).unwrap();
            return Token::Number(NUM_VAL);
        }

        // Parsing comments.
        if LAST_CHAR == '#' {
            
        }
    }

    todo!()
}
