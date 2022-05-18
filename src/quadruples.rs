// Enum used to store quadruples
// with a bit more logic than
// just 4 number arrays
// This way it has a meaning
// and it can be printed or saves
// as needed with a simple function
// if needed in the future
#[derive(Debug, PartialEq)]
pub enum Quadruple {
    Op(String, IdAddr, IdAddr, IdAddr),
    Assign(String, IdAddr, IdAddr),
    Read(String, IdAddr),
    Print(String, IdAddr),
    GoToF(IdAddr, i32),
    GoTo(i32),
    GoSub(String, i32),
    Init(i32),
    Era(String),
    Parameter(IdAddr, i32),
    EndFunc(),
    Return(),
    Temp(),
    Verify(IdAddr, i32),
    Deref(IdAddr, IdAddr),
}

pub type IdAddr = (String, i32);
