use crate::ast::{
    operator_macro::Operator, AstNode, AstSymbol, BinaryOperator, NullaryOperator,
    QuadrenaryOperator, TernaryOperator, UnaryOperator,
};

pub fn to_ast(item: &[AstSymbol], start: &mut usize) -> AstNode {
    let head = item[*start];
    *start += 1;
    match head {
        AstSymbol::Nullary(u) => AstNode::Nullary { kind: u },
        AstSymbol::Unary(u) => AstNode::Unary {
            kind: u,
            first: Box::new(to_ast(item, start)),
        },
        AstSymbol::Binary(u) => AstNode::Binary {
            kind: u,
            first: Box::new(to_ast(item, start)),
            second: Box::new(to_ast(item, start)),
        },
        AstSymbol::Ternary(u) => AstNode::Ternary {
            kind: u,
            first: Box::new(to_ast(item, start)),
            second: Box::new(to_ast(item, start)),
            third: Box::new(to_ast(item, start)),
        },
        AstSymbol::Quadrenary(u) => AstNode::Quadrenary {
            kind: u,
            first: Box::new(to_ast(item, start)),
            second: Box::new(to_ast(item, start)),
            third: Box::new(to_ast(item, start)),
            fourth: Box::new(to_ast(item, start)),
        },
    }
    // if let Some(u) = NullaryOperator::from_symbol(head) {
    //     AstNode::Nullary { kind: u }
    // } else if let Some(u) = UnaryOperator::from_symbol(head) {
    //     AstNode::Unary {
    //         kind: u,
    //         first: Box::new(to_ast(item, start)),
    //     }
    // } else if let Some(u) = BinaryOperator::from_symbol(head) {
    //     AstNode::Binary {
    //         kind: u,
    //         first: Box::new(to_ast(item, start)),
    //         second: Box::new(to_ast(item, start)),
    //     }
    // } else {
    //     panic!("Invalid AST node: {:?}", head);
    // }
}
