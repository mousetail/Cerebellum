pub trait Operator: Sized + Copy + Clone + Eq + PartialEq + std::fmt::Debug {
    fn from_symbol(symbol: &str) -> Option<Self>;
    fn all() -> Vec<Self>;
}

#[macro_export]
macro_rules! derive_operator {
    ($(#[$($attr:tt)*])* $vis:vis enum $name:ident { $(#[symbol = $symbol:expr] $variant:ident,)* }) => {
        $(#[$($attr)*])*
        $vis enum $name { $($variant,)* }
        
        impl $crate::ast::operator_macro::Operator for $name {
            fn from_symbol(symbol: &str) -> Option<Self> {
                match symbol {
                    $($symbol => Some($name::$variant),)*
                    _ => None
                }
            }

            fn all() -> Vec<Self> {
                vec![
                    $($name::$variant,)*
                ]
            }
        }
   };
}