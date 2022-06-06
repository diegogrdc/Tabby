// Execute Command: cargo run --bin compile file_name

/*
Compilation Main Code

This code puts together all the tools
created for the process of compilation.
It reads and opens the file, gets its
contents, creates the AST, analyzes it,
and finally it prints all relevant information
to IC Code

*/

// Imports
/* #[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub tabby); // synthesized by LALRPOP */
mod tabby;
extern crate lib;
use lib::ast_evaluator::AstEvaluator;
use lib::quadruples::Quadruple;
use lib::*;
use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
mod tests;

// Function to get filename from
// command line arguments
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

// Adjust path from filename, to get from
// correct folder and with .tabby extension
fn get_file_path(filename: &String) -> String {
    format!("./main/{}.tabby", filename)
}

// Parse contents from source file
fn get_contents(path: &String, filename: &String) -> String {
    let contents = fs::read_to_string(path.clone());
    if let Err(err) = contents {
        eprintln!(
            "\nERROR: Problem reading contents of file: {}.tabby\nGot error: {}\n",
            filename, err
        );
        panic!();
    }
    contents.unwrap()
}

// Generate ast by using the LALRPOP parser
fn get_ast(contents: String, filename: &String) -> Box<ast::Program> {
    let res = tabby::PROGRAMParser::new().parse(&contents);
    if let Err(err) = res {
        eprintln!(
            "\nERROR: Problem parsing Tabby file: {}.tabby\nGot error: {}\n",
            filename, err
        );
        panic!();
    }
    // Get ast by unwrapping valid Result<>
    res.unwrap()
}

// Print all generated information of the AST
// to the IC code
fn print_ic_file(eval: AstEvaluator, filename: &String, program_name: &String) {
    let mut file = File::create(format!("./main/{}.tabbyic", filename))
        .expect("DEV ERROR: Could not create IC file");

    let err = "DEV ERROR: Could not write no IC file";
    let cnst_stacks = &eval.vir_mem_alloc.cnst_vals;

    // Print Program Name
    writeln!(file, "{}", program_name).expect(err);

    // First print # of functions
    writeln!(file, "{}", eval.dir_func.len()).expect(err);
    // Print Function Sizes (name, 3 local, 3 temps)
    for (fn_name, fn_info) in eval.dir_func.iter() {
        writeln!(file, "{}", fn_name).expect(err);
        writeln!(
            file,
            "{} {} {}",
            fn_info.size_loc[0], fn_info.size_loc[1], fn_info.size_loc[2]
        )
        .expect(err);
        writeln!(
            file,
            "{} {} {}",
            fn_info.size_tmp[0], fn_info.size_tmp[1], fn_info.size_tmp[2]
        )
        .expect(err);
    }
    // Print constants
    for i in 0..4 {
        let cnst_vec = &cnst_stacks[i];
        writeln!(file, "{}", cnst_vec.len()).expect(err);
        for (idx, cnst) in cnst_vec.iter().enumerate() {
            if i == 3 {
                writeln!(file, "{}", cnst).expect(err);
            } else if idx < cnst_vec.len() - 1 {
                write!(file, "{} ", cnst).expect(err);
            } else {
                write!(file, "{}", cnst).expect(err);
            }
        }
        if i != 3 {
            write!(file, "\n").expect(err);
        }
    }

    // Print quads
    writeln!(file, "{}", eval.quads.len()).expect(err);
    for quad in eval.quads {
        let quad_printable = Quadruple::get_printable(&quad);
        writeln!(file, "{}", quad_printable).expect(err);
    }
}

fn main() {
    // Get filename and path
    let filename = get_filename();
    let path = get_file_path(&filename);
    // Get contents
    let contents = get_contents(&path, &filename);
    // Use LALRPOP parser to generate AST
    let my_ast = get_ast(contents, &filename);

    // Evaluate AST
    let mut evaluator = AstEvaluator::new();
    evaluator.eval_program(my_ast);

    // Save program name
    let program_name = evaluator.glob_scope.clone().unwrap();

    // Print info to IC file
    print_ic_file(evaluator, &filename, &program_name);

    // Print user message of correct compilation
    println!("\nProgram \"{}\" compiled succesfully!\n", program_name);
}
