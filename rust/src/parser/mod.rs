use crate::ast::*;
use crate::lexer::*;

use std::collections::HashMap;
use std::rc::Rc;

pub struct Parser<'a> {
    pub lexer: &'a mut Lexer,
    pub current_token: Token,
    pub binop_precedence: HashMap<char, i32>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut Lexer) -> Self {
        Parser {
            lexer,
            current_token: Token::Unknown(' '),
            binop_precedence: HashMap::new(),
        }
    }

    pub fn get_next_token(&mut self) {
        self.current_token = self.lexer.get_token();
    }

    fn get_tok_precedence(&self) -> Option<i32> {
        if let Token::Unknown(op) = self.current_token {
            self.binop_precedence.get(&op).cloned()
        } else {
            None
        }
    }

    pub fn parse_number_expr(&mut self) -> Rc<dyn ExprAST> {
        if let Token::Number(val) = self.current_token {
            self.get_next_token(); // consume the number
            Rc::new(NumberExprAST::new(val))
        } else {
            panic!("Expected number");
        }
    }

    pub fn parse_paren_expr(&mut self) -> Rc<dyn ExprAST> {
        self.get_next_token(); // consume '('
        let expr = self.parse_expression();
        if let Token::Unknown(')') = self.current_token {
            self.get_next_token(); // consume ')'
        } else {
            panic!("Expected ')'");
        }
        expr
    }

    pub fn parse_identifier_expr(&mut self) -> Rc<dyn ExprAST> {
        if let Token::Identifier(ref name) = self.current_token {
            let name_clone = name.clone();
            self.get_next_token(); // consume identifier

            if let Token::Unknown('(') = self.current_token {
                self.get_next_token(); // consume '('
                let mut args = Vec::new();
                if self.current_token != Token::Unknown(')') {
                    loop {
                        args.push(self.parse_expression());
                        if self.current_token == Token::Unknown(')') {
                            break;
                        }

                        if self.current_token != Token::Unknown(',') {
                            panic!("Expected ',' or ')'");
                        }
                        self.get_next_token();
                    }
                }
                self.get_next_token(); // consume ')'
                Rc::new(CallExprAST::new(name_clone, args))
            } else {
                Rc::new(VariableExprAST::new(name_clone))
            }
        } else {
            panic!("Expected identifier");
        }
    }

    pub fn parse_primary(&mut self) -> Rc<dyn ExprAST> {
        match self.current_token {
            Token::Identifier(_) => self.parse_identifier_expr(),
            Token::Number(_) => self.parse_number_expr(),
            Token::Unknown('(') => self.parse_paren_expr(),
            _ => panic!("Unknown token when expecting an expression"),
        }
    }

    pub fn parse_expression(&mut self) -> Rc<dyn ExprAST> {
        let lhs = self.parse_primary();
        self.parse_binop_rhs(0, lhs)
    }

    pub fn parse_binop_rhs(&mut self, expr_prec: i32, mut lhs: Rc<dyn ExprAST>) -> Rc<dyn ExprAST> {
        loop {
            let tok_prec = match self.get_tok_precedence() {
                Some(prec) => prec,
                None => return lhs,
            };

            if tok_prec < expr_prec {
                return lhs;
            }

            let bin_op = match self.current_token {
                Token::Unknown(op) => op,
                _ => panic!("Expected binary operator"),
            };
            self.get_next_token(); // consume binop

            let mut rhs = self.parse_primary();

            let next_prec = match self.get_tok_precedence() {
                Some(prec) => prec,
                None => -1,
            };

            if tok_prec < next_prec {
                rhs = self.parse_binop_rhs(tok_prec + 1, rhs);
            }

            lhs = Rc::new(BinaryExprAST::new(bin_op, lhs, rhs));
        }
    }
}
