// Execute Command: cargo run --bin execute file_name
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
pub mod virtual_machine;
use virtual_machine::VirtualMachine;
pub mod memory;
use memory::Memory;
pub mod cnsts_memory;
use cnsts_memory::CnstsMemory;

fn get_line(lines: &mut Lines<BufReader<File>>) -> String {
    lines.next().unwrap().unwrap()
}

fn get_filename() -> String {
    let args: Vec<String> = env::args().collect();
    let filename: String;
    if args.len() > 1 {
        filename = format!("{}", args[1]);
    } else {
        filename = "main".to_string();
    }
    filename
}

fn get_file_path(filename: &String) -> String {
    format!("./main/{}.tabbyic", filename)
}

fn get_lines(path: &String, filename: &String) -> Lines<BufReader<File>> {
    let file = File::open(path);
    if let Err(err) = file {
        eprintln!(
            "\nERROR: Problem reading contents of file: {}.tabby\nGot error: {}\n",
            filename, err
        );
        std::process::exit(1);
    }
    let file = file.unwrap();
    let reader = BufReader::new(file);
    reader.lines()
}

fn get_int_vec_from_string(lines: &mut Lines<BufReader<File>>) -> Vec<i32> {
    let line = get_line(lines);
    let vec: Vec<i32> = line.split(" ").map(|x| x.parse::<i32>().unwrap()).collect();
    vec
}

fn get_fn_mem_size(lines: &mut Lines<BufReader<File>>, vir_mach: &mut VirtualMachine) {
    let fn_name = get_line(lines);
    let locs = get_int_vec_from_string(lines);
    let tmps = get_int_vec_from_string(lines);
    let mem = Memory::new(
        locs[0] + tmps[0],
        locs[0],
        locs[1] + tmps[1],
        locs[1],
        locs[2] + tmps[2],
        locs[2],
    );
    vir_mach.mem_szs.insert(fn_name.to_string(), mem);
}

fn get_cnsts(lines: &mut Lines<BufReader<File>>, vir_mach: &mut VirtualMachine) {
    // Ints
    let int_sz = get_line(lines).parse::<i32>().unwrap();
    let line = get_line(lines);
    let vec_ints: Vec<i32> = if int_sz == 0 {
        Vec::new()
    } else {
        line.split(" ").map(|x| x.parse::<i32>().unwrap()).collect()
    };
    assert_eq!(int_sz, vec_ints.len() as i32);
    // Floats
    let float_sz = get_line(lines).parse::<i32>().unwrap();
    let line = get_line(lines);
    let vec_floats: Vec<f64> = if float_sz == 0 {
        Vec::new()
    } else {
        line.split(" ").map(|x| x.parse::<f64>().unwrap()).collect()
    };
    assert_eq!(float_sz, vec_floats.len() as i32);
    // Bools
    let bool_sz = get_line(lines).parse::<i32>().unwrap();
    let line = get_line(lines);
    let vec_bools: Vec<bool> = if bool_sz == 0 {
        Vec::new()
    } else {
        line.split(" ")
            .map(|x| x.to_lowercase())
            .map(|x| x.parse::<bool>().unwrap())
            .collect()
    };
    assert_eq!(bool_sz, vec_bools.len() as i32);
    // Strlits
    let strlit_sz = get_line(lines).parse::<i32>().unwrap();
    let line = get_line(lines);
    println!("line: {:?}", line);
    let vec_strlits: Vec<String> = if strlit_sz == 0 {
        Vec::new()
    } else {
        line.split(" ")
            .map(|x| x.parse::<String>().unwrap())
            .collect()
    };
    assert_eq!(strlit_sz, vec_strlits.len() as i32);

    // Set
    vir_mach.cnsts = CnstsMemory {
        int_cnst: vec_ints,
        float_cnst: vec_floats,
        bool_cnst: vec_bools,
        strlit_cnst: vec_strlits,
    }
}

fn parse_quad_from_string(line: String) -> [String; 4] {
    let vec: Vec<String> = line
        .split(" ")
        .map(|x| x.parse::<String>().unwrap())
        .collect();
    [
        vec[0].clone(),
        vec[1].clone(),
        vec[2].clone(),
        vec[3].clone(),
    ]
}

fn get_quads(lines: &mut Lines<BufReader<File>>, vir_mach: &mut VirtualMachine) {
    let quads_sz = get_line(lines).parse::<i32>().unwrap();
    for _ in 0..quads_sz {
        let line = get_line(lines);
        vir_mach.quads.push(parse_quad_from_string(line));
    }
}

fn main() {
    let filename = get_filename();
    let path = get_file_path(&filename);
    // Get contents
    let mut lines = get_lines(&path, &filename);
    // Init Virtual Machine
    let mut vir_mach = VirtualMachine::new();

    // Get program name
    vir_mach.prog_name = get_line(&mut lines);

    // Get fns memory
    let total_fns = get_line(&mut lines).parse::<i32>().unwrap();
    for _ in 0..total_fns {
        get_fn_mem_size(&mut lines, &mut vir_mach);
    }

    // Get cnsts
    get_cnsts(&mut lines, &mut vir_mach);

    // Get Quads
    get_quads(&mut lines, &mut vir_mach);

    // Debug
    println!("{:#?}", vir_mach);
}
