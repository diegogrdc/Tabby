#[derive(Debug)]
pub struct CnstsMemory {
    pub int_cnst: Vec<i32>,
    pub float_cnst: Vec<f64>,
    pub bool_cnst: Vec<bool>,
    pub strlit_cnst: Vec<String>,
}

impl CnstsMemory {
    pub fn empty() -> CnstsMemory {
        CnstsMemory {
            int_cnst: Vec::new(),
            float_cnst: Vec::new(),
            bool_cnst: Vec::new(),
            strlit_cnst: Vec::new(),
        }
    }
}
