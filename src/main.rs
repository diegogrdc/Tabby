#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub tabby); // synthesized by LALRPOP

use std::fs;
pub mod ast;
pub mod ast_evaluator;
use ast_evaluator::AstEvaluator;
pub mod tests;

fn main() {
    // Parse file
    let filename = "./tests/customTest.tabby";
    // Get contents
    let contents = fs::read_to_string(filename).expect("\nSomething went wrong reading the file\n");
    // Use LALRPOP parser to generate AST
    let res = tabby::PROGRAMParser::new().parse(&contents);
    if let Err(error) = res {
        println!(
            "\n\nParsing Problem parsing Tabby file:\n\n    {:?}\n\n",
            error
        );
        return;
    }
    // Get ast by unwrapping valid sResult<>
    let my_ast = res.unwrap();

    // Evaluate AST
    let evaluator = AstEvaluator::new();
    evaluator.eval_program(my_ast);
}
