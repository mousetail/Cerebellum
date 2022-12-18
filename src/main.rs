mod ast;
mod parser;
use crate::ast::operator_macro::Operator;

fn main() {
    println!("Hello, world!");

    let operator = ast::UnaryOperator::from_symbol("first");
    println!("operator: {:?}", operator);
}
