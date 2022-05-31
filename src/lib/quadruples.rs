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
    Era(String),
    Parameter(IdAddr, i32),
    EndFunc(),
    Return(),
    Temp(),
    Verify(IdAddr, i32),
    Deref(IdAddr, IdAddr),
    Statistics(String, IdAddr, IdAddr, IdAddr, i32, IdAddr),
    Plot(String, IdAddr, IdAddr, IdAddr, i32, i32, String),
    Rand(String, IdAddr),
}

impl Quadruple {
    pub fn get_printable(quad: &Quadruple) -> String {
        match quad {
            Quadruple::Op(op, (_, addr1), (_, addr2), (_, addr3)) => {
                format!("{} {} {} {}", op, addr1, addr2, addr3)
            }
            Quadruple::Assign(_, (_, addr1), (_, addr2)) => {
                format!("= {} {} -1", addr1, addr2)
            }
            Quadruple::Read(_, (_, addr)) => {
                format!("Read {} -1 - 1", addr)
            }
            Quadruple::Print(_, (_, addr)) => {
                format!("Print {} -1 -1", addr)
            }
            Quadruple::GoToF((_, addr), pos) => {
                format!("Gotof {} {} -1", addr, pos)
            }
            Quadruple::GoTo(pos) => {
                format!("Goto {} -1 -1", pos)
            }
            Quadruple::GoSub(fn_name, pos) => {
                format!("Gosub {} {} -1", fn_name, pos)
            }
            Quadruple::Era(fn_name) => {
                format!("Era {} -1 -1", fn_name)
            }
            Quadruple::Parameter((_, addr), num) => {
                format!("Parameter {} {} -1", addr, num)
            }
            Quadruple::EndFunc() => {
                format!("Endfunc -1 -1 -1")
            }
            Quadruple::Return() => {
                format!("Return -1 -1 -1")
            }
            Quadruple::Verify((_, addr), lim) => {
                format!("Verify {} {} -1", addr, lim)
            }
            Quadruple::Deref((_, addr), (_, addr2)) => {
                format!("Deref {} {} -1", addr, addr2)
            }
            Quadruple::Temp() => {
                panic!("DEV ERROR: Yoy should not print any temp quadruples");
            }
            Quadruple::Statistics(
                op,
                (_, arr_addr),
                (_, low_addr),
                (_, high_addr),
                lim,
                (_, to_addr),
            ) => {
                format!(
                    "{} {} {},{},{} {}",
                    op, arr_addr, low_addr, high_addr, lim, to_addr
                )
            }
            Quadruple::Plot(op, (_, arrx_addr), (_, arry_addr), (_, sz_addr), szx, szy, fname) => {
                format!(
                    "{} {},{} {},{},{} {}",
                    op, arrx_addr, arry_addr, sz_addr, szx, szy, fname
                )
            }
            Quadruple::Rand(op, (_, addr)) => {
                format!("{} {} -1 -1", op, addr)
            }
        }
    }
}

pub type IdAddr = (String, i32);
