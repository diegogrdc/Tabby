#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub tabby); // synthesized by LALRPOP

use std::fs;
pub mod ast;
pub mod tests;

fn main() {
    let filename = "./tests/customTest.tabby";
    let contents = fs::read_to_string(filename).expect("\nSomething went wrong reading the file\n");
    let res = tabby::PROGRAMParser::new().parse(&contents);
    match res {
        Ok(my_ast) => {
            println!("\n    Tabby Program was correctly parsed!\n\n");
            println!("{:#?}", my_ast)
        }
        Err(error) => println!(
            "\n\nParsing Problem parsing Tabby file:\n\n    {:?}\n\n",
            error
        ),
    }
}
