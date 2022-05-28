#[derive(Debug)]
pub enum MemPtr<'a> {
    Int(&'a mut i32),
    Float(&'a mut f64),
    Bool(&'a mut bool),
}

impl MemPtr<'_> {
    pub fn as_int(&mut self) -> &mut i32 {
        if let MemPtr::Int(val) = self {
            val
        } else {
            eprintln!(
                "\nEXECUTION ERROR: unwrap MemPtr \"as_int\" expected int, got {:?}",
                self
            );
            panic!();
        }
    }
    pub fn as_float(&mut self) -> &mut f64 {
        if let MemPtr::Float(val) = self {
            val
        } else {
            eprintln!(
                "\nEXECUTION ERROR: unwrap MemPtr \"as_int\" expected int, got {:?}",
                self
            );
            panic!();
        }
    }
    pub fn as_bool(&mut self) -> &mut bool {
        if let MemPtr::Bool(val) = self {
            val
        } else {
            eprintln!(
                "\nEXECUTION ERROR: unwrap MemPtr \"as_int\" expected int, got {:?}",
                self
            );
            panic!();
        }
    }
}

#[derive(Debug)]
pub enum MemVal {
    Int(i32),
    Float(f64),
    Bool(bool),
}

impl MemVal {
    pub fn as_int(&self) -> i32 {
        match self {
            MemVal::Int(val) => *val,
            MemVal::Float(val) => *val as i32,
            MemVal::Bool(val) => *val as i32,
        }
    }
    pub fn as_float(&self) -> f64 {
        match self {
            MemVal::Int(val) => *val as f64,
            MemVal::Float(val) => *val,
            MemVal::Bool(_) => {
                eprintln!("\nEXECUTION ERROR: Cannot cast bool as float\n");
                panic!();
            }
        }
    }
    pub fn as_bool(&self) -> bool {
        match self {
            MemVal::Int(val) => *val != 0,
            MemVal::Bool(val) => *val,
            MemVal::Float(_) => {
                eprintln!("\nEXECUTION ERROR: Cannot cast float as bool\n");
                panic!();
            }
        }
    }
}
