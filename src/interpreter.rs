use std::collections::HashMap;
use crate::ast::{Statement, Expression, Literal, BinaryOperator};

#[derive(Debug, Clone)]
pub enum Value {
    Number(i64),
    String(String),
    Boolean(bool),
    Null,
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "{}", s),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Null => write!(f, "null"),
        }
    }
}

pub struct Interpreter {
    variables: HashMap<String, Value>,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            variables: HashMap::new(),
        }
    }
    
    pub fn execute(&mut self, statements: Vec<Statement>) -> Result<(), String> {
        for statement in statements {
            self.execute_statement(statement)?;
        }
        Ok(())
    }
    
    fn execute_statement(&mut self, statement: Statement) -> Result<(), String> {
        match statement {
            Statement::Assignment { name, value } => {
                let val = self.evaluate_expression(value)?;
                self.variables.insert(name, val);
                Ok(())
            }
            Statement::Print(expr) => {
                let val = self.evaluate_expression(expr)?;
                println!("{}", val);
                Ok(())
            }
            Statement::Expression(expr) => {
                self.evaluate_expression(expr)?;
                Ok(())
            }
        }
    }
    
    fn evaluate_expression(&mut self, expr: Expression) -> Result<Value, String> {
        match expr {
            Expression::Literal(lit) => self.evaluate_literal(lit),
            Expression::Identifier(name) => {
                Ok(self.variables.get(&name).cloned().unwrap_or(Value::Null))
            }
            Expression::BinaryOp { left, op, right } => {
                let left_val = self.evaluate_expression(*left)?;
                let right_val = self.evaluate_expression(*right)?;
                self.apply_binary_op(left_val, op, right_val)
            }
            Expression::FunctionCall { name, args: _ } => {
                Err(format!("Unknown function: {}", name))
            }
        }
    }
    
    fn evaluate_literal(&self, lit: Literal) -> Result<Value, String> {
        match lit {
            Literal::Number(n) => Ok(Value::Number(n)),
            Literal::String(s) => Ok(Value::String(s)),
            Literal::Boolean(b) => Ok(Value::Boolean(b)),
            Literal::Null => Ok(Value::Null),
        }
    }
    
    fn apply_binary_op(&self, left: Value, op: BinaryOperator, right: Value) -> Result<Value, String> {
        match (left, right) {
            (Value::Number(l), Value::Number(r)) => {
                match op {
                    BinaryOperator::Add => Ok(Value::Number(l + r)),
                    BinaryOperator::Subtract => Ok(Value::Number(l - r)),
                    BinaryOperator::Multiply => Ok(Value::Number(l * r)),
                    BinaryOperator::Divide => {
                        if r == 0 {
                            Err("Division by zero".to_string())
                        } else {
                            Ok(Value::Number(l / r))
                        }
                    }
                }
            }
            (Value::String(l), Value::String(r)) => {
                match op {
                    BinaryOperator::Add => Ok(Value::String(format!("{}{}", l, r))),
                    _ => Err(format!("Operation {:?} not supported for strings", op)),
                }
            }
            (l, r) => {
                Err(format!("Type mismatch: cannot apply {:?} to {:?} and {:?}", op, l, r))
            }
        }
    }
}