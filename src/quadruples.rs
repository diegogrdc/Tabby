// Enum used to store quadruples
// with a bit more logic than
// just 4 number arrays
// This way it has a meaning
// and it can be printed or saves
// as needed with a simple function
// if needed in the future
#[derive(Debug, PartialEq)]
pub enum Quadruple {
    Op(String, String, String, String),
    Assign(String, String, String),
    Read(String, String),
    Print(String, String),
}
