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
use crate::mem_ptr::*;
use crate::memory::*;
use crate::vir_mem::*;
use std::collections::HashMap;

#[derive(Debug)]
pub struct VirtualMachine {
    pub prog_name: String,
    pub mem_szs: HashMap<String, MemoryInfo>,
    pub cnsts: CnstsMemory,
    pub quads: Vec<[String; 4]>,
    pub glob_mem: Memory,
    pub loc_mem: Memory,
    pub stack_ips: Vec<i32>,
    pub stack_mems: Vec<Memory>,
}

fn as_i32(num: &String) -> i32 {
    num.parse::<i32>().unwrap()
}

impl VirtualMachine {
    pub fn new() -> VirtualMachine {
        VirtualMachine {
            prog_name: "".to_string(),
            mem_szs: HashMap::new(),
            cnsts: CnstsMemory::empty(),
            quads: Vec::new(),
            glob_mem: Memory::empty(),
            loc_mem: Memory::empty(),
            stack_ips: Vec::new(),
            stack_mems: Vec::new(),
        }
    }

    pub fn get_mem_from_fn(&mut self, fn_name: &String) -> Memory {
        let info = self.mem_szs.get(fn_name).unwrap().clone();
        Memory::new(
            info.locs[0] + info.tmps[0],
            info.locs[0],
            info.locs[1] + info.tmps[1],
            info.locs[1],
            info.locs[2] + info.tmps[2],
            info.locs[2],
        )
    }

    pub fn execute(&mut self) {
        // Init Global Memory
        let prog_name = self.prog_name.clone();
        self.glob_mem = self.get_mem_from_fn(&prog_name);
        // Push first IP with Tabby Info
        let tabby_mem = self.get_mem_from_fn(&"Tabby".to_string());
        self.loc_mem = tabby_mem;
        self.stack_ips.push(0);
        self.stack_mems.push(Memory::empty());
        let quads_sz: i32 = self.quads.len() as i32;
        while self.stack_ips.last().unwrap() < &quads_sz {
            self.eval_quad(*self.stack_ips.last().unwrap());
        }
    }

    pub fn eval_quad(&mut self, i: i32) {
        let quad = self.quads.get(i as usize).unwrap().clone();
        match quad[0].as_str() {
            "Goto" => {
                self.quad_goto(&quad);
            }
            "Read" => {
                self.quad_read(&quad);
            }
            "=" => {
                self.quad_assign(&quad);
            }
            "Print" => {
                self.quad_print(&quad);
            }
            "Println" => {
                println!("");
                self.move_ip(1);
            }
            "*" => {
                self.quad_arithmetic(&quad);
            }
            "/" => {
                self.quad_arithmetic(&quad);
            }
            "+" => {
                self.quad_arithmetic(&quad);
            }
            "-" => {
                self.quad_arithmetic(&quad);
            }
            ">" => {
                self.quad_comp(&quad);
            }
            "<" => {
                self.quad_comp(&quad);
            }
            ">=" => {
                self.quad_comp(&quad);
            }
            "<=" => {
                self.quad_comp(&quad);
            }
            "==" => {
                self.quad_comp(&quad);
            }
            "!=" => {
                self.quad_comp(&quad);
            }
            "Or" => {
                self.quad_bool_op(&quad);
            }
            "And" => {
                self.quad_bool_op(&quad);
            }
            "Gotof" => {
                self.quad_gotof(&quad);
            }
            "Endfunc" => {
                self.move_ip(1);
            }
            // Gotof
            // Gosub, Era, Param, Verify, Deref, EndFunc, Return, Verify
            err_sym => {
                eprintln!("\nDEV ERROR: Unrecognized Operation {}", err_sym);
                panic!();
            }
        }
    }

    pub fn quad_goto(&mut self, quad: &[String; 4]) {
        let new_ip = as_i32(&quad[1]);
        let curr_ip = self.stack_ips.last_mut().unwrap();
        *curr_ip = new_ip;
    }

    pub fn quad_gotof(&mut self, quad: &[String; 4]) {
        let addr_val = as_i32(&quad[1]);
        let bool_val = self.get_mem_val(addr_val);
        let new_ip = as_i32(&quad[2]);
        if let MemVal::Bool(val) = bool_val {
            if val == true {
                self.move_ip(1);
            } else {
                let curr_ip = self.stack_ips.last_mut().unwrap();
                *curr_ip = new_ip;
            }
        } else {
            eprintln!(
                "\nDEV ERROR: Gotof should always recieve a boolean. It recieved a {:?}",
                bool_val
            );
            panic!();
        }
    }

    pub fn quad_read(&mut self, quad: &[String; 4]) {
        let addr_to = as_i32(&quad[1]);
        let ptr_to = self.get_mem_ptr(addr_to);
        // Read line
        let mut buffer = String::new();
        std::io::stdin()
            .read_line(&mut buffer)
            .expect("\nEXECUTION ERROR: Error reading variable");
        match ptr_to {
            MemPtr::Int(t) => {
                *t = buffer.trim().parse::<i32>().unwrap();
            }
            MemPtr::Float(t) => {
                *t = buffer.trim().parse::<f64>().unwrap();
            }
            MemPtr::Bool(t) => {
                *t = buffer.trim().to_lowercase().parse::<bool>().unwrap();
            }
        };
        self.move_ip(1);
    }

    pub fn quad_assign(&mut self, quad: &[String; 4]) {
        let addr_from = as_i32(&quad[1]);
        let addr_to = as_i32(&quad[2]);
        let val_from = self.get_mem_val(addr_from);
        let ptr_to = self.get_mem_ptr(addr_to);
        match (val_from, ptr_to) {
            (MemVal::Int(f), MemPtr::Int(t)) => {
                *t = f;
            }
            (MemVal::Float(f), MemPtr::Int(t)) => {
                *t = f as i32;
            }
            (MemVal::Bool(f), MemPtr::Int(t)) => {
                *t = f as i32;
            }
            (MemVal::Int(f), MemPtr::Float(t)) => {
                *t = f as f64;
            }
            (MemVal::Float(f), MemPtr::Float(t)) => {
                *t = f;
            }
            (MemVal::Int(f), MemPtr::Bool(t)) => {
                *t = f != 0;
            }
            (MemVal::Bool(f), MemPtr::Bool(t)) => {
                *t = f;
            }
            (f, t) => {
                eprintln!(
                    "\nDEV ERROR: This assignment should not exist in compilaton: {:?} = {:?}\n",
                    t, f
                );
                panic!();
            }
        };
        self.move_ip(1);
    }

    pub fn quad_print(&mut self, quad: &[String; 4]) {
        let addr_from = as_i32(&quad[1]);
        // Special case: Strlits
        if addr_from >= CNST_START_STRLIT {
            let val_from = self.get_strlit(addr_from);
            self.move_ip(1);
            print!("{} ", val_from);
            return;
        }
        let val_from = self.get_mem_val(addr_from);
        match val_from {
            MemVal::Int(t) => {
                print!("{} ", t);
            }
            MemVal::Float(t) => {
                print!("{} ", t);
            }
            MemVal::Bool(t) => {
                print!("{} ", if t { "True" } else { "False" });
            }
        };
        self.move_ip(1);
    }

    pub fn quad_arithmetic(&mut self, quad: &[String; 4]) {
        let op = &quad[0];
        let addr_op_1 = as_i32(&quad[1]);
        let addr_op_2 = as_i32(&quad[2]);
        let addr_to = as_i32(&quad[3]);
        let val_op_1 = self.get_mem_val(addr_op_1);
        let val_op_2 = self.get_mem_val(addr_op_2);
        let ptr_to = self.get_mem_ptr(addr_to);
        match (val_op_1, val_op_2, ptr_to) {
            (MemVal::Int(v1), MemVal::Int(v2), MemPtr::Int(to)) => {
                if op == "+" {
                    *to = v1 + v2;
                } else if op == "-" {
                    *to = v1 - v2;
                } else if op == "*" {
                    *to = v1 * v2;
                } else {
                    // div
                    *to = v1 / v2;
                }
            }
            (MemVal::Float(v1), MemVal::Float(v2), MemPtr::Float(to)) => {
                if op == "+" {
                    *to = v1 + v2;
                } else if op == "-" {
                    *to = v1 - v2;
                } else if op == "*" {
                    *to = v1 * v2;
                } else {
                    // div
                    *to = v1 / v2;
                }
            }
            (MemVal::Bool(v1), MemVal::Bool(v2), MemPtr::Int(to)) => {
                if op == "+" {
                    *to = v1 as i32 + v2 as i32;
                } else if op == "-" {
                    *to = v1 as i32 - v2 as i32;
                } else if op == "*" {
                    *to = v1 as i32 * v2 as i32;
                } else {
                    // div
                    *to = v1 as i32 / v2 as i32;
                }
            }
            (MemVal::Int(v1), MemVal::Float(v2), MemPtr::Float(to)) => {
                if op == "+" {
                    *to = v1 as f64 + v2;
                } else if op == "-" {
                    *to = v1 as f64 - v2;
                } else if op == "*" {
                    *to = v1 as f64 * v2;
                } else {
                    // div
                    *to = v1 as f64 / v2;
                }
            }
            (MemVal::Float(v1), MemVal::Int(v2), MemPtr::Float(to)) => {
                if op == "+" {
                    *to = v1 + v2 as f64;
                } else if op == "-" {
                    *to = v1 - v2 as f64;
                } else if op == "*" {
                    *to = v1 * v2 as f64;
                } else {
                    // div
                    *to = v1 / v2 as f64;
                }
            }
            (MemVal::Int(v1), MemVal::Bool(v2), MemPtr::Int(to)) => {
                if op == "+" {
                    *to = v1 + v2 as i32;
                } else if op == "-" {
                    *to = v1 - v2 as i32;
                } else if op == "*" {
                    *to = v1 * v2 as i32;
                } else {
                    // div
                    *to = v1 / v2 as i32;
                }
            }
            (MemVal::Bool(v1), MemVal::Int(v2), MemPtr::Int(to)) => {
                if op == "+" {
                    *to = v1 as i32 + v2;
                } else if op == "-" {
                    *to = v1 as i32 - v2;
                } else if op == "*" {
                    *to = v1 as i32 * v2;
                } else {
                    // div
                    *to = v1 as i32 / v2;
                }
            }
            (MemVal::Bool(v1), MemVal::Float(v2), MemPtr::Float(to)) => {
                if op == "+" {
                    *to = v1 as i32 as f64 + v2;
                } else if op == "-" {
                    *to = v1 as i32 as f64 - v2;
                } else if op == "*" {
                    *to = v1 as i32 as f64 * v2;
                } else {
                    // div
                    *to = v1 as i32 as f64 / v2;
                }
            }
            (MemVal::Float(v1), MemVal::Bool(v2), MemPtr::Float(to)) => {
                if op == "+" {
                    *to = v1 + v2 as i32 as f64;
                } else if op == "-" {
                    *to = v1 - v2 as i32 as f64;
                } else if op == "*" {
                    *to = v1 * v2 as i32 as f64;
                } else {
                    // div
                    *to = v1 / v2 as i32 as f64;
                }
            }
            (v1, v2, to) => {
                eprintln!(
                    "\nDEV ERROR: This arithmetic op should not exist in compilaton: {:?} {:?} {:?}\n",
                    v1, v2, to
                );
                panic!();
            }
        };
        self.move_ip(1);
    }

    pub fn quad_comp(&mut self, quad: &[String; 4]) {
        let comp = &quad[0];
        let addr_op_1 = as_i32(&quad[1]);
        let addr_op_2 = as_i32(&quad[2]);
        let addr_to = as_i32(&quad[3]);
        let val_op_1 = self.get_mem_val(addr_op_1);
        let val_op_2 = self.get_mem_val(addr_op_2);
        let ptr_to = self.get_mem_ptr(addr_to);
        match (val_op_1, val_op_2, ptr_to) {
            (MemVal::Int(v1), MemVal::Int(v2), MemPtr::Bool(to)) => {
                if comp == ">" {
                    *to = v1 > v2;
                } else if comp == ">=" {
                    *to = v1 >= v2;
                } else if comp == "<" {
                    *to = v1 < v2;
                } else if comp == "<=" {
                    *to = v1 <= v2;
                } else if comp == "==" {
                    *to = v1 == v2;
                } else {
                    // !=
                    *to = v1 != v2;
                }
            }
            (MemVal::Float(v1), MemVal::Float(v2), MemPtr::Bool(to)) => {
                if comp == ">" {
                    *to = v1 > v2;
                } else if comp == ">=" {
                    *to = v1 >= v2;
                } else if comp == "<" {
                    *to = v1 < v2;
                } else if comp == "<=" {
                    *to = v1 <= v2;
                } else if comp == "==" {
                    *to = v1 == v2;
                } else {
                    // !=
                    *to = v1 != v2;
                }
            }
            (MemVal::Bool(v1), MemVal::Bool(v2), MemPtr::Bool(to)) => {
                if comp == ">" {
                    *to = v1 > v2;
                } else if comp == ">=" {
                    *to = v1 >= v2;
                } else if comp == "<" {
                    *to = v1 < v2;
                } else if comp == "<=" {
                    *to = v1 <= v2;
                } else if comp == "==" {
                    *to = v1 == v2;
                } else {
                    // !=
                    *to = v1 != v2;
                }
            }
            (MemVal::Int(v1), MemVal::Float(v2), MemPtr::Bool(to)) => {
                if comp == ">" {
                    *to = v1 as f64 > v2;
                } else if comp == ">=" {
                    *to = v1 as f64 >= v2;
                } else if comp == "<" {
                    *to = (v1 as f64) < v2;
                } else if comp == "<=" {
                    *to = v1 as f64 <= v2;
                } else if comp == "==" {
                    *to = v1 as f64 == v2;
                } else {
                    // !=
                    *to = v1 as f64 != v2;
                }
            }
            (MemVal::Float(v1), MemVal::Int(v2), MemPtr::Bool(to)) => {
                if comp == ">" {
                    *to = v1 > v2 as f64;
                } else if comp == ">=" {
                    *to = v1 >= v2 as f64;
                } else if comp == "<" {
                    *to = v1 < v2 as f64;
                } else if comp == "<=" {
                    *to = v1 <= v2 as f64;
                } else if comp == "==" {
                    *to = v1 == v2 as f64;
                } else {
                    // !=
                    *to = v1 != v2 as f64;
                }
            }
            (MemVal::Bool(v1), MemVal::Int(v2), MemPtr::Bool(to)) => {
                if comp == ">" {
                    *to = v1 as i32 > v2;
                } else if comp == ">=" {
                    *to = v1 as i32 >= v2;
                } else if comp == "<" {
                    *to = (v1 as i32) < v2;
                } else if comp == "<=" {
                    *to = v1 as i32 <= v2;
                } else if comp == "==" {
                    *to = v1 as i32 == v2;
                } else {
                    // !=
                    *to = v1 as i32 != v2;
                }
            }
            (MemVal::Int(v1), MemVal::Bool(v2), MemPtr::Bool(to)) => {
                if comp == ">" {
                    *to = v1 > v2 as i32;
                } else if comp == ">=" {
                    *to = v1 >= v2 as i32;
                } else if comp == "<" {
                    *to = v1 < v2 as i32;
                } else if comp == "<=" {
                    *to = v1 <= v2 as i32;
                } else if comp == "==" {
                    *to = v1 == v2 as i32;
                } else {
                    // !=
                    *to = v1 != v2 as i32;
                }
            }
            (v1, v2, to) => {
                eprintln!(
                    "\nDEV ERROR: This comp operation should not exist in compilaton: {:?} {:?} {:?}\n",
                    v1, v2, to
                );
                panic!();
            }
        };
        self.move_ip(1);
    }

    pub fn quad_bool_op(&mut self, quad: &[String; 4]) {
        let op = &quad[0];
        let addr_op_1 = as_i32(&quad[1]);
        let addr_op_2 = as_i32(&quad[2]);
        let addr_to = as_i32(&quad[3]);
        let val_op_1 = self.get_mem_val(addr_op_1);
        let val_op_2 = self.get_mem_val(addr_op_2);
        let ptr_to = self.get_mem_ptr(addr_to);
        match (val_op_1, val_op_2, ptr_to) {
            (MemVal::Int(v1), MemVal::Int(v2), MemPtr::Bool(to)) => {
                if op == "And" {
                    *to = v1 != 0 && v2 != 0;
                } else {
                    // Or
                    *to = v1 != 0 || v2 != 0;
                }
            }
            (MemVal::Bool(v1), MemVal::Bool(v2), MemPtr::Bool(to)) => {
                if op == "And" {
                    *to = v1 && v2;
                } else {
                    // Or
                    *to = v1 || v2;
                }
            }
            (MemVal::Int(v1), MemVal::Bool(v2), MemPtr::Bool(to)) => {
                if op == "And" {
                    *to = v1 != 0 && v2;
                } else {
                    // Or
                    *to = v1 != 0 || v2;
                }
            }
            (MemVal::Bool(v1), MemVal::Int(v2), MemPtr::Bool(to)) => {
                if op == "And" {
                    *to = v1 && v2 != 0;
                } else {
                    // Or
                    *to = v1 || v2 != 0;
                }
            }
            (v1, v2, to) => {
                eprintln!(
                    "\nDEV ERROR: This boolean operation should not exist in compilaton: {:?} {:?} {:?}\n",
                    v1, v2, to
                );
                panic!();
            }
        }
        self.move_ip(1);
    }

    pub fn move_ip(&mut self, qnt: i32) {
        let ip = self.stack_ips.last_mut().unwrap();
        *ip = *ip + qnt;
    }

    pub fn get_mem_ptr(&mut self, addr: i32) -> MemPtr {
        // Global
        if addr <= GLOBAL_END {
            // Int
            if addr < GLOBAL_START_FLOAT {
                let mem_ptr: &mut i32 = self
                    .glob_mem
                    .int_mem
                    .get_mut((addr - GLOBAL_START_INT) as usize)
                    .unwrap();
                return MemPtr::Int(mem_ptr);
            }
            // Float
            else if addr < GLOBAL_START_BOOL {
                let mem_ptr: &mut f64 = self
                    .glob_mem
                    .float_mem
                    .get_mut((addr - GLOBAL_START_FLOAT) as usize)
                    .unwrap();
                return MemPtr::Float(mem_ptr);
            }
            // Bool
            else {
                let mem_ptr: &mut bool = self
                    .glob_mem
                    .bool_mem
                    .get_mut((addr - GLOBAL_START_BOOL) as usize)
                    .unwrap();
                return MemPtr::Bool(mem_ptr);
            }
        }
        // Gtemp
        else if addr <= GTEMP_END {
            // Int
            if addr < GTEMP_START_FLOAT {
                let mem_ptr: &mut i32 = self
                    .glob_mem
                    .int_mem
                    .get_mut((addr - GTEMP_START_INT + self.glob_mem.int_off) as usize)
                    .unwrap();
                return MemPtr::Int(mem_ptr);
            }
            // Float
            else if addr < GTEMP_START_BOOL {
                let mem_ptr: &mut f64 = self
                    .glob_mem
                    .float_mem
                    .get_mut((addr - GTEMP_START_FLOAT + self.glob_mem.float_off) as usize)
                    .unwrap();
                return MemPtr::Float(mem_ptr);
            }
            // Bool
            else {
                let mem_ptr: &mut bool = self
                    .glob_mem
                    .bool_mem
                    .get_mut((addr - GTEMP_START_BOOL + self.glob_mem.bool_off) as usize)
                    .unwrap();
                return MemPtr::Bool(mem_ptr);
            }
        }
        // Local
        else if addr <= LOCAL_END {
            // Int
            if addr < LOCAL_START_FLOAT {
                let mem_ptr: &mut i32 = self
                    .loc_mem
                    .int_mem
                    .get_mut((addr - LOCAL_START_INT) as usize)
                    .unwrap();
                return MemPtr::Int(mem_ptr);
            }
            // Float
            else if addr < LOCAL_START_BOOL {
                let mem_ptr: &mut f64 = self
                    .loc_mem
                    .float_mem
                    .get_mut((addr - LOCAL_START_FLOAT) as usize)
                    .unwrap();
                return MemPtr::Float(mem_ptr);
            }
            // Bool
            else {
                let mem_ptr: &mut bool = self
                    .loc_mem
                    .bool_mem
                    .get_mut((addr - LOCAL_START_BOOL) as usize)
                    .unwrap();
                return MemPtr::Bool(mem_ptr);
            }
        }
        // Ltemp
        else if addr <= LTEMP_END {
            // Int
            if addr < LTEMP_START_FLOAT {
                let mem_ptr: &mut i32 = self
                    .loc_mem
                    .int_mem
                    .get_mut((addr - LTEMP_START_INT + self.loc_mem.int_off) as usize)
                    .unwrap();
                return MemPtr::Int(mem_ptr);
            }
            // Float
            else if addr < LTEMP_START_BOOL {
                let mem_ptr: &mut f64 = self
                    .loc_mem
                    .float_mem
                    .get_mut((addr - LTEMP_START_FLOAT + self.loc_mem.float_off) as usize)
                    .unwrap();
                return MemPtr::Float(mem_ptr);
            }
            // Bool
            else {
                let mem_ptr: &mut bool = self
                    .loc_mem
                    .bool_mem
                    .get_mut((addr - LTEMP_START_BOOL + self.loc_mem.bool_off) as usize)
                    .unwrap();
                return MemPtr::Bool(mem_ptr);
            }
        }
        // Cnsts should never be pointers
        eprintln!("DEV ERROR: Memory should always be found and correct");
        panic!();
    }

    pub fn get_mem_val(&mut self, addr: i32) -> MemVal {
        // Global
        if addr <= GLOBAL_END {
            // Int
            if addr < GLOBAL_START_FLOAT {
                let mem_val: i32 = self
                    .glob_mem
                    .int_mem
                    .get((addr - GLOBAL_START_INT) as usize)
                    .unwrap()
                    .clone();
                return MemVal::Int(mem_val);
            }
            // Float
            else if addr < GLOBAL_START_BOOL {
                let mem_val: f64 = self
                    .glob_mem
                    .float_mem
                    .get((addr - GLOBAL_START_FLOAT) as usize)
                    .unwrap()
                    .clone();
                return MemVal::Float(mem_val);
            }
            // Bool
            else {
                let mem_val: bool = self
                    .glob_mem
                    .bool_mem
                    .get((addr - GLOBAL_START_BOOL) as usize)
                    .unwrap()
                    .clone();
                return MemVal::Bool(mem_val);
            }
        }
        // Gtemp
        else if addr <= GTEMP_END {
            // Int
            if addr < GTEMP_START_FLOAT {
                let mem_val: i32 = self
                    .glob_mem
                    .int_mem
                    .get((addr - GTEMP_START_INT + self.glob_mem.int_off) as usize)
                    .unwrap()
                    .clone();
                return MemVal::Int(mem_val);
            }
            // Float
            else if addr < GTEMP_START_BOOL {
                let mem_val: f64 = self
                    .glob_mem
                    .float_mem
                    .get((addr - GTEMP_START_FLOAT + self.glob_mem.float_off) as usize)
                    .unwrap()
                    .clone();
                return MemVal::Float(mem_val);
            }
            // Bool
            else {
                let mem_val: bool = self
                    .glob_mem
                    .bool_mem
                    .get((addr - GTEMP_START_BOOL + self.glob_mem.bool_off) as usize)
                    .unwrap()
                    .clone();
                return MemVal::Bool(mem_val);
            }
        }
        // Local
        else if addr <= LOCAL_END {
            // Int
            if addr < LOCAL_START_FLOAT {
                let mem_val: i32 = self
                    .loc_mem
                    .int_mem
                    .get((addr - LOCAL_START_INT) as usize)
                    .unwrap()
                    .clone();
                return MemVal::Int(mem_val);
            }
            // Float
            else if addr < LOCAL_START_BOOL {
                let mem_val: f64 = self
                    .loc_mem
                    .float_mem
                    .get((addr - LOCAL_START_FLOAT) as usize)
                    .unwrap()
                    .clone();
                return MemVal::Float(mem_val);
            }
            // Bool
            else {
                let mem_val: bool = self
                    .loc_mem
                    .bool_mem
                    .get((addr - LOCAL_START_BOOL) as usize)
                    .unwrap()
                    .clone();
                return MemVal::Bool(mem_val);
            }
        }
        // Ltemp
        else if addr <= LTEMP_END {
            // Int
            if addr < LTEMP_START_FLOAT {
                let mem_val: i32 = self
                    .loc_mem
                    .int_mem
                    .get((addr - LTEMP_START_INT + self.loc_mem.int_off) as usize)
                    .unwrap()
                    .clone();
                return MemVal::Int(mem_val);
            }
            // Float
            else if addr < LTEMP_START_BOOL {
                let mem_val: f64 = self
                    .loc_mem
                    .float_mem
                    .get((addr - LTEMP_START_FLOAT + self.loc_mem.float_off) as usize)
                    .unwrap()
                    .clone();
                return MemVal::Float(mem_val);
            }
            // Bool
            else {
                let mem_val: bool = self
                    .loc_mem
                    .bool_mem
                    .get((addr - LTEMP_START_BOOL + self.loc_mem.bool_off) as usize)
                    .unwrap()
                    .clone();
                return MemVal::Bool(mem_val);
            }
        }
        // Cnsts
        else if addr <= CNST_END {
            // Int
            if addr < CNST_START_FLOAT {
                let mem_val: i32 = self
                    .cnsts
                    .int_cnst
                    .get((addr - CNST_START_INT) as usize)
                    .unwrap()
                    .clone();
                return MemVal::Int(mem_val);
            }
            // Float
            else if addr < CNST_START_BOOL {
                let mem_val: f64 = self
                    .cnsts
                    .float_cnst
                    .get((addr - CNST_START_FLOAT) as usize)
                    .unwrap()
                    .clone();
                return MemVal::Float(mem_val);
            }
            // Bool
            else {
                let mem_val: bool = self
                    .cnsts
                    .bool_cnst
                    .get((addr - CNST_START_BOOL) as usize)
                    .unwrap()
                    .clone();
                return MemVal::Bool(mem_val);
            }
        }
        eprintln!("DEV ERROR: Memory should always be found and correct");
        panic!();
    }

    pub fn get_strlit(&mut self, addr: i32) -> String {
        self.cnsts
            .strlit_cnst
            .get((addr - CNST_START_STRLIT) as usize)
            .unwrap()
            .clone()
    }
}
