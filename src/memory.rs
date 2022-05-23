#[derive(Debug)]
pub struct Memory {
    pub int_mem: Vec<i32>,
    pub float_mem: Vec<f64>,
    pub bool_mem: Vec<bool>,
    pub int_off: i32,
    pub float_off: i32,
    pub bool_off: i32,
}

impl Memory {
    pub fn new(
        sz_int: i32,
        off_int: i32,
        sz_float: i32,
        off_float: i32,
        sz_bool: i32,
        off_bool: i32,
    ) -> Memory {
        Memory {
            int_mem: vec![0; sz_int as usize],
            float_mem: vec![0.0; sz_float as usize],
            bool_mem: vec![false; sz_bool as usize],
            int_off: off_int,
            float_off: off_float,
            bool_off: off_bool,
        }
    }

    pub fn empty() -> Memory {
        Memory {
            int_mem: Vec::new(),
            float_mem: Vec::new(),
            bool_mem: Vec::new(),
            int_off: 0,
            float_off: 0,
            bool_off: 0,
        }
    }
}

#[derive(Debug)]
pub struct MemoryInfo {
    pub locs: [i32; 3],
    pub tmps: [i32; 3],
}
