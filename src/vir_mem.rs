// Imports
use crate::tipo::Tipo;
use std::collections::HashMap;

// constants
pub const BLOCK_SIZE: i32 = 30000; // Divisible by 3
pub const GLOBAL_START: i32 = 10000;
pub const GLOBAL_END: i32 = GLOBAL_START + BLOCK_SIZE - 1;
pub const LOCAL_START: i32 = GLOBAL_END + 1;
pub const LOCAL_END: i32 = LOCAL_START + BLOCK_SIZE - 1;
pub const TEMP_START: i32 = LOCAL_END + 1;
pub const TEMP_END: i32 = TEMP_START + BLOCK_SIZE - 1;
pub const CNST_START: i32 = TEMP_END + 1;
pub const CNST_END: i32 = CNST_START + BLOCK_SIZE / 3 * 4 - 1;

pub const INT_OFFSET: i32 = 0;
pub const FLOAT_OFFSET: i32 = BLOCK_SIZE / 3;
pub const BOOL_OFFSET: i32 = BLOCK_SIZE / 3 * 2;
pub const STRLIT_OFFSET: i32 = BLOCK_SIZE / 3 * 3;

// Struct to take care of virtual memory allocation
pub struct VirMemAllocator {
    pub global: [i32; 3],
    pub local: [i32; 3],
    pub temp: [i32; 3],
    pub temp_map: HashMap<String, i32>,
    pub cnst: [i32; 4],
    pub cnst_map: HashMap<String, i32>,
}

impl VirMemAllocator {
    pub fn new() -> VirMemAllocator {
        VirMemAllocator {
            global: [
                GLOBAL_START + INT_OFFSET,
                GLOBAL_START + FLOAT_OFFSET,
                GLOBAL_START + BOOL_OFFSET,
            ],
            local: [
                LOCAL_START + INT_OFFSET,
                LOCAL_START + FLOAT_OFFSET,
                LOCAL_START + BOOL_OFFSET,
            ],
            temp: [
                TEMP_START + INT_OFFSET,
                TEMP_START + FLOAT_OFFSET,
                TEMP_START + BOOL_OFFSET,
            ],
            temp_map: HashMap::new(),
            cnst: [
                CNST_START + INT_OFFSET,
                CNST_START + FLOAT_OFFSET,
                CNST_START + BOOL_OFFSET,
                CNST_START + STRLIT_OFFSET,
            ],
            cnst_map: HashMap::new(),
        }
    }

    pub fn reset_local(&mut self) {
        self.local = [
            LOCAL_START + INT_OFFSET,
            LOCAL_START + FLOAT_OFFSET,
            LOCAL_START + BOOL_OFFSET,
        ];
    }

    pub fn get_global_addr(&mut self, tipo: &Tipo) -> i32 {
        let pos = self.get_pos(&tipo);

        let lim: i32 = match *tipo {
            Tipo::Int => GLOBAL_START + FLOAT_OFFSET - 1,
            Tipo::Float => GLOBAL_START + BOOL_OFFSET - 1,
            Tipo::Bool => GLOBAL_END,
            _ => 0,
        };

        if self.global[pos] + 1 > lim {
            panic!(
                "ERROR: Too many global variables of type {:?} declared\n",
                tipo
            );
        }

        let mem = self.global[pos];
        self.global[pos] = self.global[pos] + 1;
        mem
    }

    pub fn get_local_addr(&mut self, tipo: &Tipo) -> i32 {
        let pos = self.get_pos(&tipo);

        let lim: i32 = match *tipo {
            Tipo::Int => LOCAL_START + FLOAT_OFFSET - 1,
            Tipo::Float => LOCAL_START + BOOL_OFFSET - 1,
            Tipo::Bool => LOCAL_END,
            _ => 0,
        };

        if self.local[pos] + 1 > lim {
            panic!(
                "ERROR: Too many local variables of type {:?} declared\n",
                tipo
            );
        }

        let mem = self.local[pos];
        self.local[pos] = self.local[pos] + 1;
        mem
    }

    pub fn get_temp_addr(&mut self, id: &String, tipo: &Tipo) -> i32 {
        if let Some(addr) = self.temp_map.get(id) {
            return *addr;
        }

        let pos = self.get_pos(&tipo);

        let lim: i32 = match *tipo {
            Tipo::Int => TEMP_START + FLOAT_OFFSET - 1,
            Tipo::Float => TEMP_START + BOOL_OFFSET - 1,
            Tipo::Bool => TEMP_END,
            _ => 0,
        };

        if self.temp[pos] + 1 > lim {
            panic!(
                "ERROR: Too many temporal variables of type {:?} generated\n",
                tipo
            );
        }

        let mem = self.temp[pos];
        self.temp_map.insert(id.clone(), mem);
        self.temp[pos] = self.temp[pos] + 1;
        mem
    }

    pub fn get_cnst_addr(&mut self, num: &String, tipo: &Tipo) -> i32 {
        if let Some(addr) = self.cnst_map.get(num) {
            return *addr;
        }
        let pos = self.get_pos(&tipo);

        let lim: i32 = match *tipo {
            Tipo::Int => CNST_START + FLOAT_OFFSET - 1,
            Tipo::Float => CNST_START + BOOL_OFFSET - 1,
            Tipo::Bool => CNST_START + STRLIT_OFFSET - 1,
            Tipo::StrLit => CNST_END - 1,
            _ => 0,
        };

        if self.cnst[pos] + 1 > lim {
            panic!(
                "ERROR: Too many constant variables of type {:?} declared\n",
                tipo
            );
        }

        let mem = self.cnst[pos];
        self.cnst_map.insert(num.clone(), mem);
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
