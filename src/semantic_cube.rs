use crate::ast;
use crate::ast::Type::{Bool, Float, Int};
use std::collections::HashMap;

pub struct SemanticCube {
    table: HashMap<(i32, i32, i32), i32>,
}

type SC = SemanticCube;

impl SemanticCube {
    pub fn new() -> SemanticCube {
        // We just create valid ops
        let mut table: HashMap<(i32, i32, i32), i32> = HashMap::new();

        // Or and And
        let logic_ops = ["Or", "And"];
        for op in logic_ops {
            table.insert(
                (
                    SC::get_type_val(Int),
                    SC::get_type_val(Int),
                    SC::get_op_val(op),
                ),
                SC::get_type_val(Bool),
            );
            table.insert(
                (
                    SC::get_type_val(Int),
                    SC::get_type_val(Bool),
                    SC::get_op_val(op),
                ),
                SC::get_type_val(Bool),
            );
            table.insert(
                (
                    SC::get_type_val(Bool),
                    SC::get_type_val(Int),
                    SC::get_op_val(op),
                ),
                SC::get_type_val(Bool),
            );
            table.insert(
                (
                    SC::get_type_val(Bool),
                    SC::get_type_val(Bool),
                    SC::get_op_val(op),
                ),
                SC::get_type_val(Bool),
            );
        }

        // Greater than, Greater or equal than
        // Les than, Less or equal than
        // Not equal, Equal
        let cond_ops = [">=", ">", "<", "<=", "!=", "=="];
        for op in cond_ops {
            table.insert(
                (
                    SC::get_type_val(Int),
                    SC::get_type_val(Int),
                    SC::get_op_val(op),
                ),
                SC::get_type_val(Bool),
            );
            table.insert(
                (
                    SC::get_type_val(Int),
                    SC::get_type_val(Float),
                    SC::get_op_val(op),
                ),
                SC::get_type_val(Bool),
            );
            table.insert(
                (
                    SC::get_type_val(Int),
                    SC::get_type_val(Bool),
                    SC::get_op_val(op),
                ),
                SC::get_type_val(Bool),
            );
            table.insert(
                (
                    SC::get_type_val(Bool),
                    SC::get_type_val(Int),
                    SC::get_op_val(op),
                ),
                SC::get_type_val(Bool),
            );
            table.insert(
                (
                    SC::get_type_val(Bool),
                    SC::get_type_val(Float),
                    SC::get_op_val(op),
                ),
                SC::get_type_val(Bool),
            );
            table.insert(
                (
                    SC::get_type_val(Bool),
                    SC::get_type_val(Bool),
                    SC::get_op_val(op),
                ),
                SC::get_type_val(Bool),
            );
            table.insert(
                (
                    SC::get_type_val(Float),
                    SC::get_type_val(Int),
                    SC::get_op_val(op),
                ),
                SC::get_type_val(Bool),
            );
            table.insert(
                (
                    SC::get_type_val(Float),
                    SC::get_type_val(Float),
                    SC::get_op_val(op),
                ),
                SC::get_type_val(Bool),
            );
            table.insert(
                (
                    SC::get_type_val(Float),
                    SC::get_type_val(Bool),
                    SC::get_op_val(op),
                ),
                SC::get_type_val(Bool),
            );
        }
        // Sum, Substract
        // Multiply, Divide
        let algebraic_ops = ["+", "-", "*", "/"];
        for op in algebraic_ops {
            table.insert(
                (
                    SC::get_type_val(Int),
                    SC::get_type_val(Int),
                    SC::get_op_val(op),
                ),
                SC::get_type_val(Int),
            );
            table.insert(
                (
                    SC::get_type_val(Int),
                    SC::get_type_val(Float),
                    SC::get_op_val(op),
                ),
                SC::get_type_val(Float),
            );
            table.insert(
                (
                    SC::get_type_val(Int),
                    SC::get_type_val(Bool),
                    SC::get_op_val(op),
                ),
                SC::get_type_val(Int),
            );
            table.insert(
                (
                    SC::get_type_val(Float),
                    SC::get_type_val(Int),
                    SC::get_op_val(op),
                ),
                SC::get_type_val(Float),
            );
            table.insert(
                (
                    SC::get_type_val(Float),
                    SC::get_type_val(Float),
                    SC::get_op_val(op),
                ),
                SC::get_type_val(Float),
            );
            table.insert(
                (
                    SC::get_type_val(Bool),
                    SC::get_type_val(Int),
                    SC::get_op_val(op),
                ),
                SC::get_type_val(Int),
            );
            table.insert(
                (
                    SC::get_type_val(Bool),
                    SC::get_type_val(Bool),
                    SC::get_op_val(op),
                ),
                SC::get_type_val(Int),
            );
        }
        SemanticCube { table: table }
    }

    pub fn get_op_val(s: &str) -> i32 {
        match s {
            "Or" => 0,
            "And" => 1,
            ">" => 2,
            ">=" => 3,
            "<" => 4,
            "<=" => 5,
            "!=" => 6,
            "==" => 7,
            "+" => 8,
            "-" => 9,
            "*" => 10,
            "/" => 11,
            _ => -1,
        }
    }

    pub fn get_val_op(v: i32) -> String {
        match v {
            0 => "Or".to_string(),
            1 => "And".to_string(),
            2 => ">".to_string(),
            3 => ">=".to_string(),
            4 => "<".to_string(),
            5 => "<=".to_string(),
            6 => "!=".to_string(),
            7 => "==".to_string(),
            8 => "+".to_string(),
            9 => "-".to_string(),
            10 => "*".to_string(),
            11 => "/".to_string(),
            _ => "Err".to_string(),
        }
    }

    pub fn get_type_val(t: ast::Type) -> i32 {
        match t {
            ast::Type::Int => 0,
            ast::Type::Float => 1,
            ast::Type::Bool => 2,
        }
    }

    pub fn get_val_type(v: i32) -> ast::Type {
        match v {
            0 => ast::Type::Int,
            1 => ast::Type::Float,
            2 => ast::Type::Bool,
            _ => ast::Type::Int,
        }
    }
}
