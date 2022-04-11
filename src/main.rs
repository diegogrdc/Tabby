use std::fs;

#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub tabby); // synthesized by LALRPOP

fn main() {
    let filename = "./tests/customTest.tabby";
    let contents = fs::read_to_string(filename).expect("\nSomething went wrong reading the file\n");
    let res = tabby::PROGRAMParser::new().parse(&contents);
    match res {
        Ok(_) => print!("\n    Tabby Program was correctly parsed!\n\n"),
        Err(error) => print!(
            "\n\nParsing Problem parsing Tabby file:\n\n    {:?}\n\n",
            error
        ),
    }
}
