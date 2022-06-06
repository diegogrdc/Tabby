use crate::dir_var::DirVar;
use crate::tipo::Tipo;
use std::collections::HashMap;
/*
Struct created to store
any relevant function
information on our directory
of functions stored in a
hash map
*/
#[derive(Debug, Clone)]
pub struct FuncInfo {
    pub tipo: Tipo,
    pub dir_var: DirVar,
    pub pos_init: i32,
    pub size_loc: [i32; 3],
    pub size_tmp: [i32; 3],
    pub params: Vec<Tipo>,
    pub params_addrs: Vec<i32>,
}

// Directory of Functions,
// keeping funcion names as a key
// stored as a String,
// and function information as value
// stored as a DirFuncInfo
pub type DirFunc = HashMap<String, FuncInfo>;
