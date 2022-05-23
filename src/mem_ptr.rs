#[derive(Debug)]
pub enum MemPtr<'a> {
    Int(&'a mut i32),
    Float(&'a mut f64),
    Bool(&'a mut bool),
}

#[derive(Debug)]
pub enum MemVal {
    Int(i32),
    Float(f64),
    Bool(bool),
}
