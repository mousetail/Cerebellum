pub mod run;
pub mod builtins;
use crate::ast;

pub struct ProgramState {
    constants: Vec<ProgramValue>,
    variables: Vec<ProgramValue>,
}

pub struct Generator {
    iterator: Box<dyn Iterator<Item = ProgramValue>>,
}

pub enum Function {
    BuiltInFunction(Box<dyn Fn([ProgramValue;3], ProgramState)->ProgramValue>),
    DynamicFunction(Box<ast::AstNode>)
}

pub struct Mapping {
    mapping_function: Box<dyn Fn(i64)->ProgramValue>,
    length: Option<isize>
}

pub enum ProgramValue {
    Int(i64),
    Float(f64),
    Array(Vec<ProgramValue>),
    Generator(Generator),
    Mapping(Mapping),
    Function(Function),
}
