use std::rc::Rc;

pub trait ExprAST {
    fn as_any(&self) -> &dyn std::any::Any;
}

pub struct NumberExprAST {
    pub value: f64,
}

impl NumberExprAST {
    pub fn new(value: f64) -> Self {
        NumberExprAST { value }
    }
}

impl ExprAST for NumberExprAST {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

pub struct VariableExprAST {
    pub name: String,
}

impl VariableExprAST {
    pub fn new(name: String) -> Self {
        VariableExprAST { name }
    }
}

impl ExprAST for VariableExprAST {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

pub struct BinaryExprAST {
    pub op: char,
    pub lhs: Rc<dyn ExprAST>,
    pub rhs: Rc<dyn ExprAST>,
}

impl BinaryExprAST {
    pub fn new(op: char, lhs: Rc<dyn ExprAST>, rhs: Rc<dyn ExprAST>) -> Self {
        BinaryExprAST { op, lhs, rhs }
    }
}

impl ExprAST for BinaryExprAST {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

pub struct CallExprAST {
    pub callee: String,
    pub args: Vec<Rc<dyn ExprAST>>,
}

impl CallExprAST {
    pub fn new(callee: String, args: Vec<Rc<dyn ExprAST>>) -> Self {
        CallExprAST { callee, args }
    }
}

impl ExprAST for CallExprAST {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
