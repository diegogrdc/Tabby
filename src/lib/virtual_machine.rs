/*
Virtual Machine

Structure created to take
care of IC code execution
and to catch execution errors
It executes all operations given by
IC code, and prints any info given by
operations to STDOUT

*/

// Plotlib Imports
use plotlib::page::Page;
use plotlib::repr::Plot;
use plotlib::repr::{Histogram, HistogramBins};
use plotlib::style::BoxStyle;
use plotlib::style::{LineJoin, LineStyle};
use plotlib::style::{PointMarker, PointStyle};
use plotlib::view::ContinuousView;

// Lib Imports
use crate::cnsts_memory::CnstsMemory;
use crate::mem_ptr::*;
use crate::memory::*;
use crate::vir_mem::*;
use std::collections::HashMap;
use std::mem;
/*
Parameters used in our Virtual Machine
where we execute Quadruples with information
obtained from IC code
and check for execution errors
Params:
- prog_name
    Variable to store program name
    to find global variables
- mem_szs
    Keeps information on size
    needed to reserve each function
    when it is called
- cnsts
    Keeps all constants read from
    IC file stored and ready for when they
    are used
- quads
    Keeps a list of 4 vaules on each
    element, representing the quadruples
    of execution, that we will traverse
    around on execution
- glob_mem
    Keeps global memory and data
- loc_mem
    Keeps current local memory and data
- loc_ptrs
    Keeps current local pointers
- stack_ips
    Keeps pending ips on a call stack
- stack_mems
    Keeps pending memory structures
    on a call stack
- stack_ptrs
    Keeps pending pointers on a call stack
- stack_params
    Keeps a stack of params
    to initialize memory when
    getting to a new function call
- output
    Stores all output for
    testing purposes
*/
#[derive(Debug)]
pub struct VirtualMachine {
    pub prog_name: String,
    pub mem_szs: HashMap<String, MemoryInfo>,
    pub cnsts: CnstsMemory,
    pub quads: Vec<[String; 4]>,
    pub glob_mem: Memory,
    pub loc_mem: Memory,
    pub loc_ptrs: HashMap<i32, i32>,
    pub stack_ips: Vec<i32>,
    pub stack_mems: Vec<Memory>,
    pub stack_ptrs: Vec<HashMap<i32, i32>>,
    pub stack_params: Vec<(MemVal, i32)>,
    pub output: String,
}

fn as_i32(num: &String) -> i32 {
    num.parse::<i32>().unwrap()
}

fn as_i32_pair(pair: &String) -> (i32, i32) {
    let vec: Vec<i32> = pair.split(",").map(|x| x.parse::<i32>().unwrap()).collect();
    (vec[0], vec[1])
}

fn as_i32_triple(triple: &String) -> (i32, i32, i32) {
    let vec: Vec<i32> = triple
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    (vec[0], vec[1], vec[2])
}

fn check_range_in_lims(real_low: i32, real_high: i32, low: i32, high: i32, name: &str) {
    if high < low {
        eprintln!("\nEXECUTION ERROR: On access to function: {}, low must be smaller to high. Got {}, {}\n", name, low, high);
        panic!();
    }
    if low < real_low || high > real_high {
        eprintln!("\nEXECUTION ERROR: On access to function: {}, low and high must be on array limits. Got {} to {}. Expected in range {} to {}\n", name, low, high, real_low, real_high);
        panic!();
    }
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
            loc_ptrs: HashMap::new(),
            stack_ips: Vec::new(),
            stack_mems: Vec::new(),
            stack_ptrs: Vec::new(),
            stack_params: Vec::new(),
            output: String::new(),
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
            "Deref" => {
                self.quad_deref(&quad);
            }
            "Verify" => {
                self.quad_verify(&quad);
            }
            "Gosub" => {
                self.quad_gosub(&quad);
            }
            "Era" => {
                self.quad_era();
            }
            "Endfunc" => {
                self.quad_endfunc();
            }
            "Return" => {
                self.quad_return();
            }
            "Parameter" => {
                self.quad_parameter(&quad);
            }
            "MinFlt" => {
                self.quad_statistics(&quad, "MinFlt");
            }
            "MinInt" => {
                self.quad_statistics(&quad, "MinInt");
            }
            "MaxFlt" => {
                self.quad_statistics(&quad, "MaxFlt");
            }
            "MaxInt" => {
                self.quad_statistics(&quad, "MaxInt");
            }
            "RangeFlt" => {
                self.quad_statistics(&quad, "RangeFlt");
            }
            "RangeInt" => {
                self.quad_statistics(&quad, "RangeInt");
            }
            "MeanFlt" => {
                self.quad_statistics(&quad, "MeanFlt");
            }
            "MeanInt" => {
                self.quad_statistics(&quad, "MeanInt");
            }
            "ModeInt" => {
                self.quad_statistics(&quad, "ModeInt");
            }
            "MedianFlt" => {
                self.quad_statistics(&quad, "MedianFlt");
            }
            "MedianInt" => {
                self.quad_statistics(&quad, "MedianInt");
            }
            "StdDevFlt" => {
                self.quad_statistics(&quad, "StdDevFlt");
            }
            "StdDevInt" => {
                self.quad_statistics(&quad, "StdDevInt");
            }
            "VarianceFlt" => {
                self.quad_statistics(&quad, "VarianceFlt");
            }
            "VarianceInt" => {
                self.quad_statistics(&quad, "VarianceInt");
            }
            "HistogramPlot" => {
                self.quad_plot(&quad, "HistogramPlot");
            }
            "LinePlot" => {
                self.quad_plot(&quad, "LinePlot");
            }
            "ScatterPlot" => {
                self.quad_plot(&quad, "ScatterPlot");
            }
            "RandInt" => {
                self.quad_rand(&quad);
            }
            "RandFlt" => {
                self.quad_rand(&quad);
            }
            err_sym => {
                eprintln!("\nDEV ERROR: Unrecognized Operation {}", err_sym);
                panic!();
            }
        }
    }

    pub fn quad_rand(&mut self, quad: &[String; 4]) {
        let addr = as_i32(&quad[1]);
        if quad[0] == "RandInt" {
            let mut mem_ptr = self.get_mem_ptr(addr);
            let ptr_int = mem_ptr.as_int();
            *ptr_int = rand::random::<i32>();
        } else {
            let mut mem_ptr = self.get_mem_ptr(addr);
            let ptr_flt = mem_ptr.as_float();
            *ptr_flt = rand::random::<f64>();
        }
        self.move_ip(1);
    }

    pub fn get_float_vec_from_range(&mut self, base_addr: i32, low: i32, high: i32) -> Vec<f64> {
        let mut vec = Vec::new();
        for i in low..=high {
            let val = self.get_mem_val(base_addr + i);
            vec.push(val.as_float());
        }
        vec
    }

    pub fn quad_plot(&mut self, quad: &[String; 4], fn_name: &str) {
        // Do generic stuff for all
        let (base_addrx, base_addry) = as_i32_pair(&quad[1]);
        let (sz_addr, szx, szy) = as_i32_triple(&quad[2]);
        let filename = quad[3].clone();
        let sz = self.get_mem_val(sz_addr).as_int();
        check_range_in_lims(0, szx, 0, sz, fn_name);
        check_range_in_lims(0, szy, 0, sz, fn_name);
        // Create vecs
        let datax = self.get_float_vec_from_range(base_addrx, 0, sz - 1);
        // Choose specific
        match fn_name {
            "HistogramPlot" => {
                self.plot_histogram(datax, base_addry, filename);
            }
            "LinePlot" => {
                let datay = self.get_float_vec_from_range(base_addry, 0, sz - 1);
                self.plot_line(datax, datay, filename);
            }
            "ScatterPlot" => {
                let datay = self.get_float_vec_from_range(base_addry, 0, sz - 1);
                self.plot_scatter(datax, datay, filename);
            }
            _ => {
                panic!("NOPE");
            }
        };
        self.move_ip(1);
    }

    pub fn plot_histogram(&mut self, data: Vec<f64>, bins: i32, filename: String) {
        let h = Histogram::from_slice(&data, HistogramBins::Count(bins as usize))
            .style(&BoxStyle::new().fill("burlywood"));
        let v = ContinuousView::new().add(h);
        Page::single(&v)
            .save(format!("./main/{}.svg", filename))
            .expect("saving Histogram svg...");
        println!("Histogram Plot created correctly on file {}.svg", filename);
    }

    pub fn plot_line(&mut self, datax: Vec<f64>, datay: Vec<f64>, filename: String) {
        let data = datax.into_iter().zip(datay.into_iter()).collect();
        let l1 = Plot::new(data).line_style(
            LineStyle::new()
                .colour("burlywood")
                .linejoin(LineJoin::Round),
        );
        let v = ContinuousView::new().add(l1);
        Page::single(&v)
            .save(format!("./main/{}.svg", filename))
            .expect("saving Line svg...");
        println!("Line Plot created correctly on file {}.svg", filename);
    }

    pub fn plot_scatter(&mut self, datax: Vec<f64>, datay: Vec<f64>, filename: String) {
        let data = datax.into_iter().zip(datay.into_iter()).collect();
        // We create our scatter plot from the data
        let s1: Plot = Plot::new(data).point_style(
            PointStyle::new()
                .marker(PointMarker::Square) // setting the marker to be a square
                .colour("#DD3355"),
        ); // and a custom colour
           // The 'view' describes what set of data is drawn
        let v = ContinuousView::new().add(s1);
        Page::single(&v)
            .save(format!("./main/{}.svg", filename))
            .expect("saving Scatter svg...");
        println!("Scatter Plot created correctly on file {}.svg", filename);
    }

    pub fn quad_statistics(&mut self, quad: &[String; 4], fn_name: &str) {
        // Do generics
        let base_addr = as_i32(&quad[1]);
        let (low_addr, high_addr, lim) = as_i32_triple(&quad[2]);
        let low = self.get_mem_val(low_addr).as_int();
        let high = self.get_mem_val(high_addr).as_int();
        check_range_in_lims(0, lim - 1, low, high, fn_name);
        let to_addr = as_i32(&quad[3]);
        // Do specific stuff
        match fn_name {
            "MinFlt" => {
                let min = self.eval_minflt(base_addr, low, high);
                let mut to_mem_ptr = self.get_mem_ptr(to_addr);
                let flt_ptr = to_mem_ptr.as_float();
                *flt_ptr = min;
            }
            "MinInt" => {
                let min = self.eval_minint(base_addr, low, high);
                let mut to_mem_ptr = self.get_mem_ptr(to_addr);
                let int_ptr = to_mem_ptr.as_int();
                *int_ptr = min;
            }
            "MaxFlt" => {
                let max = self.eval_maxflt(base_addr, low, high);
                let mut to_mem_ptr = self.get_mem_ptr(to_addr);
                let flt_ptr = to_mem_ptr.as_float();
                *flt_ptr = max;
            }
            "MaxInt" => {
                let max = self.eval_maxint(base_addr, low, high);
                let mut to_mem_ptr = self.get_mem_ptr(to_addr);
                let int_ptr = to_mem_ptr.as_int();
                *int_ptr = max;
            }
            "RangeFlt" => {
                let range = self.eval_rangeflt(base_addr, low, high);
                let mut to_mem_ptr = self.get_mem_ptr(to_addr);
                let flt_ptr = to_mem_ptr.as_float();
                *flt_ptr = range;
            }
            "RangeInt" => {
                let range = self.eval_rangeint(base_addr, low, high);
                let mut to_mem_ptr = self.get_mem_ptr(to_addr);
                let int_ptr = to_mem_ptr.as_int();
                *int_ptr = range;
            }
            "MeanFlt" => {
                let mean = self.eval_mean(base_addr, low, high);
                let mut to_mem_ptr = self.get_mem_ptr(to_addr);
                let flt_ptr = to_mem_ptr.as_float();
                *flt_ptr = mean;
            }
            "MeanInt" => {
                let mean = self.eval_mean(base_addr, low, high);
                let mut to_mem_ptr = self.get_mem_ptr(to_addr);
                let flt_ptr = to_mem_ptr.as_float();
                *flt_ptr = mean;
            }
            "ModeInt" => {
                let mode = self.eval_modeint(base_addr, low, high);
                let mut to_mem_ptr = self.get_mem_ptr(to_addr);
                let int_ptr = to_mem_ptr.as_int();
                *int_ptr = mode;
            }
            "MedianFlt" => {
                let med = self.eval_median(base_addr, low, high);
                let mut to_mem_ptr = self.get_mem_ptr(to_addr);
                let flt_ptr = to_mem_ptr.as_float();
                *flt_ptr = med;
            }
            "MedianInt" => {
                let med = self.eval_median(base_addr, low, high);
                let mut to_mem_ptr = self.get_mem_ptr(to_addr);
                let flt_ptr = to_mem_ptr.as_float();
                *flt_ptr = med;
            }
            "StdDevFlt" => {
                let stddev = self.eval_stddev(base_addr, low, high);
                let mut to_mem_ptr = self.get_mem_ptr(to_addr);
                let flt_ptr = to_mem_ptr.as_float();
                *flt_ptr = stddev;
            }
            "StdDevInt" => {
                let stddev = self.eval_stddev(base_addr, low, high);
                let mut to_mem_ptr = self.get_mem_ptr(to_addr);
                let flt_ptr = to_mem_ptr.as_float();
                *flt_ptr = stddev;
            }
            "VarianceFlt" => {
                let variance = self.eval_variance(base_addr, low, high);
                let mut to_mem_ptr = self.get_mem_ptr(to_addr);
                let flt_ptr = to_mem_ptr.as_float();
                *flt_ptr = variance;
            }
            "VarianceInt" => {
                let variance = self.eval_variance(base_addr, low, high);
                let mut to_mem_ptr = self.get_mem_ptr(to_addr);
                let flt_ptr = to_mem_ptr.as_float();
                *flt_ptr = variance;
            }
            _ => {
                eprintln!("DEV ERROR: Unknown statistics function: {}", fn_name);
                panic!();
            }
        }
        self.move_ip(1);
    }

    pub fn eval_minflt(&mut self, base_addr: i32, low: i32, high: i32) -> f64 {
        let mut min = self.get_mem_val(base_addr + low).as_float();
        for i in low..=high {
            let nw = self.get_mem_val(base_addr + i).as_float();
            if nw < min {
                min = nw;
            }
        }
        min
    }

    pub fn eval_minint(&mut self, base_addr: i32, low: i32, high: i32) -> i32 {
        let mut min = self.get_mem_val(base_addr + low).as_int();
        for i in low..=high {
            let nw = self.get_mem_val(base_addr + i).as_int();
            if nw < min {
                min = nw;
            }
        }
        min
    }

    pub fn eval_maxflt(&mut self, base_addr: i32, low: i32, high: i32) -> f64 {
        let mut max = self.get_mem_val(base_addr + low).as_float();
        for i in low..=high {
            let nw = self.get_mem_val(base_addr + i).as_float();
            if nw > max {
                max = nw;
            }
        }
        max
    }

    pub fn eval_maxint(&mut self, base_addr: i32, low: i32, high: i32) -> i32 {
        let mut max = self.get_mem_val(base_addr + low).as_int();
        for i in low..=high {
            let nw = self.get_mem_val(base_addr + i).as_int();
            if nw > max {
                max = nw;
            }
        }
        max
    }

    pub fn eval_rangeflt(&mut self, base_addr: i32, low: i32, high: i32) -> f64 {
        self.eval_maxflt(base_addr, low, high) - self.eval_minflt(base_addr, low, high)
    }

    pub fn eval_rangeint(&mut self, base_addr: i32, low: i32, high: i32) -> i32 {
        self.eval_maxint(base_addr, low, high) - self.eval_minint(base_addr, low, high)
    }

    pub fn eval_mean(&mut self, base_addr: i32, low: i32, high: i32) -> f64 {
        let mut sum: f64 = 0.0;
        let len = (high - low + 1) as f64;
        for i in low..=high {
            sum = sum + self.get_mem_val(base_addr + i).as_float();
        }
        sum / len
    }

    pub fn eval_modeint(&mut self, base_addr: i32, low: i32, high: i32) -> i32 {
        let mut mode: (i32, i32) = (0, 0);
        let mut map: HashMap<i32, i32> = HashMap::new();
        for i in low..=high {
            let nw = self.get_mem_val(base_addr + i).as_int();
            let map_ref = map.get_mut(&nw);
            if map_ref.is_none() {
                map.insert(nw, 1);
            } else {
                *map_ref.unwrap() += 1;
            }
            let val = map.get(&nw).unwrap();
            if val > &mode.1 || (val == &mode.1 && nw < mode.0) {
                mode = (nw, *val);
            }
        }
        mode.0
    }

    pub fn eval_median(&mut self, base_addr: i32, low: i32, high: i32) -> f64 {
        let mut vec: Vec<f64> = Vec::new();
        for i in low..=high {
            vec.push(self.get_mem_val(base_addr + i).as_float());
        }
        vec.sort_by(|a, b| a.partial_cmp(b).unwrap());
        if vec.len() % 2 == 0 {
            let mid = vec.len() / 2;
            (vec[mid] + vec[mid - 1]) / 2.0
        } else {
            let mid = vec.len() / 2;
            vec[mid]
        }
    }

    pub fn eval_stddev(&mut self, base_addr: i32, low: i32, high: i32) -> f64 {
        self.eval_variance(base_addr, low, high).sqrt()
    }

    pub fn eval_variance(&mut self, base_addr: i32, low: i32, high: i32) -> f64 {
        let avg: f64 = self.eval_mean(base_addr, low, high);
        let mut sum: f64 = 0.0;
        let len = (high - low + 1) as f64;
        for i in low..=high {
            let nw = self.get_mem_val(base_addr + i).as_float() - avg;
            sum += nw * nw;
        }
        sum / len
    }

    pub fn quad_parameter(&mut self, quad: &[String; 4]) {
        let addr_from = as_i32(&quad[1]);
        let val_from = self.get_mem_val(addr_from);
        let addr_to = as_i32(&quad[2]);
        self.stack_params.push((val_from, addr_to));
        self.move_ip(1);
    }

    pub fn quad_return(&mut self) {
        self.quad_endfunc();
    }

    pub fn quad_endfunc(&mut self) {
        self.stack_ips.pop();
        self.move_ip(1);
        // Local mem
        let loc_mem = &mut self.loc_mem;
        let stack_mem = self.stack_mems.last_mut().unwrap();
        mem::swap(loc_mem, stack_mem);
        self.stack_mems.pop();
        // Local ptrs
        let loc_ptrs = &mut self.loc_ptrs;
        let stack_ptrs = self.stack_ptrs.last_mut().unwrap();
        mem::swap(loc_ptrs, stack_ptrs);
        self.stack_ptrs.pop();
    }

    pub fn quad_era(&mut self) {
        self.stack_params.push((MemVal::Int(0), -1));
        self.move_ip(1);
    }

    pub fn setup_new_mem_fn(&mut self, fn_name: &String) {
        let fn_sz = self.mem_szs.get(fn_name).unwrap();
        let mut new_mem = Memory::new(
            fn_sz.locs[0] + fn_sz.tmps[0],
            fn_sz.locs[0],
            fn_sz.locs[1] + fn_sz.tmps[1],
            fn_sz.locs[1],
            fn_sz.locs[2] + fn_sz.tmps[2],
            fn_sz.locs[2],
        );
        // Loc mem
        let old_mem = &mut self.loc_mem;
        mem::swap(&mut new_mem, old_mem);
        self.stack_mems.push(new_mem);
        // Loc ptrs
        let mut new_ptrs = HashMap::new();
        let old_ptrs = &mut self.loc_ptrs;
        mem::swap(&mut new_ptrs, old_ptrs);
        self.stack_ptrs.push(new_ptrs);
    }

    pub fn quad_gosub(&mut self, quad: &[String; 4]) {
        self.setup_new_mem_fn(&quad[1]);
        self.copy_params_fn();
        let new_ip = as_i32(&quad[2]);
        self.stack_ips.push(new_ip);
    }

    pub fn copy_params_fn(&mut self) {
        loop {
            let (val_from, addr) = self.stack_params.pop().unwrap();
            if addr == -1 {
                break;
            }
            let ptr_to = self.get_mem_ptr(addr);
            match (val_from, ptr_to) {
                (MemVal::Int(f), MemPtr::Int(t)) => {
                    *t = f;
                }
                (MemVal::Int(f), MemPtr::Float(t)) => {
                    *t = f as f64;
                }
                (MemVal::Int(f), MemPtr::Bool(t)) => {
                    *t = f != 0;
                }
                (MemVal::Float(f), MemPtr::Float(t)) => {
                    *t = f;
                }
                (MemVal::Float(f), MemPtr::Int(t)) => {
                    *t = f as i32;
                }
                (MemVal::Bool(f), MemPtr::Bool(t)) => {
                    *t = f;
                }
                (MemVal::Bool(f), MemPtr::Int(t)) => {
                    *t = f as i32;
                }
                (f, t) => {
                    eprintln!(
                        "\nDEV ERROR: Params setup should be of same type. Got {:?} to {:?}",
                        f, t
                    );
                    panic!();
                }
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

    pub fn quad_deref(&mut self, quad: &[String; 4]) {
        let imp_addr_addr = as_i32(&quad[1]);
        if let MemVal::Int(imp_addr) = self.get_mem_val(imp_addr_addr) {
            let new_ptr = as_i32(&quad[2]);
            self.loc_ptrs.insert(new_ptr, imp_addr);
        } else {
            eprintln!(
                "\nDEV ERROR: Deref first element should always be Int, it got {:?}",
                self.get_mem_val(imp_addr_addr)
            );
            panic!();
        }
        self.move_ip(1);
    }

    pub fn quad_verify(&mut self, quad: &[String; 4]) {
        let addr_val = as_i32(&quad[1]);
        let lim = as_i32(&quad[2]);
        if let MemVal::Int(val) = self.get_mem_val(addr_val) {
            if val >= lim || val < 0 {
                eprintln!("\nEXECUTION ERROR: Out of Bounds. Tried to access an array in an invalid index: {}.\nIndex was expected between {} and {}\n", val, 0, lim - 1);
                panic!();
            }
        } else {
            eprintln!(
                "\nDEV ERROR: Verify first element must contain an Int. It contains: {:?}\n",
                self.get_mem_val(addr_val)
            );
            panic!();
        }
        self.move_ip(1);
    }

    pub fn quad_read(&mut self, quad: &[String; 4]) {
        let addr_to = as_i32(&quad[1]);
        let ptr_to = self.get_mem_ptr(addr_to);
        // Read line
        let mut buffer = String::new();
        std::io::stdin()
            .read_line(&mut buffer)
            .expect("\nEXECUTION ERROR: Error reading variable\n");
        match ptr_to {
            MemPtr::Int(t) => {
                *t = buffer.trim().parse::<i32>().unwrap();
            }
            MemPtr::Float(t) => {
                *t = buffer.trim().parse::<f64>().unwrap();
            }
            MemPtr::Bool(t) => {
                *t = buffer
                    .trim()
                    .to_lowercase()
                    .parse::<bool>()
                    .unwrap_or(buffer.trim().to_lowercase().parse::<i32>().unwrap() != 0);
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
            let output = format!("{}", val_from);
            print!("{}", output);
            self.output.push_str(&output);
            return;
        }
        let val_from = self.get_mem_val(addr_from);
        match val_from {
            MemVal::Int(t) => {
                let output = format!("{}", t);
                print!("{}", output);
                self.output.push_str(&output);
            }
            MemVal::Float(t) => {
                let output = format!("{}", t);
                print!("{}", output);
                self.output.push_str(&output);
            }
            MemVal::Bool(t) => {
                let output = format!("{}", if t { "True" } else { "False" });
                print!("{}", output);
                self.output.push_str(&output);
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
                    "\nDEV ERROR: This arithmetic op should not exist in compilaton: {:?} {:?} {:?} {:?}\n",
                    v1, v2, to, op
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

    pub fn get_mem_ptr(&mut self, addr_ptr: i32) -> MemPtr {
        // Check if ptr and convert to addr
        let mut addr = addr_ptr;
        if self.loc_ptrs.get(&addr_ptr).is_some() {
            addr = *self.loc_ptrs.get(&addr_ptr).unwrap();
        }
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
        eprintln!("\nDEV ERROR: Memory should always be found and correct");
        panic!();
    }

    pub fn get_mem_val(&mut self, addr_ptr: i32) -> MemVal {
        // Check if ptr and convert to addr
        let mut addr = addr_ptr;
        if self.loc_ptrs.get(&addr_ptr).is_some() {
            addr = *self.loc_ptrs.get(&addr_ptr).unwrap();
        }
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
        eprintln!("\nDEV ERROR: Memory should always be found and correct");
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
