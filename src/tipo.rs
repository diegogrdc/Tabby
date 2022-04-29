use crate::ast;

// Enum used to store functions
// and variable types in a
// structured way, instead of
// storing as string
#[derive(Debug, PartialEq, Clone)]
pub enum Tipo {
    Int,
    Float,
    Bool,
    Void,
    Program,
}

pub fn tipo_from_type(typ: &ast::Type) -> Tipo {
    match typ {
        ast::Type::Int => Tipo::Int,
        ast::Type::Float => Tipo::Float,
        ast::Type::Bool => Tipo::Bool,
    }
}
