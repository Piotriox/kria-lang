/// Abstract Syntax Tree definitions for Kria language

#[derive(Debug, Clone)]
pub enum Statement {
    Assignment {
        name: String,
        value: Expression,
    },
    Print(Expression),
    Expression(Expression),
}

#[derive(Debug, Clone)]
pub enum Expression {
    Literal(Literal),
    Identifier(String),
    BinaryOp {
        left: Box<Expression>,
        op: BinaryOperator,
        right: Box<Expression>,
    },
    FunctionCall {
        name: String,
        args: Vec<Expression>,
    },
}

#[derive(Debug, Clone)]
pub enum Literal {
    Number(i64),
    String(String),
    Boolean(bool),
    Null,
}

#[derive(Debug, Clone, Copy)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
}
