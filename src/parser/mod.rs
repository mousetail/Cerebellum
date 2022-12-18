use crate::ast::{operator_macro::{Operator}, AstNode, UnaryOperator, BinaryOperator, TernaryOperator, QuadrenaryOperator};


fn to_ast(item: &[&str], start: &mut usize) -> AstNode {
    let head = item[*start];
    *start+=1;
    if let Some(u) = UnaryOperator::from_symbol(head) {
        AstNode::Unary{kind: u, first:Box::new(to_ast(item, start))}
    } else if let Some(u) = BinaryOperator::from_symbol(head) {
        AstNode::Binary{kind: u, first: Box::new(to_ast(item, start)), second: Box::new(to_ast(item, start))}
    } else {
        panic!("Invalid AST node");
    }
}