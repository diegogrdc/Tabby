// Imports
#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub tabby); // synthesized by LALRPOP

use std::env;
use std::fs;
pub mod ast;
pub mod ast_evaluator;
use ast_evaluator::AstEvaluator;
pub mod dir_func;
pub mod dir_var;
pub mod quadruples;
use quadruples::Quadruple;
pub mod semantic_cube;
pub mod tests;
pub mod tipo;
pub mod vir_mem;
use std::fs::File;
use std::io::Write;

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
    format!("./main/{}.tabby", filename)
}

fn get_contents(path: &String, filename: &String) -> String {
    let contents = fs::read_to_string(path.clone());
    if let Err(err) = contents {
        eprintln!(
            "\nERROR: Problem reading contents of file: {}.tabby\nGot error: {}\n",
            filename, err
        );
        std::process::exit(1);
    }
    contents.unwrap()
}

fn get_ast(contents: String, filename: &String) -> Box<ast::Program> {
    let res = tabby::PROGRAMParser::new().parse(&contents);
    if let Err(err) = res {
        eprintln!(
            "\nERROR: Problem parsing Tabby file: {}.tabby\nGot error: {:?}\n",
            filename, err
        );
        std::process::exit(1);
    }
    // Get ast by unwrapping valid Result<>
    res.unwrap()
}

fn print_ic_file(eval: AstEvaluator, filename: &String, program_name: &String) {
    let mut file = File::create(format!("./main/{}.tabbyic", filename))
        .expect("DEV ERROR: Could not create IC file");

    let err = "DEV ERROR: Could not write no IC file";
    let cnst_stacks = &eval.vir_mem_alloc.cnst_vals;

    // Print Global Sizes (name, 3 local, 3 temps)
    let global_info = eval.dir_func.get(program_name).unwrap();
    writeln!(file, "{}", program_name).expect(err);
    writeln!(
        file,
        "{} {} {}",
        global_info.size_loc[0], global_info.size_loc[1], global_info.size_loc[2]
    )
    .expect(err);
    writeln!(file, "1 1 1").expect(err);

    // First print # of functions
    writeln!(file, "{}", eval.dir_func.len() - 1).expect(err);
    // Print Function Sizes (name, 3 local, 3 temps)
    for (fn_name, fn_info) in eval.dir_func.iter() {
        // Avoid global scope
        if fn_name == program_name {
            continue;
        }
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
        if cnst_vec.len() > 0 {
            for cnst in cnst_vec {
                write!(file, "{} ", cnst).expect(err);
            }
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

    print_ic_file(evaluator, &filename, &program_name);
    println!("\nProgram \"{}\" compiled succesfully!\n", program_name);
}
