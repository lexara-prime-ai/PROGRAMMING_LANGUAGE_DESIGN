use std::sync::Arc;

pub trait ExprAST {
    fn evaluate(&self) -> f64;
}

struct NumberExprAST {
    val: f64,
}

impl NumberExprAST {
    fn new(val: f64) -> Self {
        Self { val }
    }
}

impl ExprAST for NumberExprAST {
    fn evaluate(&self) -> f64 {
        self.val
    }
}

struct BinaryExprAST {
    op: char,
    lhs: Box<dyn ExprAST>,
    rhs: Box<dyn ExprAST>,
}

impl BinaryExprAST {
    fn new(op: char, lhs: Box<dyn ExprAST>, rhs: Box<dyn ExprAST>) -> Self {
        Self { op, lhs, rhs }
    }
}

impl ExprAST for BinaryExprAST {
    fn evaluate(&self) -> f64 {
        // To do - Parse binary expressions.
        0.0
    }
}

struct CallExprAST {
    callee: String,
    args: Vec<Box<dyn ExprAST>>,
}

impl CallExprAST {
    fn new(callee: String, args: Vec<Box<dyn ExprAST>>) -> Self {
        Self { callee, args }
    }
}

impl ExprAST for CallExprAST {
    fn evaluate(&self) -> f64 {
        // To do - Implement logic for function call.
        0.0
    }
}

struct PrototypeAST {
    name: String,
    args: Vec<String>,
}

impl PrototypeAST {
    fn new(name: String, args: Vec<String>) -> Self {
        Self { name, args }
    }

    fn get_name(&self) -> &String {
        &self.name
    }
}

struct FunctionAST {
    proto: Arc<PrototypeAST>,
    body: Box<dyn ExprAST>,
}

impl FunctionAST {
    fn new(proto: Arc<PrototypeAST>, body: Box<dyn ExprAST>) -> Self {
        Self { proto, body }
    }
}
