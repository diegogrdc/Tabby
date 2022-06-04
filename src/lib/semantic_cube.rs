use crate::tipo::Tipo;
use crate::tipo::Tipo::{Bool, Float, Int};
use std::collections::HashMap;

pub struct SemanticCube {
    pub table: HashMap<(i32, i32, i32), i32>,
}

pub type SC = SemanticCube;

impl SemanticCube {
    pub fn new() -> SemanticCube {
        // We just create valid ops
        let mut table: HashMap<(i32, i32, i32), i32> = HashMap::new();

        // Or and And
        let logic_ops = ["Or", "And"];
        for op in logic_ops {
            table.insert(
                (
                    SC::tipo_to_val(&Int),
                    SC::tipo_to_val(&Int),
                    SC::op_to_val(op),
                ),
                SC::tipo_to_val(&Bool),
            );
            table.insert(
                (
                    SC::tipo_to_val(&Int),
                    SC::tipo_to_val(&Bool),
                    SC::op_to_val(op),
                ),
                SC::tipo_to_val(&Bool),
            );
            table.insert(
                (
                    SC::tipo_to_val(&Bool),
                    SC::tipo_to_val(&Int),
                    SC::op_to_val(op),
                ),
                SC::tipo_to_val(&Bool),
            );
            table.insert(
                (
                    SC::tipo_to_val(&Bool),
                    SC::tipo_to_val(&Bool),
                    SC::op_to_val(op),
                ),
                SC::tipo_to_val(&Bool),
            );
        }

        // Greater than, Greater or equal than
        // Les than, Less or equal than
        // Not equal, Equal
        let cond_ops = [">=", ">", "<", "<=", "!=", "=="];
        for op in cond_ops {
            table.insert(
                (
                    SC::tipo_to_val(&Int),
                    SC::tipo_to_val(&Int),
                    SC::op_to_val(op),
                ),
                SC::tipo_to_val(&Bool),
            );
            table.insert(
                (
                    SC::tipo_to_val(&Int),
                    SC::tipo_to_val(&Float),
                    SC::op_to_val(op),
                ),
                SC::tipo_to_val(&Bool),
            );
            table.insert(
                (
                    SC::tipo_to_val(&Int),
                    SC::tipo_to_val(&Bool),
                    SC::op_to_val(op),
                ),
                SC::tipo_to_val(&Bool),
            );
            table.insert(
                (
                    SC::tipo_to_val(&Bool),
                    SC::tipo_to_val(&Int),
                    SC::op_to_val(op),
                ),
                SC::tipo_to_val(&Bool),
            );
            table.insert(
                (
                    SC::tipo_to_val(&Bool),
                    SC::tipo_to_val(&Bool),
                    SC::op_to_val(op),
                ),
                SC::tipo_to_val(&Bool),
            );
            table.insert(
                (
                    SC::tipo_to_val(&Float),
                    SC::tipo_to_val(&Int),
                    SC::op_to_val(op),
                ),
                SC::tipo_to_val(&Bool),
            );
            table.insert(
                (
                    SC::tipo_to_val(&Float),
                    SC::tipo_to_val(&Float),
                    SC::op_to_val(op),
                ),
                SC::tipo_to_val(&Bool),
            );
        }
        // Sum, Substract
        // Multiply, Divide
        let algebraic_ops = ["+", "-", "*", "/"];
        for op in algebraic_ops {
            table.insert(
                (
                    SC::tipo_to_val(&Int),
                    SC::tipo_to_val(&Int),
                    SC::op_to_val(op),
                ),
                SC::tipo_to_val(&Int),
            );
            table.insert(
                (
                    SC::tipo_to_val(&Int),
                    SC::tipo_to_val(&Float),
                    SC::op_to_val(op),
                ),
                SC::tipo_to_val(&Float),
            );
            table.insert(
                (
                    SC::tipo_to_val(&Int),
                    SC::tipo_to_val(&Bool),
                    SC::op_to_val(op),
                ),
                SC::tipo_to_val(&Int),
            );
            table.insert(
                (
                    SC::tipo_to_val(&Float),
                    SC::tipo_to_val(&Int),
                    SC::op_to_val(op),
                ),
                SC::tipo_to_val(&Float),
            );
            table.insert(
                (
                    SC::tipo_to_val(&Float),
                    SC::tipo_to_val(&Float),
                    SC::op_to_val(op),
                ),
                SC::tipo_to_val(&Float),
            );
            table.insert(
                (
                    SC::tipo_to_val(&Bool),
                    SC::tipo_to_val(&Int),
                    SC::op_to_val(op),
                ),
                SC::tipo_to_val(&Int),
            );
            table.insert(
                (
                    SC::tipo_to_val(&Bool),
                    SC::tipo_to_val(&Bool),
                    SC::op_to_val(op),
                ),
                SC::tipo_to_val(&Int),
            );
            table.insert(
                (
                    SC::tipo_to_val(&Float),
                    SC::tipo_to_val(&Bool),
                    SC::op_to_val(op),
                ),
                SC::tipo_to_val(&Float),
            );
            table.insert(
                (
                    SC::tipo_to_val(&Bool),
                    SC::tipo_to_val(&Float),
                    SC::op_to_val(op),
                ),
                SC::tipo_to_val(&Float),
            );
        }
        // Assigmnent and Return
        let assign_ops = ["=", "Return", "Param"];
        for op in assign_ops {
            table.insert(
                (
                    SC::tipo_to_val(&Int),
                    SC::tipo_to_val(&Int),
                    SC::op_to_val(op),
                ),
                SC::tipo_to_val(&Int),
            );
            table.insert(
                (
                    SC::tipo_to_val(&Int),
                    SC::tipo_to_val(&Float),
                    SC::op_to_val(op),
                ),
                SC::tipo_to_val(&Int),
            );
            table.insert(
                (
                    SC::tipo_to_val(&Int),
                    SC::tipo_to_val(&Bool),
                    SC::op_to_val(op),
                ),
                SC::tipo_to_val(&Int),
            );
            table.insert(
                (
                    SC::tipo_to_val(&Bool),
                    SC::tipo_to_val(&Int),
                    SC::op_to_val(op),
                ),
                SC::tipo_to_val(&Bool),
            );
            table.insert(
                (
                    SC::tipo_to_val(&Bool),
                    SC::tipo_to_val(&Bool),
                    SC::op_to_val(op),
                ),
                SC::tipo_to_val(&Bool),
            );
            table.insert(
                (
                    SC::tipo_to_val(&Float),
                    SC::tipo_to_val(&Int),
                    SC::op_to_val(op),
                ),
                SC::tipo_to_val(&Float),
            );
            table.insert(
                (
                    SC::tipo_to_val(&Float),
                    SC::tipo_to_val(&Float),
                    SC::op_to_val(op),
                ),
                SC::tipo_to_val(&Float),
            );
        }
        SemanticCube { table: table }
    }

    pub fn op_to_val(s: &str) -> i32 {
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
            "=" => 12,
            "Return" => 13,
            "Param" => 14,
            _ => -1,
        }
    }

    pub fn val_to_op(v: i32) -> String {
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
            12 => "=".to_string(),
            13 => "Return".to_string(),
            14 => "Param".to_string(),
            _ => "Err".to_string(),
        }
    }

    pub fn tipo_to_val(t: &Tipo) -> i32 {
        match t {
            Tipo::Int => 0,
            Tipo::Float => 1,
            Tipo::Bool => 2,
            _ => 3,
        }
    }

    pub fn val_to_tipo(v: i32) -> Tipo {
        match v {
            0 => Tipo::Int,
            1 => Tipo::Float,
            2 => Tipo::Bool,
            _ => Tipo::Int,
        }
    }
}
