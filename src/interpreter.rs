use std::collections::HashMap;
use crate::ast::{Statement, Expression, Literal, BinaryOperator, UnaryOperator};

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
            Statement::If { branches, else_branch } => {
                for (condition, branch) in branches {
                    let cond_value = self.evaluate_expression(condition)?;
                    match cond_value {
                        Value::Boolean(true) => {
                            for stmt in branch {
                                self.execute_statement(stmt)?;
                            }
                            return Ok(());
                        }
                        Value::Boolean(false) => continue,
                        other => {
                            return Err(format!("If condition must be boolean, found {:?}", other));
                        }
                    }
                }
                if let Some(branch) = else_branch {
                    for stmt in branch {
                        self.execute_statement(stmt)?;
                    }
                }
                Ok(())
            }
            Statement::While { condition, body } => {
                loop {
                    let cond_value = self.evaluate_expression(condition.clone())?;
                    match cond_value {
                        Value::Boolean(true) => {
                            for stmt in body.clone() {
                                self.execute_statement(stmt)?;
                            }
                        }
                        Value::Boolean(false) => break,
                        other => {
                            return Err(format!("While condition must be boolean, found {:?}", other));
                        }
                    }
                }
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
            Expression::UnaryOp { op, expr } => {
                let value = self.evaluate_expression(*expr)?;
                match op {
                    UnaryOperator::Not => match value {
                        Value::Boolean(b) => Ok(Value::Boolean(!b)),
                        other => Err(format!("not operation requires boolean, found {:?}", other)),
                    },
                }
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
                    BinaryOperator::Equals => Ok(Value::Boolean(l == r)),
                    BinaryOperator::NotEquals => Ok(Value::Boolean(l != r)),
                    BinaryOperator::GreaterThan => Ok(Value::Boolean(l > r)),
                    BinaryOperator::LessThan => Ok(Value::Boolean(l < r)),
                    BinaryOperator::GreaterOrEqual => Ok(Value::Boolean(l >= r)),
                    BinaryOperator::LessOrEqual => Ok(Value::Boolean(l <= r)),
                    _ => Err(format!("Operation {:?} not supported for numbers", op)),
                }
            }
            (Value::String(l), Value::String(r)) => {
                match op {
                    BinaryOperator::Add => Ok(Value::String(format!("{}{}", l, r))),
                    BinaryOperator::Equals => Ok(Value::Boolean(l == r)),
                    _ => Err(format!("Operation {:?} not supported for strings", op)),
                }
            }
            (Value::Boolean(l), Value::Boolean(r)) => {
                match op {
                    BinaryOperator::Equals => Ok(Value::Boolean(l == r)),
                    BinaryOperator::And => Ok(Value::Boolean(l && r)),
                    BinaryOperator::Or => Ok(Value::Boolean(l || r)),
                    _ => Err(format!("Operation {:?} not supported for booleans", op)),
                }
            }
            (Value::Null, Value::Null) => {
                match op {
                    BinaryOperator::Equals => Ok(Value::Boolean(true)),
                    _ => Err(format!("Operation {:?} not supported for null", op)),
                }
            }
            (l, r) => {
                match op {
                    BinaryOperator::Equals => Ok(Value::Boolean(false)),
                    _ => Err(format!("Type mismatch: cannot apply {:?} to {:?} and {:?}", op, l, r)),
                }
            }
        }
    }
}