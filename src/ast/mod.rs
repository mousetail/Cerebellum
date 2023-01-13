pub mod operator_macro;
use operator_macro::Operator;

#[derive(Debug, PartialEq)]
pub enum AstNode {
    Nullary {
        kind: NullaryOperator,
    },
    Unary {
        kind: UnaryOperator,
        first: Box<AstNode>,
    },
    Binary {
        kind: BinaryOperator,
        first: Box<AstNode>,
        second: Box<AstNode>,
    },
    Ternary {
        kind: TernaryOperator,
        first: Box<AstNode>,
        second: Box<AstNode>,
        third: Box<AstNode>,
    },
    Quadrenary {
        kind: QuadrenaryOperator,
        first: Box<AstNode>,
        second: Box<AstNode>,
        third: Box<AstNode>,
        fourth: Box<AstNode>,
    },
}

impl AstNode {
    fn getSymbol(self) -> AstSymbol {
        match self {
            Self::Nullary { kind } => AstSymbol::Nullary(kind),
            Self::Unary { kind, .. } => AstSymbol::Unary(kind),
            Self::Binary { kind, .. } => AstSymbol::Binary(kind),
            Self::Ternary { kind, .. } => AstSymbol::Ternary(kind),
            Self::Quadrenary { kind, .. } => AstSymbol::Quadrenary(kind),
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone, Eq)]
pub enum AstSymbol {
    Nullary(NullaryOperator),
    Unary(UnaryOperator),
    Binary(BinaryOperator),
    Ternary(TernaryOperator),
    Quadrenary(QuadrenaryOperator),
}

impl AstSymbol {
    pub fn from_verbose_name(word: &str) -> AstSymbol {
        if let Some(u) = NullaryOperator::from_symbol(word) {
            Self::Nullary(u)
        } else if let Some(u) = UnaryOperator::from_symbol(word) {
            Self::Unary(u)
        } else if let Some(u) = BinaryOperator::from_symbol(word) {
            Self::Binary(u)
        } else if let Some(u) = TernaryOperator::from_symbol(word) {
            Self::Ternary(u)
        } else if let Some(u) = QuadrenaryOperator::from_symbol(word) {
            Self::Quadrenary(u)
        } else {
            panic!("Invalid AST node: {:?}", word);
        }
    }
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
        #[symbol = "3"]
        Three,
    }
}

crate::derive_operator! {
    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    pub enum BinaryOperator {
        #[symbol = "add"]
        Add,
    }
}

crate::derive_operator! {
    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    pub enum TernaryOperator {}
}

crate::derive_operator! {
    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    pub enum QuadrenaryOperator {}
}
