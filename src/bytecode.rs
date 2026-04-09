use crate::ast::Literal;

#[derive(Debug, Clone)]
pub enum Instruction {
    Constant(Literal),
    LoadGlobal(usize),
    StoreGlobal(usize),
    Add,
    Subtract,
    Multiply,
    Divide,
    AddInt,
    SubtractInt,
    MultiplyInt,
    DivideInt,
    IncGlobal(usize),
    AddGlobal(usize, i64),
    Equals,
    NotEquals,
    GreaterThan,
    LessThan,
    GreaterOrEqual,
    LessOrEqual,
    And,
    Or,
    Not,
    Print,
    Pop,
    Jump(usize),
    JumpIfFalse(usize),
}

pub type Bytecode = Vec<Instruction>;
