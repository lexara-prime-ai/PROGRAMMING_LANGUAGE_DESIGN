use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::values::{BasicValue, BasicValueEnum, FunctionValue};
use inkwell::FloatPredicate;
use std::collections::HashMap;
use std::rc::Rc;

pub type Value<'ctx> = BasicValueEnum<'ctx>;

pub trait ExprAST {
    fn codegen<'ctx>(
        &self,
        context: &'ctx Context,
        builder: &'ctx Builder<'ctx>,
        module: &'ctx Module<'ctx>,
        named_values: &mut HashMap<String, Value<'ctx>>,
    ) -> Result<Value<'ctx>, String>;
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
    fn codegen<'ctx>(
        &self,
        context: &'ctx Context,
        _builder: &'ctx Builder<'ctx>, // `Builder` is not used in this function but included for consistency
        _module: &'ctx Module<'ctx>, // `Module` is not used in this function but included for consistency
        _named_values: &mut HashMap<String, Value<'ctx>>, // `named_values` is not used in this function but included for consistency
    ) -> Result<Value<'ctx>, String> {
        let float_type = context.f64_type();
        Ok(float_type.const_float(self.value).as_basic_value_enum())
    }
}

// 1. VariableExprAST
pub struct VariableExprAST {
    pub name: String,
}

impl VariableExprAST {
    pub fn new(name: String) -> Self {
        VariableExprAST { name }
    }
}

impl ExprAST for VariableExprAST {
    fn codegen<'ctx>(
        &self,
        _context: &'ctx Context,
        _builder: &'ctx Builder<'ctx>,
        _module: &'ctx Module<'ctx>,
        named_values: &mut HashMap<String, Value<'ctx>>,
    ) -> Result<Value<'ctx>, String> {
        // Look up variable in symbol table (named_values)
        if let Some(value) = named_values.get(&self.name) {
            Ok(*value)
        } else {
            Err(format!("Unknown variable name: {}", self.name))
        }
    }
}

// 2. BinaryExprAST
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
    fn codegen<'ctx>(
        &self,
        context: &'ctx Context,
        builder: &'ctx Builder<'ctx>,
        module: &'ctx Module<'ctx>,
        named_values: &mut HashMap<String, Value<'ctx>>,
    ) -> Result<Value<'ctx>, String> {
        let lhs_val = self.lhs.codegen(context, builder, module, named_values)?;
        let rhs_val = self.rhs.codegen(context, builder, module, named_values)?;

        let float_type = context.f64_type();

        // Apply the binary operator
        let result = match self.op {
            '+' => builder.build_float_add(
                lhs_val.into_float_value(),
                rhs_val.into_float_value(),
                "addtmp",
            ),
            '-' => builder.build_float_sub(
                lhs_val.into_float_value(),
                rhs_val.into_float_value(),
                "subtmp",
            ),
            '*' => builder.build_float_mul(
                lhs_val.into_float_value(),
                rhs_val.into_float_value(),
                "multmp",
            ),
            '/' => builder.build_float_div(
                lhs_val.into_float_value(),
                rhs_val.into_float_value(),
                "divtmp",
            ),
            '<' => {
                let cmp = builder.build_float_compare(
                    FloatPredicate::ULT,
                    lhs_val.into_float_value(),
                    rhs_val.into_float_value(),
                    "cmptmp",
                );
                // Handle the Result from build_float_compare
                match cmp {
                    Ok(cmp_val) => {
                        builder.build_unsigned_int_to_float(cmp_val, float_type, "booltmp")
                    }
                    Err(e) => return Err(format!("Comparison error: {:?}", e)),
                }
            }
            _ => return Err(format!("Unknown binary operator: {}", self.op)),
        };

        Ok(result.unwrap().as_basic_value_enum())
    }
}

// 3. CallExprAST
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
    fn codegen<'ctx>(
        &self,
        context: &'ctx Context,
        builder: &'ctx Builder<'ctx>,
        module: &'ctx Module<'ctx>,
        named_values: &mut HashMap<String, Value<'ctx>>,
    ) -> Result<Value<'ctx>, String> {
        // Look up the function in the module
        let function = module
            .get_function(&self.callee)
            .ok_or_else(|| format!("Unknown function referenced: {}", self.callee))?;

        // Check that the function has the correct number of arguments
        if function.count_params() as usize != self.args.len() {
            return Err(format!(
                "Incorrect number of arguments passed to function: {}",
                self.callee
            ));
        }

        // Generate code for the arguments
        let mut arg_values = vec![];
        for arg in &self.args {
            arg_values.push(arg.codegen(context, builder, module, named_values)?);
        }

        // Convert arg_values from BasicValueEnum to BasicMetadataValueEnum
        let arg_values: Vec<inkwell::values::BasicMetadataValueEnum<'ctx>> = arg_values
            .iter()
            .map(|v| v.as_basic_value_enum().into())
            .collect();

        // Call the function
        let call = builder.build_call(function, &arg_values[..], "calltmp");

        // Extract and return the result from the call
        let result = call
            .unwrap()
            .try_as_basic_value()
            .left()
            .ok_or_else(|| "Failed to get return value from function call".to_string())?;

        Ok(result)
    }
}
