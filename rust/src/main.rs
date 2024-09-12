#![allow(unused)]

mod ast;
mod lexer;
mod parser;

use crate::lexer::*;
use crate::parser::*;

use std::io::{self, Write};

fn main() {
    let mut lexer;
    let mut parser;

    // Binary operator precedence.
    let mut binop_precedence = std::collections::HashMap::new();
    binop_precedence.insert('<', 10);
    binop_precedence.insert('+', 20);
    binop_precedence.insert('-', 20);
    binop_precedence.insert('*', 40);

    // Main loop for interactive input
    loop {
        print!("ready> ");
        io::stdout().flush().unwrap(); // Ensure prompt is displayed immediately

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if input.trim().is_empty() {
            continue; // Skip empty lines
        }

        lexer = Lexer::new(input); // Recreate lexer for new input
        parser = Parser::new(&mut lexer);

        // Set operator precedence
        parser.binop_precedence = binop_precedence.clone();

        // Prime the first token
        parser.get_next_token();

        match parser.current_token {
            Token::Eof => break, // End of input
            Token::Def => {
                // Handle function definition
                parser.get_next_token(); // Move past 'def'
                let _ = parser.parse_expression(); // Parsing function body
                println!("Parsed a function definition.");
            }
            Token::Extern => {
                // Handle extern
                parser.get_next_token(); // Move past 'extern'
                let _ = parser.parse_expression(); // Parsing extern
                println!("Parsed an extern.");
            }
            _ => {
                // Handle expression
                let _ = parser.parse_expression(); // Parsing top-level expression
                println!("Parsed a top-level expression.");
            }
        }
    }
}

// To do - Documention (Referenc LLVM docs).
