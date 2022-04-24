use crate::tipo::Tipo;
use std::collections::HashMap;

// Struct created to store
// variable information
// on our directory of
// variables, stored in a
// hash map
#[derive(Debug)]
pub struct VarInfo {
    pub tipo: Tipo,
}

// Directory of Variables
// keeping varaible names as a key
// stored as a String,
// and variable information as value
// stored as a DirFuncInfo
pub type DirVar = HashMap<String, VarInfo>;
