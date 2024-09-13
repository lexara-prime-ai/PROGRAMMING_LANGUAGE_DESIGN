#![allow(unused)]

mod ast;
mod lexer;
mod parser;

use crate::lexer::*;
use crate::parser::*;

use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lexer = Lexer::new(input);

    let mut parser = Parser::new(&mut lexer);

    // Binary operator precedence.
    parser.binop_precedence.insert('<', 10);
    parser.binop_precedence.insert('+', 20);
    parser.binop_precedence.insert('-', 20);
    parser.binop_precedence.insert('*', 40);

    // Prime the first token.
    parser.get_next_token();

    // Main loop.
    loop {
        print!("ready> ");
        match parser.current_token {
            Token::Eof => break,
            Token::Def => {
                // Handle function definition
            }
            Token::Extern => {
                // Handle extern
            }
            _ => {
                // Handle expression
                parser.parse_expression();
            }
        }
    }
}
