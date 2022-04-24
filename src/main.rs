// Imports
#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub tabby); // synthesized by LALRPOP

use std::fs;
pub mod ast;
pub mod ast_evaluator;
use ast_evaluator::AstEvaluator;
pub mod dir_func;
pub mod dir_var;
pub mod tests;
pub mod tipo;

// Main File
fn main() {
    // Parse file
    let filename = "./tests/customTest.tabby";
    // Get contents
    let contents = fs::read_to_string(filename).expect("\nSomething went wrong reading the file\n");
    // Use LALRPOP parser to generate AST
    let res = tabby::PROGRAMParser::new().parse(&contents);
    if let Err(error) = res {
        panic!("\nERROR: Problem parsing Tabby file:\n\n    {:?}\n", error);
    }
    // Get ast by unwrapping valid Result<>
    let my_ast = res.unwrap();

    // Evaluate AST
    let mut evaluator = AstEvaluator::new();
    evaluator.eval_program(my_ast);

    // For debugging purposes, we print the directory function
    println!("{:#?}", evaluator.dir_func);
}
