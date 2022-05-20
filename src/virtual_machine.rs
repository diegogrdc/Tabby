/*
Must contain:
- Global info and memory
- Constant table and memory
  - use a memory allocator class and reference objects
  when needed
- Fn Info
- Option to create Local Info and Delete
- Instruction Pointer and Stack
- Quadruples as (String, String, String, String),
- Mega Switch to execute stuff
*/

use crate::cnsts_memory::CnstsMemory;
use crate::memory::Memory;
use std::collections::HashMap;

#[derive(Debug)]
pub struct VirtualMachine {
    pub prog_name: String,
    pub mem_szs: HashMap<String, Memory>,
    pub cnsts: CnstsMemory,
    pub quads: Vec<[String; 4]>,
}

impl VirtualMachine {
    pub fn new() -> VirtualMachine {
        VirtualMachine {
            prog_name: "".to_string(),
            mem_szs: HashMap::new(),
            cnsts: CnstsMemory::empty(),
            quads: Vec::new(),
        }
    }
}
