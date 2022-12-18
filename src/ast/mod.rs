pub mod operator_macro;

pub enum AstNode {
    Nullary{kind: NullaryOperator},
    Unary{kind: UnaryOperator, first: Box<AstNode>},
    Binary{kind: BinaryOperator, first: Box<AstNode>, second: Box<AstNode>},
    Ternary{kind: TernaryOperator, first: Box<AstNode>, second: Box<AstNode>, third: Box<AstNode>},
    Quadrenary{kind: TernaryOperator, first: Box<AstNode>, second: Box<AstNode>, third: Box<AstNode>, fourth: Box<AstNode>}
}

crate::derive_operator! {
    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    pub enum UnaryOperator {
        #[symbol = "first"]
        FirstVar,
        #[symbol = "second"]
        SecondVar,
        #[symbol = "third"]
        ThirdVar,
    }
}

crate::derive_operator! {
    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    pub enum NullaryOperator {
        #[symbol = "1"]
        One,
        #[symbol = "2"]
        Two,
    }
}

crate::derive_operator! {
    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    pub enum BinaryOperator {
        #[symbol = "add"]
        Add,
    }
}  

pub enum TernaryOperator {

}

pub enum QuadrenaryOperator {

}