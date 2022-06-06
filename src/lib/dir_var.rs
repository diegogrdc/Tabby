use crate::tipo::Tipo;
use std::collections::HashMap;

/*
Struct created to store
any relevant
variable information
on our directory of
variables, stored in a
hash map
*/
#[derive(Debug, Clone)]
pub struct VarInfo {
    pub tipo: Tipo,
    pub addr: i32,
    pub dim: Dim,
}

// Directory of Variables
// keeping varaible names as a key
// stored as a String,
// and variable information as value
// stored as a DirFuncInfo
pub type DirVar = HashMap<String, VarInfo>;

// Type defined to
// easily know the dimensions
// of a variable defined
#[derive(Debug, Clone, PartialEq)]
pub enum Dim {
    Single,
    Arr(i32),
    Mat(i32, i32),
}
