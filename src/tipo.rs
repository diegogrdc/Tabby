use crate::ast;

// Enum used to store functions
// and variable types in a
// structured way, instead of
// storing as string
#[derive(Debug, PartialEq)]
pub enum Tipo {
    Int,
    ArrInt(i32),
    MatInt(i32, i32),
    Float,
    ArrFloat(i32),
    MatFloat(i32, i32),
    Bool,
    ArrBool(i32),
    MatBool(i32, i32),
    Void,
    ParamRef,
    Program,
}

pub fn tipo_from_type(typ: &ast::Type) -> Tipo {
    match typ {
        ast::Type::Int => Tipo::Int,
        ast::Type::Float => Tipo::Float,
        ast::Type::Bool => Tipo::Bool,
    }
}

pub fn arr_tipo_from_type(typ: &ast::Type, sz: i32) -> Tipo {
    match typ {
        ast::Type::Int => Tipo::ArrInt(sz),
        ast::Type::Float => Tipo::ArrFloat(sz),
        ast::Type::Bool => Tipo::ArrBool(sz),
    }
}

pub fn mat_tipo_from_type(typ: &ast::Type, sz1: i32, sz2: i32) -> Tipo {
    match typ {
        ast::Type::Int => Tipo::MatInt(sz1, sz2),
        ast::Type::Float => Tipo::MatFloat(sz1, sz2),
        ast::Type::Bool => Tipo::MatBool(sz1, sz2),
    }
}
