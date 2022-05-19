// Imports
use crate::tipo::Tipo;
use std::collections::HashMap;

// constants
pub const LOCAL_BLOCK_SIZE: i32 = 10002; // Divisible by 3
pub const GLOBAL_BLOCK_SIZE: i32 = 100002; // Divisible by 3

pub const GLOBAL_START: i32 = 0;
pub const GLOBAL_END: i32 = GLOBAL_START + GLOBAL_BLOCK_SIZE - 1;
pub const GTEMP_START: i32 = GLOBAL_END + 1;
pub const GTEMP_END: i32 = GTEMP_START + 3 - 1;
pub const LOCAL_START: i32 = GTEMP_END + 1;
pub const LOCAL_END: i32 = LOCAL_START + LOCAL_BLOCK_SIZE - 1;
pub const LTEMP_START: i32 = LOCAL_END + 1;
pub const LTEMP_END: i32 = LTEMP_START + LOCAL_BLOCK_SIZE - 1;
pub const CNST_START: i32 = LTEMP_END + 1;
pub const CNST_END: i32 = CNST_START + LOCAL_BLOCK_SIZE / 3 * 4 - 1;

pub const GLOBAL_INT_OFFSET: i32 = 0;
pub const GLOBAL_FLOAT_OFFSET: i32 = GLOBAL_BLOCK_SIZE / 3;
pub const GLOBAL_BOOL_OFFSET: i32 = GLOBAL_BLOCK_SIZE / 3 * 2;

pub const LOCAL_INT_OFFSET: i32 = 0;
pub const LOCAL_FLOAT_OFFSET: i32 = LOCAL_BLOCK_SIZE / 3;
pub const LOCAL_BOOL_OFFSET: i32 = LOCAL_BLOCK_SIZE / 3 * 2;
pub const STRLIT_OFFSET: i32 = LOCAL_BLOCK_SIZE / 3 * 3;

// Struct to take care of virtual memory allocation
pub struct VirMemAllocator {
    pub global: [i32; 3],
    pub gtemp: [i32; 3],
    pub local: [i32; 3],
    pub ltemp: [i32; 3],
    pub ltemp_map: HashMap<String, i32>,
    pub cnst: [i32; 4],
    pub cnst_map: HashMap<String, i32>,
    pub cnst_vals: [Vec<String>; 4],
}

impl VirMemAllocator {
    pub fn new() -> VirMemAllocator {
        VirMemAllocator {
            global: [
                GLOBAL_START + GLOBAL_INT_OFFSET,
                GLOBAL_START + GLOBAL_FLOAT_OFFSET,
                GLOBAL_START + GLOBAL_BOOL_OFFSET,
            ],
            gtemp: [GTEMP_START + 0, GTEMP_START + 1, GTEMP_START + 2],
            local: [
                LOCAL_START + LOCAL_INT_OFFSET,
                LOCAL_START + LOCAL_FLOAT_OFFSET,
                LOCAL_START + LOCAL_BOOL_OFFSET,
            ],
            ltemp: [
                LTEMP_START + LOCAL_INT_OFFSET,
                LTEMP_START + LOCAL_FLOAT_OFFSET,
                LTEMP_START + LOCAL_BOOL_OFFSET,
            ],
            ltemp_map: HashMap::new(),
            cnst: [
                CNST_START + LOCAL_INT_OFFSET,
                CNST_START + LOCAL_FLOAT_OFFSET,
                CNST_START + LOCAL_BOOL_OFFSET,
                CNST_START + STRLIT_OFFSET,
            ],
            cnst_map: HashMap::new(),
            cnst_vals: [Vec::new(), Vec::new(), Vec::new(), Vec::new()],
        }
    }

    pub fn reset_local(&mut self) {
        self.local = [
            LOCAL_START + LOCAL_INT_OFFSET,
            LOCAL_START + LOCAL_FLOAT_OFFSET,
            LOCAL_START + LOCAL_BOOL_OFFSET,
        ];
        self.ltemp = [
            LTEMP_START + LOCAL_INT_OFFSET,
            LTEMP_START + LOCAL_FLOAT_OFFSET,
            LTEMP_START + LOCAL_BOOL_OFFSET,
        ];
        self.ltemp_map = HashMap::new();
    }

    pub fn get_global_addr(&mut self, tipo: &Tipo, qnt: i32) -> i32 {
        let pos = self.get_pos(&tipo);

        let lim: i32 = match *tipo {
            Tipo::Int => GLOBAL_START + GLOBAL_FLOAT_OFFSET - 1,
            Tipo::Float => GLOBAL_START + GLOBAL_BOOL_OFFSET - 1,
            Tipo::Bool => GLOBAL_END,
            _ => 0,
        };

        if self.global[pos] + qnt > lim {
            eprintln!(
                "\nCOMPILATION ERROR: Too many global variables of type {:?} declared\n",
                tipo
            );
            std::process::exit(1);
        }

        let mem = self.global[pos];
        self.global[pos] = self.global[pos] + qnt;
        mem
    }

    pub fn get_gtemp_addr(&mut self, tipo: &Tipo) -> i32 {
        let pos = self.get_pos(&tipo);
        self.gtemp[pos]
    }

    pub fn get_local_addr(&mut self, tipo: &Tipo) -> i32 {
        let pos = self.get_pos(&tipo);

        let lim: i32 = match *tipo {
            Tipo::Int => LOCAL_START + LOCAL_FLOAT_OFFSET - 1,
            Tipo::Float => LOCAL_START + LOCAL_BOOL_OFFSET - 1,
            Tipo::Bool => LOCAL_END,
            _ => 0,
        };

        if self.local[pos] + 1 > lim {
            eprintln!(
                "\n COMPILATION ERROR: Too many local variables of type {:?} declared\n",
                tipo
            );
            std::process::exit(1);
        }

        let mem = self.local[pos];
        self.local[pos] = self.local[pos] + 1;
        mem
    }

    pub fn get_ltemp_addr(&mut self, id: &String, tipo: &Tipo) -> i32 {
        if let Some(addr) = self.ltemp_map.get(id) {
            return *addr;
        }

        let pos = self.get_pos(&tipo);

        let lim: i32 = match *tipo {
            Tipo::Int => LTEMP_START + LOCAL_FLOAT_OFFSET - 1,
            Tipo::Float => LTEMP_START + LOCAL_BOOL_OFFSET - 1,
            Tipo::Bool => LTEMP_END,
            _ => 0,
        };

        if self.ltemp[pos] + 1 > lim {
            eprintln!(
                "\nCOMPILATION ERROR: Too many temporal variables of type {:?} generated\n",
                tipo
            );
            std::process::exit(1);
        }

        let mem = self.ltemp[pos];
        self.ltemp_map.insert(id.clone(), mem);
        self.ltemp[pos] = self.ltemp[pos] + 1;
        mem
    }

    pub fn get_cnst_addr(&mut self, cnst: &String, tipo: &Tipo) -> i32 {
        if let Some(addr) = self.cnst_map.get(cnst) {
            return *addr;
        }
        let pos = self.get_pos(&tipo);
        if pos == 3 {
            let mut chars = cnst.chars();
            chars.next();
            self.cnst_vals[pos].push(chars.as_str().to_string());
        } else {
            self.cnst_vals[pos].push(cnst.clone());
        }

        let lim: i32 = match *tipo {
            Tipo::Int => CNST_START + LOCAL_FLOAT_OFFSET - 1,
            Tipo::Float => CNST_START + LOCAL_BOOL_OFFSET - 1,
            Tipo::Bool => CNST_START + STRLIT_OFFSET - 1,
            Tipo::StrLit => CNST_END - 1,
            _ => 0,
        };

        if self.cnst[pos] + 1 > lim {
            eprintln!(
                "\nCOMPILATION ERROR: Too many constant variables of type {:?} declared\n",
                tipo
            );
            std::process::exit(1);
        }

        let mem = self.cnst[pos];
        self.cnst_map.insert(cnst.clone(), mem);
        self.cnst[pos] = self.cnst[pos] + 1;
        mem
    }

    pub fn get_pos(&self, tipo: &Tipo) -> usize {
        match *tipo {
            Tipo::Int => 0,
            Tipo::Float => 1,
            Tipo::Bool => 2,
            Tipo::StrLit => 3,
            _ => {
                panic!("DEV ERROR: Cannot assign virtual memory to type {:?}", tipo);
            }
        }
    }
}
