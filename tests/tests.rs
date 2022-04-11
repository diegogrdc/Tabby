#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub tabby); // synthesized by LALRPOP

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_id_parsing() {
        assert!(tabby::IDParser::new().parse("idCorrect").is_ok());
        assert!(tabby::IDParser::new().parse("validID").is_ok());
        assert!(tabby::IDParser::new().parse("InvalidID").is_err());
        assert!(tabby::IDParser::new().parse("cantHaveNumb3rs").is_err());
        assert!(tabby::IDParser::new().parse("idWeird_nope_?12!").is_err());
    }
    #[test]
    fn test_int_parsing() {
        assert!(tabby::INTParser::new().parse("123").is_ok());
        assert!(tabby::INTParser::new().parse("+999").is_ok());
        assert!(tabby::INTParser::new().parse("0").is_ok());
        assert!(tabby::INTParser::new().parse("-123").is_ok());
        assert!(tabby::INTParser::new().parse("+-12").is_err());
        assert!(tabby::INTParser::new().parse("12.3").is_err());
    }

    #[test]
    fn test_float_parsing() {
        assert!(tabby::FLOATParser::new().parse("123.5").is_ok());
        assert!(tabby::FLOATParser::new().parse("+0.123").is_ok());
        assert!(tabby::FLOATParser::new().parse("-12.3333").is_ok());
        assert!(tabby::FLOATParser::new().parse("12").is_err());
        assert!(tabby::FLOATParser::new().parse("0.").is_err());
        assert!(tabby::FLOATParser::new().parse("+-2.1").is_err());
    }

    #[test]
    fn test_char_parsing() {
        assert!(tabby::CHARParser::new().parse("'a'").is_ok());
        assert!(tabby::CHARParser::new().parse("'1'").is_ok());
        assert!(tabby::CHARParser::new().parse("'?'").is_ok());
        assert!(tabby::CHARParser::new().parse("'nope'").is_err());
        assert!(tabby::CHARParser::new().parse("'nope2").is_err());
        assert!(tabby::CHARParser::new().parse("!").is_err());
    }

    #[test]
    fn test_bool_parsing() {
        assert!(tabby::BOOLParser::new().parse("True").is_ok());
        assert!(tabby::BOOLParser::new().parse("False").is_ok());
        assert!(tabby::BOOLParser::new().parse("true").is_err());
        assert!(tabby::BOOLParser::new().parse("false").is_err());
        assert!(tabby::BOOLParser::new().parse("if").is_err());
    }
    #[test]
    fn test_stringlit_parsing() {
        assert!(tabby::STRINGLITParser::new()
            .parse(r#""string literal""#)
            .is_ok());
        assert!(tabby::STRINGLITParser::new().parse(r#""""#).is_ok());
        assert!(tabby::STRINGLITParser::new()
            .parse(r#""unmatched end"#)
            .is_err());
        assert!(tabby::STRINGLITParser::new().parse(r#""no"pe""#).is_err());
    }
    #[test]
    fn test_fact_parsing() {
        assert!(tabby::FACTParser::new().parse("12").is_ok());
        assert!(tabby::FACTParser::new().parse("-37.5").is_ok());
        assert!(tabby::FACTParser::new().parse("True").is_ok());
        assert!(tabby::FACTParser::new().parse("function()").is_ok());
        assert!(tabby::FACTParser::new().parse("variable").is_ok());
        assert!(tabby::FACTParser::new().parse("((wow))").is_ok());
        assert!(tabby::FACTParser::new().parse("'a'").is_ok());

        assert!(tabby::FACTParser::new().parse("((wow)").is_err());
        assert!(tabby::FACTParser::new()
            .parse("invalidFunction();")
            .is_err());
        assert!(tabby::FACTParser::new().parse("Int").is_err());
        assert!(tabby::FACTParser::new().parse("(Nope)").is_err());
    }

    #[test]
    fn test_term_parsing() {
        assert!(tabby::TERMParser::new().parse("12 * 15").is_ok());
        assert!(tabby::TERMParser::new().parse("12.5 / var").is_ok());
        assert!(tabby::TERMParser::new()
            .parse("(9 * 5) / (2 * (3 * 5))")
            .is_ok());
        assert!(tabby::TERMParser::new().parse("12 ** 5").is_err());
        assert!(tabby::TERMParser::new().parse("/ 5").is_err());
        assert!(tabby::TERMParser::new().parse("4 *").is_err());
    }
    #[test]
    fn test_mexp_parsing() {
        assert!(tabby::MEXPParser::new().parse("12.5 + function()").is_ok());
        assert!(tabby::MEXPParser::new()
            .parse("12.5 + (var * 43 - 12.4) * True")
            .is_ok());

        assert!(tabby::MEXPParser::new().parse("- 45 + 12").is_err());
        assert!(tabby::MEXPParser::new().parse("(2 - 2 -) * 4").is_err());
    }
    #[test]
    fn test_gexp_parsing() {
        assert!(tabby::GEXPParser::new().parse("12 > 5").is_ok());
        assert!(tabby::GEXPParser::new().parse("12 != 4.123").is_ok());
        assert!(tabby::GEXPParser::new()
            .parse("function() >= (12.5 != 3)")
            .is_ok());

        assert!(tabby::GEXPParser::new().parse("12 > 5 > 3").is_err());
        assert!(tabby::GEXPParser::new().parse("12 <> 45").is_err());
        assert!(tabby::GEXPParser::new().parse("12 ==").is_err());
    }
    #[test]
    fn test_texp_parsing() {
        assert!(tabby::TEXPParser::new().parse("True And False").is_ok());
        assert!(tabby::TEXPParser::new().parse("True And (12 < 5)").is_ok());
        assert!(tabby::TEXPParser::new().parse("5 + 4 And 12 * 5").is_ok());

        assert!(tabby::TEXPParser::new()
            .parse("5 + 4 And 12 * 5 And")
            .is_err());
        assert!(tabby::TEXPParser::new().parse("12 Or 14").is_err());
        assert!(tabby::TEXPParser::new().parse("True and False").is_err());
    }
    #[test]
    fn test_exp_parsing() {
        assert!(tabby::EXPParser::new().parse("True Or False").is_ok());
        assert!(tabby::EXPParser::new()
            .parse("12 And 15 Or 5.4 * funct() + var * 3 > 2")
            .is_ok());
        assert!(tabby::EXPParser::new()
            .parse("((9 * 4 + 12 >= 12) Or (12 + 54 And func() - 23.1))")
            .is_ok());
    }

    #[test]
    fn test_type_parsing() {
        assert!(tabby::TYPEParser::new().parse("Int").is_ok());
        assert!(tabby::TYPEParser::new().parse("Float").is_ok());
        assert!(tabby::TYPEParser::new().parse("Char").is_ok());
        assert!(tabby::TYPEParser::new().parse("Bool").is_ok());
        assert!(tabby::TYPEParser::new().parse("int").is_err());
        assert!(tabby::TYPEParser::new().parse("float").is_err());
        assert!(tabby::TYPEParser::new().parse("char").is_err());
        assert!(tabby::TYPEParser::new().parse("bool").is_err());
    }

    #[test]
    fn test_comp_parsing() {
        assert!(tabby::COMPParser::new().parse(">").is_ok());
        assert!(tabby::COMPParser::new().parse("<").is_ok());
        assert!(tabby::COMPParser::new().parse(">=").is_ok());
        assert!(tabby::COMPParser::new().parse("<=").is_ok());
        assert!(tabby::COMPParser::new().parse("!=").is_ok());
        assert!(tabby::COMPParser::new().parse("==").is_ok());
    }

    #[test]
    fn test_return_parsing() {
        assert!(tabby::RETURNParser::new().parse("Return 12 * 5").is_ok());
        assert!(tabby::RETURNParser::new()
            .parse("Return function()")
            .is_ok());
        assert!(tabby::RETURNParser::new()
            .parse("Return function() * anotherFunc * var")
            .is_ok());
        assert!(tabby::RETURNParser::new().parse("return nope").is_err());
        assert!(tabby::RETURNParser::new()
            .parse("Return semicolon;")
            .is_err());
    }

    #[test]
    fn test_cond_parsing() {
        assert!(tabby::CONDParser::new().parse("If (12 > 5) {}").is_ok());
        assert!(tabby::CONDParser::new()
            .parse("If (False) {} Else {}")
            .is_ok());
        assert!(tabby::CONDParser::new()
            .parse("If (12 > 5) {} Else")
            .is_err());
        assert!(tabby::CONDParser::new().parse("Else {}").is_err());
    }

    #[test]
    fn test_ciclef_parsing() {
        assert!(tabby::CICLEFParser::new()
            .parse("For( i > 5; i = i + 2) {}")
            .is_ok());
        assert!(tabby::CICLEFParser::new()
            .parse("For( (12 And 5) * 4; id = function()) {}")
            .is_ok());
        assert!(tabby::CICLEFParser::new().parse("For(12 > 5) {}").is_err());
        assert!(tabby::CICLEFParser::new()
            .parse("For(i = 0; i < 3; i = i + 1) {}")
            .is_err());
    }

    #[test]
    fn test_ciclew_parsing() {
        assert!(tabby::CICLEWParser::new().parse("While(True) {}").is_ok());
        assert!(tabby::CICLEWParser::new()
            .parse("While(function() == var And True) {}")
            .is_ok());
        assert!(tabby::CICLEWParser::new().parse("While() {}").is_err());
        assert!(tabby::CICLEWParser::new()
            .parse("While(i < 3; i = i + 1) {}")
            .is_err());
    }

    #[test]
    fn test_call_parsing() {
        assert!(tabby::CALLParser::new().parse("function()").is_ok());
        assert!(tabby::CALLParser::new().parse("function(12)").is_ok());
        assert!(tabby::CALLParser::new()
            .parse("fn(12 * 5, True, id, 'a')")
            .is_ok());

        assert!(tabby::CALLParser::new().parse("function(,)").is_err());
        assert!(tabby::CALLParser::new()
            .parse(r#"function("Nope")"#)
            .is_err());
    }
    #[test]
    fn test_variable_parsing() {
        assert!(tabby::VARIABLEParser::new().parse("var").is_ok());
        assert!(tabby::VARIABLEParser::new().parse("arr[12 * 5]").is_ok());
        assert!(tabby::VARIABLEParser::new()
            .parse("arr[12 * 5][True]")
            .is_ok());

        assert!(tabby::VARIABLEParser::new().parse("var[]").is_err());
        assert!(tabby::VARIABLEParser::new().parse("var()").is_err());
        assert!(tabby::VARIABLEParser::new().parse(r#"var["wow"]"#).is_err());
        assert!(tabby::VARIABLEParser::new()
            .parse("var[id][id2][id3]")
            .is_err());
    }

    #[test]
    fn test_print_parsing() {
        assert!(tabby::PRINTParser::new()
            .parse(r#"Write("Hello!", 12 * 5, function(), vars)"#)
            .is_ok());
        assert!(tabby::PRINTParser::new()
            .parse(r#"Write("Hello World!")"#)
            .is_ok());
        assert!(tabby::PRINTParser::new().parse("Write()").is_err());
        assert!(tabby::PRINTParser::new()
            .parse(r#"Write(12 "Nope")"#)
            .is_err());
    }

    #[test]
    fn test_assignment_parsing() {
        assert!(tabby::ASSIGNMENTParser::new().parse("var = 12 * 5").is_ok());
        assert!(tabby::ASSIGNMENTParser::new()
            .parse("var[54] = False")
            .is_ok());
        assert!(tabby::ASSIGNMENTParser::new()
            .parse("var = functionCall()")
            .is_ok());
        assert!(tabby::ASSIGNMENTParser::new().parse("var = True").is_ok());
        assert!(tabby::ASSIGNMENTParser::new().parse("var = 'c'").is_ok());
        assert!(tabby::ASSIGNMENTParser::new()
            .parse(r#"var = "str""#)
            .is_err());
        assert!(tabby::ASSIGNMENTParser::new().parse("var() = 12").is_err());
        assert!(tabby::ASSIGNMENTParser::new().parse("154 = var").is_err());
    }

    #[test]
    fn test_read_parsing() {
        assert!(tabby::READParser::new().parse("Read(var)").is_ok());
        assert!(tabby::READParser::new().parse("Read(var[12])").is_ok());
        assert!(tabby::READParser::new().parse("Read(var[i][j])").is_ok());
        assert!(tabby::READParser::new().parse("Read(var, var2)").is_err());
        assert!(tabby::READParser::new().parse("Read(var())").is_err());
    }

    #[test]
    fn test_tabby_parsing() {
        assert!(tabby::TABBYParser::new().parse("Tabby() {}").is_ok());
        assert!(tabby::TABBYParser::new().parse("Tabby()").is_err());
        assert!(tabby::TABBYParser::new().parse("tabby() {}").is_err());
    }
    #[test]
    fn test_params_parsing() {
        assert!(tabby::PARAMSParser::new().parse("").is_ok());
        assert!(tabby::PARAMSParser::new().parse("Int var").is_ok());
        assert!(tabby::PARAMSParser::new()
            .parse("Int varOne, Arr Float varTwo, Char varThree, Arr Bool varFour")
            .is_ok());
        assert!(tabby::PARAMSParser::new().parse("Arr Int var").is_ok());

        assert!(tabby::PARAMSParser::new().parse("var").is_err());
        assert!(tabby::PARAMSParser::new().parse("Void var").is_err());
        assert!(tabby::PARAMSParser::new().parse("Arr var").is_err());
        assert!(tabby::PARAMSParser::new().parse("Arr Void var").is_err());
    }

    #[test]
    fn test_function_parsing() {
        assert!(tabby::FUNCTIONParser::new()
            .parse("Fn Void fnc(Int var, Float varTwo) {}")
            .is_ok());
        assert!(tabby::FUNCTIONParser::new().parse("Fn Void f() {}").is_ok());
        assert!(tabby::FUNCTIONParser::new()
            .parse("Fn Int add(Int x) { Return x + 1; }")
            .is_ok());
        assert!(tabby::FUNCTIONParser::new()
            .parse("Void Fn fnc() {}")
            .is_err());
        assert!(tabby::FUNCTIONParser::new()
            .parse("Fn Char fnc();")
            .is_err());
        assert!(tabby::FUNCTIONParser::new()
            .parse("Fn Char fnc {}")
            .is_err());
        assert!(tabby::FUNCTIONParser::new()
            .parse("Fn Void a() {} Fn Void b() {}")
            .is_err());
    }

    #[test]
    fn test_functions_parsing() {
        assert!(tabby::FUNCTIONSParser::new()
            .parse("Fn Void a() {} Fn Void b() {}")
            .is_ok());
        assert!(tabby::FUNCTIONSParser::new().parse("").is_ok());
        assert!(tabby::FUNCTIONSParser::new()
            .parse("Fn Void a() {}")
            .is_ok());
        assert!(tabby::FUNCTIONSParser::new()
            .parse("Fn Int Tabby() {}")
            .is_err());
    }

    #[test]
    fn test_vardec_parsing() {
        assert!(tabby::VARDECParser::new()
            .parse("Var Int id, idTwo, idThree;")
            .is_ok());
        assert!(tabby::VARDECParser::new().parse("Var Char char;").is_ok());
        assert!(tabby::VARDECParser::new()
            .parse("Var Arr Bool bitmask[15];")
            .is_ok());
        assert!(tabby::VARDECParser::new()
            .parse("Var Arr Int map[10][10];")
            .is_ok());

        assert!(tabby::VARDECParser::new()
            .parse("Var Arr Int map[10][];")
            .is_err());
        assert!(tabby::VARDECParser::new()
            .parse("Var Arr Int nope;")
            .is_err());
        assert!(tabby::VARDECParser::new()
            .parse("Var Int id, idTwo[15];")
            .is_err());
        assert!(tabby::VARDECParser::new()
            .parse("Var Int arr[1], arrTwo[2];")
            .is_err());
        assert!(tabby::VARDECParser::new()
            .parse("Var Int one; Var Bool two;")
            .is_err());
    }

    #[test]
    fn test_vardecs_parsing() {
        assert!(tabby::VARDECSParser::new().parse("").is_ok());
        assert!(tabby::VARDECSParser::new()
            .parse("Var Int one; Var Bool two;")
            .is_ok());
        assert!(tabby::VARDECSParser::new()
            .parse("Var Int one; Var Arr Bool two[12]; Var Arr Char three[1][2];")
            .is_ok());

        assert!(tabby::VARDECSParser::new()
            .parse("Var Int one, Var Char two;")
            .is_err());
    }

    #[test]
    fn test_statute_parsing() {
        assert!(tabby::STATUTEParser::new().parse("id = 12;").is_ok());
        assert!(tabby::STATUTEParser::new().parse("fibonacci();").is_ok());
        assert!(tabby::STATUTEParser::new().parse("Read(var);").is_ok());
        assert!(tabby::STATUTEParser::new().parse("Write(var, 12);").is_ok());
        assert!(tabby::STATUTEParser::new().parse("Return id;").is_ok());
        assert!(tabby::STATUTEParser::new().parse("While(True) {}").is_ok());
        assert!(tabby::STATUTEParser::new()
            .parse("For(True; i = i + 1) {}")
            .is_ok());
        assert!(tabby::STATUTEParser::new()
            .parse("If (True) {} Else {}")
            .is_ok());
        assert!(tabby::STATUTEParser::new().parse("Tabby();").is_err());
        assert!(tabby::STATUTEParser::new().parse("12 + 4;").is_err());
        assert!(tabby::STATUTEParser::new().parse("12 > 4;").is_err());
    }

    #[test]
    fn test_statutes_parsing() {
        assert!(tabby::STATUTESParser::new()
            .parse("Read(inpt); id = 12 + inpt; idTwo = id * 3; Write(inpt);")
            .is_ok());
        assert!(tabby::STATUTESParser::new().parse("").is_ok());
    }

    #[test]
    fn test_block_parsing() {
        assert!(tabby::BLOCKParser::new().parse("{ Write(12); }").is_ok());
        assert!(tabby::BLOCKParser::new().parse("{ Return True; }").is_ok());
        assert!(tabby::BLOCKParser::new()
            .parse("{ If (param > 5) { Return True; } Else { Return False; } }")
            .is_ok());
    }

    #[test]
    fn test_program_parsing() {
        assert!(tabby::PROGRAMParser::new()
            .parse(r#"Program helloWorld; Tabby() { Write("Hello World!"); }"#)
            .is_ok());
        assert!(tabby::PROGRAMParser::new()
            .parse(r#"Program test; Var Int one; Fn Int add(Int x) { Return x + 1; } Tabby() { one = 12; one = add(one); Write(one); }"#)
            .is_ok());
        assert!(tabby::PROGRAMParser::new()
            .parse(r#"Program helloWorld; Fn Void tabby() { Write("Hello World!"); }"#)
            .is_err());
    }
}
