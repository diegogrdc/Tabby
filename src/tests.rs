lalrpop_mod!(pub tabby); // synthesized by LALRPOP

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast_evaluator::AstEvaluator;
    use crate::quadruples::Quadruple;
    use crate::tipo::Tipo;
    use std::fs;
    /*
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
        assert!(tabby::TYPEParser::new().parse("Bool").is_ok());
        assert!(tabby::TYPEParser::new().parse("int").is_err());
        assert!(tabby::TYPEParser::new().parse("float").is_err());
        assert!(tabby::TYPEParser::new().parse("char").is_err());
        assert!(tabby::TYPEParser::new().parse("Char").is_err());
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
            .parse("fn(12 * 5, True, id)")
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
        assert!(tabby::PARAMSParser::new().parse("Int var").is_ok());
        assert!(tabby::PARAMSParser::new()
            .parse("Int varOne, Arr Float varTwo, Arr Bool varFour")
            .is_ok());
        assert!(tabby::PARAMSParser::new().parse("Arr Int var").is_ok());

        assert!(tabby::PARAMSParser::new().parse("var").is_err());
        assert!(tabby::PARAMSParser::new().parse("Void var").is_err());
        assert!(tabby::PARAMSParser::new().parse("Arr var").is_err());
        assert!(tabby::PARAMSParser::new().parse("Arr Void var").is_err());
        assert!(tabby::PARAMSParser::new().parse("").is_err());
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
        assert!(tabby::VARDECParser::new().parse("Var Int;").is_err());
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
            .parse("Var Int one; Var Arr Bool two[12]; Var Arr Float three[1][2];")
            .is_ok());

        assert!(tabby::VARDECSParser::new()
            .parse("Var Int one, Var Bool two;")
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

    // AST Testing
    #[test]
    fn test_id_ast() {
        assert_eq!(
            tabby::IDParser::new().parse("idCorrect").unwrap(),
            "idCorrect"
        );
        assert_eq!(tabby::IDParser::new().parse("validID").unwrap(), "validID");
    }
    #[test]
    fn test_int_ast() {
        assert_eq!(tabby::INTParser::new().parse("123").unwrap(), 123i32);
        assert_eq!(tabby::INTParser::new().parse("+999").unwrap(), 999i32);
        assert_eq!(tabby::INTParser::new().parse("-2").unwrap(), -2i32);
        assert_eq!(tabby::INTParser::new().parse("0").unwrap(), 0i32);
    }
    #[test]
    fn test_float_ast() {
        assert_eq!(tabby::FLOATParser::new().parse("123.5").unwrap(), 123.5f64);
        assert_eq!(tabby::FLOATParser::new().parse("+0.015").unwrap(), 0.015f64);
        assert_eq!(
            tabby::FLOATParser::new().parse("-12.333").unwrap(),
            -12.333f64
        );
    }

    #[test]
    fn test_bool_ast() {
        assert_eq!(tabby::BOOLParser::new().parse("True").unwrap(), true);
        assert_eq!(tabby::BOOLParser::new().parse("False").unwrap(), false);
    }
    #[test]
    fn test_stringlit_ast() {
        assert_eq!(
            tabby::STRINGLITParser::new()
                .parse(r#""string literal""#)
                .unwrap(),
            "string literal"
        );
        assert_eq!(tabby::STRINGLITParser::new().parse(r#""""#).unwrap(), "");
    }
    #[test]
    fn test_fact_ast() {
        assert_eq!(
            format!("{:?}", tabby::FACTParser::new().parse("(12)").unwrap()),
            "Parentheses(Texp([Gexp([Mexp(Term(Fact(Int(12))))])]))"
        );
    }
    #[test]
    fn test_term_ast() {
        assert_eq!(
            format!("{:?}", tabby::TERMParser::new().parse("12 * 15").unwrap()),
            "Mul(Fact(Int(12)), Int(15))"
        );
        assert_eq!(
            format!("{:?}", tabby::TERMParser::new().parse("0.1 / var").unwrap()),
            r#"Div(Fact(Float(0.1)), Variable(Id("var")))"#
        );
    }
    #[test]
    fn test_mexp_ast() {
        assert_eq!(
            format!("{:?}", tabby::MEXPParser::new().parse("12 + 2.1").unwrap()),
            "Sum(Term(Fact(Int(12))), Fact(Float(2.1)))"
        );
        assert_eq!(
            format!("{:?}", tabby::MEXPParser::new().parse("12 - 2.1").unwrap()),
            "Sub(Term(Fact(Int(12))), Fact(Float(2.1)))"
        );
    }
    #[test]
    fn test_gexp_ast() {
        assert_eq!(
            format!("{:?}", tabby::GEXPParser::new().parse("12 > 5").unwrap()),
            "Comp(Term(Fact(Int(12))), Greater, Term(Fact(Int(5))))"
        );
        assert_eq!(
            format!(
                "{:?}",
                tabby::GEXPParser::new().parse("12 != 4.123").unwrap()
            ),
            "Comp(Term(Fact(Int(12))), NotEqual, Term(Fact(Float(4.123))))"
        );
    }
    #[test]
    fn test_texp_ast() {
        assert_eq!(
            format!(
                "{:?}",
                tabby::TEXPParser::new().parse("True And False").unwrap()
            ),
            "Gexp([Mexp(Term(Fact(Bool(true)))), Mexp(Term(Fact(Bool(false))))])"
        );
    }
    #[test]
    fn test_exp_ast() {
        assert_eq!(
            format!(
                "{:?}",
                tabby::EXPParser::new().parse("True Or False").unwrap()
            ),
            "Texp([Gexp([Mexp(Term(Fact(Bool(true))))]), Gexp([Mexp(Term(Fact(Bool(false))))])])"
        );
    }
    #[test]
    fn test_type_ast() {
        assert_eq!(
            format!("{:?}", tabby::TYPEParser::new().parse("Int").unwrap()),
            "Int"
        );
        assert_eq!(
            format!("{:?}", tabby::TYPEParser::new().parse("Float").unwrap()),
            "Float"
        );
        assert_eq!(
            format!("{:?}", tabby::TYPEParser::new().parse("Bool").unwrap()),
            "Bool"
        );
    }

    #[test]
    fn test_comp_ast() {
        assert_eq!(
            format!("{:?}", tabby::COMPParser::new().parse(">").unwrap()),
            "Greater"
        );
        assert_eq!(
            format!("{:?}", tabby::COMPParser::new().parse("<").unwrap()),
            "Smaller"
        );
    }
    #[test]
    fn test_return_ast() {
        assert_eq!(
            format!(
                "{:?}",
                tabby::RETURNParser::new().parse("Return 12").unwrap()
            ),
            "Return(Texp([Gexp([Mexp(Term(Fact(Int(12))))])]))"
        );
    }

    #[test]
    fn test_cond_ast() {
        assert_eq!(
            format!(
                "{:?}",
                tabby::CONDParser::new().parse("If(True) {}").unwrap()
            ),
            "If(Texp([Gexp([Mexp(Term(Fact(Bool(true))))])]), Block(Statutes([])))"
        );
        assert_eq!(
            format!(
                "{:?}",
                tabby::CONDParser::new()
                    .parse("If(True) {} Else {}")
                    .unwrap()
            ),
            "IfElse(Texp([Gexp([Mexp(Term(Fact(Bool(true))))])]), Block(Statutes([])), Block(Statutes([])))"
        );
    }
    #[test]
    fn test_ciclef_ast() {
        assert_eq!(
            format!(
                "{:?}",
                tabby::CICLEFParser::new()
                    .parse("For(i; i = 1) {}")
                    .unwrap()
            ),
            "For(Texp([Gexp([Mexp(Term(Fact(Variable(Id(\"i\")))))])]), Assign(Id(\"i\"), Texp([Gexp([Mexp(Term(Fact(Int(1))))])])), Block(Statutes([])))"
        );
    }

    #[test]
    fn test_ciclew_ast() {
        assert_eq!(
            format!(
                "{:?}",
                tabby::CICLEWParser::new().parse("While(True) {}").unwrap()
            ),
            "While(Texp([Gexp([Mexp(Term(Fact(Bool(true))))])]), Block(Statutes([])))"
        );
    }

    #[test]
    fn test_call_ast() {
        assert_eq!(
            format!(
                "{:?}",
                tabby::CALLParser::new().parse("fn(12, id)").unwrap()
            ),
            "Call(\"fn\", [Texp([Gexp([Mexp(Term(Fact(Int(12))))])]), Texp([Gexp([Mexp(Term(Fact(Variable(Id(\"id\")))))])])])"
        );
    }
    #[test]
    fn test_variable_ast() {
        assert_eq!(
            format!(
                "{:?}",
                tabby::VARIABLEParser::new().parse("var[1][0]").unwrap()
            ),
            "Mat(\"var\", Texp([Gexp([Mexp(Term(Fact(Int(1))))])]), Texp([Gexp([Mexp(Term(Fact(Int(0))))])]))"
        );
    }

    #[test]
    fn test_print_ast() {
        assert_eq!(
            format!(
                "{:?}",
                tabby::PRINTParser::new().parse("Write(12, True)").unwrap()
            ),
            "Print(ExpPV(Texp([Gexp([Mexp(Term(Fact(Int(12))))])]), Exp(Texp([Gexp([Mexp(Term(Fact(Bool(true))))])]))))"
        );
    }

    #[test]
    fn test_assignment_ast() {
        assert_eq!(
            format!(
                "{:?}",
                tabby::ASSIGNMENTParser::new()
                    .parse("var = 12 * 4")
                    .unwrap()
            ),
            "Assign(Id(\"var\"), Texp([Gexp([Mexp(Term(Mul(Fact(Int(12)), Int(4))))])]))"
        );
    }

    #[test]
    fn test_read_ast() {
        assert_eq!(
            format!(
                "{:?}",
                tabby::READParser::new().parse("Read(a[i][j])").unwrap()
            ),
            "Read(Mat(\"a\", Texp([Gexp([Mexp(Term(Fact(Variable(Id(\"i\")))))])]), Texp([Gexp([Mexp(Term(Fact(Variable(Id(\"j\")))))])])))"
        );
    }

    #[test]
    fn test_tabby_ast() {
        assert_eq!(
            format!(
                "{:?}",
                tabby::TABBYParser::new().parse("Tabby() {}").unwrap()
            ),
            "Tabby(Block(Statutes([])))"
        );
    }
    #[test]
    fn test_params_ast() {
        assert_eq!(
            format!(
                "{:?}",
                tabby::PARAMSParser::new()
                    .parse("Int varOne, Arr Float varTwo")
                    .unwrap()
            ),
            "ParamAnd(Int, \"varOne\", ArrParam(Float, \"varTwo\"))"
        );
    }

    #[test]
    fn test_function_ast() {
        assert_eq!(
            format!(
                "{:?}",
                tabby::FUNCTIONParser::new()
                    .parse("Fn Void f() {}")
                    .unwrap()
            ),
            "FnVoid(\"f\", Block(Statutes([])))"
        );
    }

    #[test]
    fn test_functions_ast() {
        assert_eq!(
            format!(
                "{:?}",
                tabby::FUNCTIONSParser::new()
                    .parse("Fn Int a(Int p) {} Fn Void b() {}")
                    .unwrap()
            ),
            "Fns([FnParams(Int, \"a\", Param(Int, \"p\"), Block(Statutes([]))), FnVoid(\"b\", Block(Statutes([])))])"
        );
    }

    #[test]
    fn test_vardec_ast() {
        assert_eq!(
            format!(
                "{:?}",
                tabby::VARDECParser::new()
                    .parse("Var Arr Int map[10][10];")
                    .unwrap()
            ),
            "Mat(Int, \"map\", 10, 10)"
        );
    }

    #[test]
    fn test_vardecs_ast() {
        assert_eq!(
            format!(
                "{:?}",
                tabby::VARDECSParser::new()
                    .parse("Var Int one; Var Bool two;")
                    .unwrap()
            ),
            "Vardecs([Vars(Int, [\"one\"]), Vars(Bool, [\"two\"])])"
        );
    }

    #[test]
    fn test_statute_ast() {
        assert_eq!(
            format!(
                "{:?}",
                tabby::STATUTEParser::new().parse("fibonacci();").unwrap()
            ),
            "Call(Call(\"fibonacci\", []))"
        );
    }

    #[test]
    fn test_statutes_ast() {
        assert_eq!(
            format!(
                "{:?}",
                tabby::STATUTESParser::new()
                    .parse("Read(i); Write(i);")
                    .unwrap()
            ),
            "Statutes([Read(Read(Id(\"i\"))), Print(Print(Exp(Texp([Gexp([Mexp(Term(Fact(Variable(Id(\"i\")))))])]))))])"
        );
    }

    #[test]
    fn test_block_ast() {
        assert_eq!(
            format!(
                "{:?}",
                tabby::BLOCKParser::new().parse("{ Return True; }").unwrap()
            ),
            "Block(Statutes([Return(Return(Texp([Gexp([Mexp(Term(Fact(Bool(true))))])])))]))"
        );
    }

    #[test]
    fn test_program_ast() {
        assert_eq!(
            format!(
                "{:?}",
                tabby::PROGRAMParser::new()
                    .parse(r#"Program helloWorld; Tabby() { Write("Hello World!"); }"#)
                    .unwrap()
            ),
            "Program(\"helloWorld\", Vardecs([]), Fns([]), Tabby(Block(Statutes([Print(Print(StrLit(\"Hello World!\")))]))))"
        );
    }

    */

    #[test]
    fn test_dirs_ok_1() {
        let filename = "./tests/dirs_ok_1.tabby";
        let contents = fs::read_to_string(filename).unwrap();
        let res = tabby::PROGRAMParser::new().parse(&contents);
        assert!(res.is_ok());
        let my_ast = res.unwrap();
        let mut evaluator = AstEvaluator::new();
        evaluator.eval_program(my_ast);

        // Assert global scope and vars
        let program_name = "testAllFunctionalities";
        assert!(evaluator.dir_func.get(program_name).is_some());
        let glob_dir = evaluator.dir_func.get(program_name).unwrap();
        assert_eq!(glob_dir.tipo, Tipo::Program);
        let glob_vars = &glob_dir.dir_var;

        assert!(glob_vars.get("intOne").is_some());
        assert_eq!(glob_vars.get("intOne").unwrap().tipo, Tipo::Int);
        assert!(glob_vars.get("intTwo").is_some());
        assert_eq!(glob_vars.get("intTwo").unwrap().tipo, Tipo::Int);
        assert!(glob_vars.get("bool").is_some());
        assert_eq!(glob_vars.get("bool").unwrap().tipo, Tipo::Bool);
        assert!(glob_vars.get("float").is_some());
        assert_eq!(glob_vars.get("float").unwrap().tipo, Tipo::Float);

        assert!(glob_vars.get("arrInt").is_some());
        assert_eq!(glob_vars.get("arrInt").unwrap().tipo, Tipo::Int);
        assert!(glob_vars.get("arrFloat").is_some());
        assert_eq!(glob_vars.get("arrFloat").unwrap().tipo, Tipo::Float);
        assert!(glob_vars.get("arrBool").is_some());
        assert_eq!(glob_vars.get("arrBool").unwrap().tipo, Tipo::Bool);

        assert!(glob_vars.get("matInt").is_some());
        assert_eq!(glob_vars.get("matInt").unwrap().tipo, Tipo::Int);
        assert!(glob_vars.get("matFloat").is_some());
        assert_eq!(glob_vars.get("matFloat").unwrap().tipo, Tipo::Float);
        assert!(glob_vars.get("matBool").is_some());
        assert_eq!(glob_vars.get("matBool").unwrap().tipo, Tipo::Bool);

        // Assert functions without parameters
        assert!(evaluator.dir_func.get("fnVoid").is_some());
        assert_eq!(evaluator.dir_func.get("fnVoid").unwrap().tipo, Tipo::Void);

        assert!(evaluator.dir_func.get("fnBool").is_some());
        assert_eq!(evaluator.dir_func.get("fnBool").unwrap().tipo, Tipo::Bool);

        // Assert functions with parameters
        assert!(evaluator.dir_func.get("fnIntParams").is_some());
        let fn_dir = &evaluator.dir_func.get("fnIntParams").unwrap();
        assert_eq!(fn_dir.tipo, Tipo::Int);
        assert!(fn_dir.dir_var.get("pInt").is_some());
        assert_eq!(fn_dir.dir_var.get("pInt").unwrap().tipo, Tipo::Int);
        assert!(fn_dir.dir_var.get("pBool").is_some());
        assert_eq!(fn_dir.dir_var.get("pBool").unwrap().tipo, Tipo::Bool);
        assert!(fn_dir.dir_var.get("pFloat").is_some());
        assert_eq!(fn_dir.dir_var.get("pFloat").unwrap().tipo, Tipo::Float);
        assert!(fn_dir.dir_var.get("intOne").is_none());

        assert!(evaluator.dir_func.get("fnVoidParams").is_some());
        let fn_dir = &evaluator.dir_func.get("fnVoidParams").unwrap();
        assert_eq!(fn_dir.tipo, Tipo::Void);
        assert!(fn_dir.dir_var.get("arrInt").is_some());
        assert_eq!(fn_dir.dir_var.get("arrInt").unwrap().tipo, Tipo::Int);
        assert!(fn_dir.dir_var.get("arrBool").is_some());
        assert_eq!(fn_dir.dir_var.get("arrBool").unwrap().tipo, Tipo::Bool);
        assert!(fn_dir.dir_var.get("arrFloat").is_some());
        assert_eq!(fn_dir.dir_var.get("arrFloat").unwrap().tipo, Tipo::Float);
    }

    #[test]
    #[should_panic]
    fn test_dirs_fail_1() {
        let filename = "./tests/dirs_fail_1.tabby";
        let contents = fs::read_to_string(filename).unwrap();
        let res = tabby::PROGRAMParser::new().parse(&contents);
        assert!(res.is_ok());
        let my_ast = res.unwrap();
        let mut evaluator = AstEvaluator::new();
        evaluator.eval_program(my_ast);
    }

    #[test]
    #[should_panic]
    fn test_dirs_fail_2() {
        let filename = "./tests/dirs_fail_2.tabby";
        let contents = fs::read_to_string(filename).unwrap();
        let res = tabby::PROGRAMParser::new().parse(&contents);
        assert!(res.is_ok());
        let my_ast = res.unwrap();
        let mut evaluator = AstEvaluator::new();
        evaluator.eval_program(my_ast);
    }

    #[test]
    #[should_panic]
    fn test_dirs_fail_3() {
        let filename = "./tests/dirs_fail_3.tabby";
        let contents = fs::read_to_string(filename).unwrap();
        let res = tabby::PROGRAMParser::new().parse(&contents);
        assert!(res.is_ok());
        let my_ast = res.unwrap();
        let mut evaluator = AstEvaluator::new();
        evaluator.eval_program(my_ast);
    }

    #[test]
    #[should_panic]
    fn test_undeclared_var_fail_1() {
        let filename = "./tests/undeclared_var_fail_1.tabby";
        let contents = fs::read_to_string(filename).unwrap();
        let res = tabby::PROGRAMParser::new().parse(&contents);
        assert!(res.is_ok());
        let my_ast = res.unwrap();
        let mut evaluator = AstEvaluator::new();
        evaluator.eval_program(my_ast);
    }

    #[test]
    #[should_panic]
    fn test_undeclared_var_fail_2() {
        let filename = "./tests/undeclared_var_fail_2.tabby";
        let contents = fs::read_to_string(filename).unwrap();
        let res = tabby::PROGRAMParser::new().parse(&contents);
        assert!(res.is_ok());
        let my_ast = res.unwrap();
        let mut evaluator = AstEvaluator::new();
        evaluator.eval_program(my_ast);
    }

    #[test]
    #[should_panic]
    fn test_undeclared_var_fail_3() {
        let filename = "./tests/undeclared_var_fail_3.tabby";
        let contents = fs::read_to_string(filename).unwrap();
        let res = tabby::PROGRAMParser::new().parse(&contents);
        assert!(res.is_ok());
        let my_ast = res.unwrap();
        let mut evaluator = AstEvaluator::new();
        evaluator.eval_program(my_ast);
    }

    #[test]
    #[should_panic]
    fn test_undeclared_fn_fail_1() {
        let filename = "./tests/undeclared_fn_fail_1.tabby";
        let contents = fs::read_to_string(filename).unwrap();
        let res = tabby::PROGRAMParser::new().parse(&contents);
        assert!(res.is_ok());
        let my_ast = res.unwrap();
        let mut evaluator = AstEvaluator::new();
        evaluator.eval_program(my_ast);
    }

    #[test]
    fn test_quads_ok_1() {
        let filename = "./tests/quads_ok_1.tabby";
        let contents = fs::read_to_string(filename).unwrap();
        let res = tabby::PROGRAMParser::new().parse(&contents);
        assert!(res.is_ok());
        let my_ast = res.unwrap();
        let mut evaluator = AstEvaluator::new();
        evaluator.eval_program(my_ast);
        assert_eq!(evaluator.quads.len(), 29);
        assert_eq!(
            evaluator.quads.get(0).unwrap(),
            &Quadruple::Op(
                "+".to_string(),
                ("a".to_string(), 10000),
                ("a".to_string(), 10000),
                ("temp1".to_string(), 70000)
            )
        );
        assert_eq!(
            evaluator.quads.get(1).unwrap(),
            &Quadruple::Assign(
                "=".to_string(),
                ("temp1".to_string(), 70000),
                ("a".to_string(), 10000)
            )
        );
        assert_eq!(
            evaluator.quads.get(2).unwrap(),
            &Quadruple::Op(
                "-".to_string(),
                ("b".to_string(), 20000),
                ("a".to_string(), 10000),
                ("temp2".to_string(), 80000)
            )
        );
        assert_eq!(
            evaluator.quads.get(3).unwrap(),
            &Quadruple::Assign(
                "=".to_string(),
                ("temp2".to_string(), 80000),
                ("b".to_string(), 20000)
            )
        );
        assert_eq!(
            evaluator.quads.get(4).unwrap(),
            &Quadruple::Op(
                "*".to_string(),
                ("c".to_string(), 30000),
                ("a".to_string(), 10000),
                ("temp3".to_string(), 70001)
            )
        );
        assert_eq!(
            evaluator.quads.get(5).unwrap(),
            &Quadruple::Assign(
                "=".to_string(),
                ("temp3".to_string(), 70001),
                ("b".to_string(), 20000)
            )
        );
        assert_eq!(
            evaluator.quads.get(6).unwrap(),
            &Quadruple::Op(
                "/".to_string(),
                ("a".to_string(), 10000),
                ("c".to_string(), 30000),
                ("temp4".to_string(), 70002),
            )
        );
        assert_eq!(
            evaluator.quads.get(7).unwrap(),
            &Quadruple::Assign(
                "=".to_string(),
                ("temp4".to_string(), 70002),
                ("c".to_string(), 30000)
            )
        );
        assert_eq!(
            evaluator.quads.get(8).unwrap(),
            &Quadruple::Op(
                ">".to_string(),
                ("12".to_string(), 100000),
                ("5".to_string(), 100001),
                ("temp5".to_string(), 90000)
            )
        );
        assert_eq!(
            evaluator.quads.get(9).unwrap(),
            &Quadruple::Assign(
                "=".to_string(),
                ("temp5".to_string(), 90000),
                ("a".to_string(), 10000)
            )
        );
        assert_eq!(
            evaluator.quads.get(10).unwrap(),
            &Quadruple::Op(
                "==".to_string(),
                ("b".to_string(), 20000),
                ("b".to_string(), 20000),
                ("temp6".to_string(), 90001)
            )
        );
        assert_eq!(
            evaluator.quads.get(11).unwrap(),
            &Quadruple::Assign(
                "=".to_string(),
                ("temp6".to_string(), 90001),
                ("c".to_string(), 30000)
            )
        );
        assert_eq!(
            evaluator.quads.get(12).unwrap(),
            &Quadruple::Op(
                "And".to_string(),
                ("True".to_string(), 120000),
                ("False".to_string(), 120001),
                ("temp7".to_string(), 90002)
            )
        );
        assert_eq!(
            evaluator.quads.get(13).unwrap(),
            &Quadruple::Assign(
                "=".to_string(),
                ("temp7".to_string(), 90002),
                ("c".to_string(), 30000)
            )
        );
        assert_eq!(
            evaluator.quads.get(14).unwrap(),
            &Quadruple::Op(
                "Or".to_string(),
                ("False".to_string(), 120001),
                ("True".to_string(), 120000),
                ("temp8".to_string(), 90003)
            )
        );
        assert_eq!(
            evaluator.quads.get(15).unwrap(),
            &Quadruple::Assign(
                "=".to_string(),
                ("temp8".to_string(), 90003),
                ("a".to_string(), 10000)
            )
        );
        assert_eq!(
            evaluator.quads.get(16).unwrap(),
            &Quadruple::Assign(
                "=".to_string(),
                ("fn()".to_string(), -1),
                ("a".to_string(), 10000)
            )
        );
        assert_eq!(
            evaluator.quads.get(17).unwrap(),
            &Quadruple::Op(
                "*".to_string(),
                ("fn()".to_string(), -1),
                ("5".to_string(), 100001),
                ("temp9".to_string(), 70003)
            )
        );
        assert_eq!(
            evaluator.quads.get(18).unwrap(),
            &Quadruple::Assign(
                "=".to_string(),
                ("temp9".to_string(), 70003),
                ("b".to_string(), 20000)
            )
        );
        assert_eq!(
            evaluator.quads.get(19).unwrap(),
            &Quadruple::Read("Read".to_string(), ("a".to_string(), 10000))
        );
        assert_eq!(
            evaluator.quads.get(20).unwrap(),
            &Quadruple::Read("Read".to_string(), ("b".to_string(), 20000))
        );
        assert_eq!(
            evaluator.quads.get(21).unwrap(),
            &Quadruple::Op(
                "*".to_string(),
                ("True".to_string(), 120000),
                ("c".to_string(), 30000),
                ("temp10".to_string(), 70004)
            )
        );
        assert_eq!(
            evaluator.quads.get(22).unwrap(),
            &Quadruple::Op(
                "+".to_string(),
                ("a".to_string(), 10000),
                ("temp10".to_string(), 70004),
                ("temp11".to_string(), 70005)
            )
        );
        assert_eq!(
            evaluator.quads.get(23).unwrap(),
            &Quadruple::Print("Print".to_string(), ("temp11".to_string(), 70005))
        );
        assert_eq!(
            evaluator.quads.get(24).unwrap(),
            &Quadruple::Print("PrintSL".to_string(), ("\"Hello".to_string(), 130000))
        );
        assert_eq!(
            evaluator.quads.get(25).unwrap(),
            &Quadruple::Print("PrintSL".to_string(), ("\"wow!".to_string(), 130001))
        );
        assert_eq!(
            evaluator.quads.get(26).unwrap(),
            &Quadruple::Op(
                "*".to_string(),
                ("a".to_string(), 10000),
                ("b".to_string(), 20000),
                ("temp12".to_string(), 80001)
            )
        );
        assert_eq!(
            evaluator.quads.get(27).unwrap(),
            &Quadruple::Print("Print".to_string(), ("temp12".to_string(), 80001))
        );
        assert_eq!(
            evaluator.quads.get(28).unwrap(),
            &Quadruple::Print("PrintSL".to_string(), ("\"nope".to_string(), 130002))
        );
    }

    #[test]
    fn test_quads_ok_2() {
        let filename = "./tests/quads_ok_2.tabby";
        let contents = fs::read_to_string(filename).unwrap();
        let res = tabby::PROGRAMParser::new().parse(&contents);
        assert!(res.is_ok());
        let my_ast = res.unwrap();
        let mut evaluator = AstEvaluator::new();
        evaluator.eval_program(my_ast);
        assert_eq!(evaluator.quads.len(), 12);
        assert_eq!(
            evaluator.quads.get(0).unwrap(),
            &Quadruple::Op(
                "+".to_string(),
                ("2".to_string(), 100000),
                ("b".to_string(), 30000),
                ("temp1".to_string(), 70000)
            )
        );
        assert_eq!(
            evaluator.quads.get(1).unwrap(),
            &Quadruple::Op(
                "*".to_string(),
                ("temp1".to_string(), 70000),
                ("b".to_string(), 30000),
                ("temp2".to_string(), 70001)
            )
        );
        assert_eq!(
            evaluator.quads.get(2).unwrap(),
            &Quadruple::Op(
                "/".to_string(),
                ("i".to_string(), 10000),
                ("i".to_string(), 10000),
                ("temp3".to_string(), 70002)
            )
        );
        assert_eq!(
            evaluator.quads.get(3).unwrap(),
            &Quadruple::Op(
                "-".to_string(),
                ("temp2".to_string(), 70001),
                ("temp3".to_string(), 70002),
                ("temp4".to_string(), 70003)
            )
        );
        assert_eq!(
            evaluator.quads.get(4).unwrap(),
            &Quadruple::Op(
                "+".to_string(),
                ("temp4".to_string(), 70003),
                ("b".to_string(), 30000),
                ("temp5".to_string(), 70004)
            )
        );
        assert_eq!(
            evaluator.quads.get(5).unwrap(),
            &Quadruple::Op(
                ">".to_string(),
                ("temp5".to_string(), 70004),
                ("False".to_string(), 120000),
                ("temp6".to_string(), 90000)
            )
        );
        assert_eq!(
            evaluator.quads.get(6).unwrap(),
            &Quadruple::Op(
                "!=".to_string(),
                ("10".to_string(), 100001),
                ("i".to_string(), 10000),
                ("temp7".to_string(), 90001)
            )
        );
        assert_eq!(
            evaluator.quads.get(7).unwrap(),
            &Quadruple::Op(
                "Or".to_string(),
                ("temp7".to_string(), 90001),
                ("True".to_string(), 120001),
                ("temp8".to_string(), 90002)
            )
        );
        assert_eq!(
            evaluator.quads.get(8).unwrap(),
            &Quadruple::Op(
                "And".to_string(),
                ("temp6".to_string(), 90000),
                ("temp8".to_string(), 90002),
                ("temp9".to_string(), 90003)
            )
        );
        assert_eq!(
            evaluator.quads.get(9).unwrap(),
            &Quadruple::Op(
                "Or".to_string(),
                ("temp9".to_string(), 90003),
                ("b".to_string(), 30000),
                ("temp10".to_string(), 90004)
            )
        );
        assert_eq!(
            evaluator.quads.get(10).unwrap(),
            &Quadruple::Op(
                "Or".to_string(),
                ("temp10".to_string(), 90004),
                ("i".to_string(), 10000),
                ("temp11".to_string(), 90005)
            )
        );
        assert_eq!(
            evaluator.quads.get(11).unwrap(),
            &Quadruple::Assign(
                "=".to_string(),
                ("temp11".to_string(), 90005),
                ("res".to_string(), 30001)
            )
        );
    }

    #[test]
    #[should_panic]
    fn test_quads_fail_1() {
        let filename = "./tests/quads_fail_1.tabby";
        let contents = fs::read_to_string(filename).unwrap();
        let res = tabby::PROGRAMParser::new().parse(&contents);
        assert!(res.is_ok());
        let my_ast = res.unwrap();
        let mut evaluator = AstEvaluator::new();
        evaluator.eval_program(my_ast);
    }

    #[test]
    fn test_quads_call_ok_1() {
        let filename = "./tests/quads_call_ok_1.tabby";
        let contents = fs::read_to_string(filename).unwrap();
        let res = tabby::PROGRAMParser::new().parse(&contents);
        assert!(res.is_ok());
        let my_ast = res.unwrap();
        let mut evaluator = AstEvaluator::new();
        evaluator.eval_program(my_ast);
        assert!(evaluator.st_ops.is_empty());
        assert!(evaluator.st_vals.is_empty());
        assert!(evaluator.st_tips.is_empty());
    }

    #[test]
    fn test_quads_if_ok_1() {
        let filename = "./tests/quads_if_ok_1.tabby";
        let contents = fs::read_to_string(filename).unwrap();
        let res = tabby::PROGRAMParser::new().parse(&contents);
        assert!(res.is_ok());
        let my_ast = res.unwrap();
        let mut evaluator = AstEvaluator::new();
        evaluator.eval_program(my_ast);
        assert_eq!(evaluator.quads.len(), 9);
        assert_eq!(
            evaluator.quads.get(0).unwrap(),
            &Quadruple::Read("Read".to_string(), ("a".to_string(), 10000))
        );
        assert_eq!(
            evaluator.quads.get(1).unwrap(),
            &Quadruple::Op(
                "*".to_string(),
                ("12".to_string(), 100000),
                ("a".to_string(), 10000),
                ("temp1".to_string(), 70000)
            )
        );
        assert_eq!(
            evaluator.quads.get(2).unwrap(),
            &Quadruple::Op(
                ">".to_string(),
                ("temp1".to_string(), 70000),
                ("18".to_string(), 100001),
                ("temp2".to_string(), 90000)
            )
        );
        assert_eq!(
            evaluator.quads.get(3).unwrap(),
            &Quadruple::GoToF(("temp2".to_string(), 90000), 7)
        );
        assert_eq!(
            evaluator.quads.get(4).unwrap(),
            &Quadruple::Op(
                "*".to_string(),
                ("a".to_string(), 10000),
                ("12".to_string(), 100000),
                ("temp3".to_string(), 70001)
            )
        );
        assert_eq!(
            evaluator.quads.get(5).unwrap(),
            &Quadruple::Assign(
                "=".to_string(),
                ("temp3".to_string(), 70001),
                ("a".to_string(), 10000)
            )
        );
        assert_eq!(
            evaluator.quads.get(6).unwrap(),
            &Quadruple::Print("PrintSL".to_string(), ("\"If".to_string(), 130000))
        );
        assert_eq!(
            evaluator.quads.get(7).unwrap(),
            &Quadruple::Print("PrintSL".to_string(), ("\"End".to_string(), 130001))
        );
        assert_eq!(
            evaluator.quads.get(8).unwrap(),
            &Quadruple::Print("Print".to_string(), ("a".to_string(), 10000))
        );
    }

    #[test]
    fn test_quads_if_else_ok_1() {
        let filename = "./tests/quads_if_else_ok_1.tabby";
        let contents = fs::read_to_string(filename).unwrap();
        let res = tabby::PROGRAMParser::new().parse(&contents);
        assert!(res.is_ok());
        let my_ast = res.unwrap();
        let mut evaluator = AstEvaluator::new();
        evaluator.eval_program(my_ast);
        assert_eq!(evaluator.quads.len(), 18);
        assert_eq!(
            evaluator.quads.get(0).unwrap(),
            &Quadruple::Read("Read".to_string(), ("a".to_string(), 10000))
        );
        assert_eq!(
            evaluator.quads.get(1).unwrap(),
            &Quadruple::Read("Read".to_string(), ("b".to_string(), 10001))
        );
        assert_eq!(
            evaluator.quads.get(2).unwrap(),
            &Quadruple::Op(
                "*".to_string(),
                ("a".to_string(), 10000),
                ("3".to_string(), 100000),
                ("temp1".to_string(), 70000)
            )
        );
        assert_eq!(
            evaluator.quads.get(3).unwrap(),
            &Quadruple::Op(
                "*".to_string(),
                ("b".to_string(), 10001),
                ("5".to_string(), 100001),
                ("temp2".to_string(), 70001)
            )
        );
        assert_eq!(
            evaluator.quads.get(4).unwrap(),
            &Quadruple::Op(
                "!=".to_string(),
                ("temp1".to_string(), 70000),
                ("temp2".to_string(), 70001),
                ("temp3".to_string(), 90000)
            )
        );
        assert_eq!(
            evaluator.quads.get(5).unwrap(),
            &Quadruple::GoToF(("temp3".to_string(), 90000), 10)
        );
        assert_eq!(
            evaluator.quads.get(6).unwrap(),
            &Quadruple::Op(
                "+".to_string(),
                ("a".to_string(), 10000),
                ("1".to_string(), 100002),
                ("temp4".to_string(), 70002)
            )
        );
        assert_eq!(
            evaluator.quads.get(7).unwrap(),
            &Quadruple::Assign(
                "=".to_string(),
                ("temp4".to_string(), 70002),
                ("a".to_string(), 10000)
            )
        );
        assert_eq!(
            evaluator.quads.get(8).unwrap(),
            &Quadruple::Print("Print".to_string(), ("a".to_string(), 10000))
        );
        assert_eq!(evaluator.quads.get(9).unwrap(), &Quadruple::GoTo(14));
        assert_eq!(
            evaluator.quads.get(10).unwrap(),
            &Quadruple::Op(
                "+".to_string(),
                ("a".to_string(), 10000),
                ("b".to_string(), 10001),
                ("temp5".to_string(), 70003)
            )
        );
        assert_eq!(
            evaluator.quads.get(11).unwrap(),
            &Quadruple::Op(
                "+".to_string(),
                ("temp5".to_string(), 70003),
                ("1".to_string(), 100002),
                ("temp6".to_string(), 70004)
            )
        );
        assert_eq!(
            evaluator.quads.get(12).unwrap(),
            &Quadruple::Assign(
                "=".to_string(),
                ("temp6".to_string(), 70004),
                ("b".to_string(), 10001)
            )
        );
        assert_eq!(
            evaluator.quads.get(13).unwrap(),
            &Quadruple::Print("Print".to_string(), ("b".to_string(), 10001))
        );
        assert_eq!(
            evaluator.quads.get(14).unwrap(),
            &Quadruple::Op(
                "*".to_string(),
                ("a".to_string(), 10000),
                ("b".to_string(), 10001),
                ("temp7".to_string(), 70005)
            )
        );
        assert_eq!(
            evaluator.quads.get(15).unwrap(),
            &Quadruple::Op(
                "+".to_string(),
                ("temp7".to_string(), 70005),
                ("2".to_string(), 100003),
                ("temp8".to_string(), 70006)
            )
        );
        assert_eq!(
            evaluator.quads.get(16).unwrap(),
            &Quadruple::Assign(
                "=".to_string(),
                ("temp8".to_string(), 70006),
                ("a".to_string(), 10000)
            )
        );
        assert_eq!(
            evaluator.quads.get(17).unwrap(),
            &Quadruple::Print("Print".to_string(), ("a".to_string(), 10000))
        );
    }

    #[test]
    #[should_panic]
    fn test_if_fail_1() {
        let filename = "./tests/if_fail_1.tabby";
        let contents = fs::read_to_string(filename).unwrap();
        let res = tabby::PROGRAMParser::new().parse(&contents);
        assert!(res.is_ok());
        let my_ast = res.unwrap();
        let mut evaluator = AstEvaluator::new();
        evaluator.eval_program(my_ast);
    }

    #[test]
    fn test_vir_mem_ok_1() {
        let filename = "./tests/vir_mem_ok_1.tabby";
        let contents = fs::read_to_string(filename).unwrap();
        let res = tabby::PROGRAMParser::new().parse(&contents);
        assert!(res.is_ok());
        let my_ast = res.unwrap();
        let mut evaluator = AstEvaluator::new();
        evaluator.eval_program(my_ast);
        // Test vir mem addresses
        // Globals
        let global_map = &evaluator.dir_func.get("testVirMemAlloc").unwrap().dir_var;
        assert_eq!(global_map.get("a").unwrap().addr, 10000);
        assert_eq!(global_map.get("b").unwrap().addr, 10001);
        assert_eq!(global_map.get("c").unwrap().addr, 10002);
        assert_eq!(global_map.get("d").unwrap().addr, 20000);
        assert_eq!(global_map.get("e").unwrap().addr, 20001);
        assert_eq!(global_map.get("f").unwrap().addr, 20002);
        assert_eq!(global_map.get("g").unwrap().addr, 30000);
        assert_eq!(global_map.get("h").unwrap().addr, 30001);
        assert_eq!(global_map.get("i").unwrap().addr, 30002);
        // Locals
        let fn_one_map = &evaluator.dir_func.get("fnOne").unwrap().dir_var;
        assert_eq!(fn_one_map.get("a").unwrap().addr, 40000);
        assert_eq!(fn_one_map.get("b").unwrap().addr, 50000);
        assert_eq!(fn_one_map.get("c").unwrap().addr, 60000);
        let fn_two_map = &evaluator.dir_func.get("fnOne").unwrap().dir_var;
        assert_eq!(fn_two_map.get("a").unwrap().addr, 40000);
        assert_eq!(fn_two_map.get("b").unwrap().addr, 50000);
        assert_eq!(fn_two_map.get("c").unwrap().addr, 60000);
        // Test Quads
        assert_eq!(evaluator.quads.len(), 10);
        assert_eq!(
            evaluator.quads.get(0).unwrap(),
            &Quadruple::Op(
                "*".to_string(),
                ("b".to_string(), 10001),
                ("c".to_string(), 10002),
                ("temp1".to_string(), 70000)
            )
        );
        assert_eq!(
            evaluator.quads.get(1).unwrap(),
            &Quadruple::Op(
                "*".to_string(),
                ("temp1".to_string(), 70000),
                ("1".to_string(), 100000),
                ("temp2".to_string(), 70001)
            )
        );
        assert_eq!(
            evaluator.quads.get(2).unwrap(),
            &Quadruple::Assign(
                "=".to_string(),
                ("temp2".to_string(), 70001),
                ("a".to_string(), 10000)
            )
        );
        assert_eq!(
            evaluator.quads.get(3).unwrap(),
            &Quadruple::Op(
                "/".to_string(),
                ("e".to_string(), 20001),
                ("f".to_string(), 20002),
                ("temp3".to_string(), 80000)
            )
        );
        assert_eq!(
            evaluator.quads.get(4).unwrap(),
            &Quadruple::Op(
                "*".to_string(),
                ("temp3".to_string(), 80000),
                ("1.2".to_string(), 110000),
                ("temp4".to_string(), 80001)
            )
        );
        assert_eq!(
            evaluator.quads.get(5).unwrap(),
            &Quadruple::Assign(
                "=".to_string(),
                ("temp4".to_string(), 80001),
                ("d".to_string(), 20000)
            )
        );
        assert_eq!(
            evaluator.quads.get(6).unwrap(),
            &Quadruple::Op(
                "And".to_string(),
                ("h".to_string(), 30001),
                ("i".to_string(), 30002),
                ("temp5".to_string(), 90000)
            )
        );
        assert_eq!(
            evaluator.quads.get(7).unwrap(),
            &Quadruple::Op(
                "Or".to_string(),
                ("temp5".to_string(), 90000),
                ("True".to_string(), 120000),
                ("temp6".to_string(), 90001)
            )
        );
        assert_eq!(
            evaluator.quads.get(8).unwrap(),
            &Quadruple::Assign(
                "=".to_string(),
                ("temp6".to_string(), 90001),
                ("g".to_string(), 30000)
            )
        );
        assert_eq!(
            evaluator.quads.get(9).unwrap(),
            &Quadruple::Print("PrintSL".to_string(), ("\"Wow".to_string(), 130000))
        );
    }

    #[test]
    fn test_quads_while_ok_1() {
        let filename = "./tests/quads_while_ok_1.tabby";
        let contents = fs::read_to_string(filename).unwrap();
        let res = tabby::PROGRAMParser::new().parse(&contents);
        assert!(res.is_ok());
        let my_ast = res.unwrap();
        let mut evaluator = AstEvaluator::new();
        evaluator.eval_program(my_ast);
        assert_eq!(evaluator.quads.len(), 11);
        assert_eq!(
            evaluator.quads.get(0).unwrap(),
            &Quadruple::Read("Read".to_string(), ("a".to_string(), 10000))
        );
        assert_eq!(
            evaluator.quads.get(1).unwrap(),
            &Quadruple::Op(
                "+".to_string(),
                ("a".to_string(), 10000),
                ("5".to_string(), 100000),
                ("temp1".to_string(), 70000)
            )
        );
        assert_eq!(
            evaluator.quads.get(2).unwrap(),
            &Quadruple::Op(
                "*".to_string(),
                ("10".to_string(), 100001),
                ("2".to_string(), 100002),
                ("temp2".to_string(), 70001)
            )
        );
        assert_eq!(
            evaluator.quads.get(3).unwrap(),
            &Quadruple::Op(
                "<".to_string(),
                ("temp1".to_string(), 70000),
                ("temp2".to_string(), 70001),
                ("temp3".to_string(), 90000)
            )
        );
        assert_eq!(
            evaluator.quads.get(4).unwrap(),
            &Quadruple::GoToF(("temp3".to_string(), 90000), 9)
        );
        assert_eq!(
            evaluator.quads.get(5).unwrap(),
            &Quadruple::Print("Print".to_string(), ("a".to_string(), 10000))
        );
        assert_eq!(
            evaluator.quads.get(6).unwrap(),
            &Quadruple::Op(
                "+".to_string(),
                ("a".to_string(), 10000),
                ("1".to_string(), 100003),
                ("temp4".to_string(), 70002)
            )
        );
        assert_eq!(
            evaluator.quads.get(7).unwrap(),
            &Quadruple::Assign(
                "=".to_string(),
                ("temp4".to_string(), 70002),
                ("a".to_string(), 10000)
            )
        );
        assert_eq!(evaluator.quads.get(8).unwrap(), &Quadruple::GoTo(1));
        assert_eq!(
            evaluator.quads.get(9).unwrap(),
            &Quadruple::Print("PrintSL".to_string(), ("\"Finish".to_string(), 130000))
        );
        assert_eq!(
            evaluator.quads.get(10).unwrap(),
            &Quadruple::Print("Print".to_string(), ("a".to_string(), 10000))
        );
    }

    #[test]
    fn test_quads_for_ok_1() {
        let filename = "./tests/quads_for_ok_1.tabby";
        let contents = fs::read_to_string(filename).unwrap();
        let res = tabby::PROGRAMParser::new().parse(&contents);
        assert!(res.is_ok());
        let my_ast = res.unwrap();
        let mut evaluator = AstEvaluator::new();
        evaluator.eval_program(my_ast);
        assert_eq!(evaluator.quads.len(), 10);
        assert_eq!(
            evaluator.quads.get(0).unwrap(),
            &Quadruple::Read("Read".to_string(), ("i".to_string(), 10000))
        );
        assert_eq!(
            evaluator.quads.get(1).unwrap(),
            &Quadruple::Op(
                "+".to_string(),
                ("i".to_string(), 10000),
                ("1".to_string(), 100000),
                ("temp1".to_string(), 70000)
            )
        );
        assert_eq!(
            evaluator.quads.get(2).unwrap(),
            &Quadruple::Op(
                ">".to_string(),
                ("temp1".to_string(), 70000),
                ("5".to_string(), 100001),
                ("temp2".to_string(), 90000)
            )
        );
        assert_eq!(
            evaluator.quads.get(3).unwrap(),
            &Quadruple::GoToF(("temp2".to_string(), 90000), 9)
        );
        assert_eq!(
            evaluator.quads.get(4).unwrap(),
            &Quadruple::Print("PrintSL".to_string(), ("\"Loop".to_string(), 130000))
        );
        assert_eq!(
            evaluator.quads.get(5).unwrap(),
            &Quadruple::Print("Print".to_string(), ("i".to_string(), 10000))
        );
        assert_eq!(
            evaluator.quads.get(6).unwrap(),
            &Quadruple::Op(
                "*".to_string(),
                ("2".to_string(), 100002),
                ("i".to_string(), 10000),
                ("temp3".to_string(), 70001)
            )
        );
        assert_eq!(
            evaluator.quads.get(7).unwrap(),
            &Quadruple::Assign(
                "=".to_string(),
                ("temp3".to_string(), 70001),
                ("i".to_string(), 10000)
            )
        );
        assert_eq!(evaluator.quads.get(8).unwrap(), &Quadruple::GoTo(1));
        assert_eq!(
            evaluator.quads.get(9).unwrap(),
            &Quadruple::Print("PrintSL".to_string(), ("\"Finish".to_string(), 130001))
        );
    }

    #[test]
    fn test_quads_non_linear_ok_1() {
        let filename = "./tests/quads_non_linear_ok_1.tabby";
        let contents = fs::read_to_string(filename).unwrap();
        let res = tabby::PROGRAMParser::new().parse(&contents);
        assert!(res.is_ok());
        let my_ast = res.unwrap();
        let mut evaluator = AstEvaluator::new();
        evaluator.eval_program(my_ast);
        assert_eq!(evaluator.quads.len(), 24);
        assert_eq!(
            evaluator.quads.get(0).unwrap(),
            &Quadruple::Print("PrintSL".to_string(), ("\"ProgramStt".to_string(), 130000))
        );
        assert_eq!(
            evaluator.quads.get(1).unwrap(),
            &Quadruple::Op(
                "==".to_string(),
                ("i".to_string(), 30000),
                ("True".to_string(), 120000),
                ("temp1".to_string(), 90000)
            )
        );
        assert_eq!(
            evaluator.quads.get(2).unwrap(),
            &Quadruple::GoToF(("temp1".to_string(), 90000), 23)
        );
        assert_eq!(
            evaluator.quads.get(3).unwrap(),
            &Quadruple::Print("PrintSL".to_string(), ("\"ForStt".to_string(), 130001))
        );
        assert_eq!(
            evaluator.quads.get(4).unwrap(),
            &Quadruple::Op(
                "==".to_string(),
                ("i".to_string(), 30000),
                ("True".to_string(), 120000),
                ("temp2".to_string(), 90001)
            )
        );
        assert_eq!(
            evaluator.quads.get(5).unwrap(),
            &Quadruple::GoToF(("temp2".to_string(), 90001), 14)
        );
        assert_eq!(
            evaluator.quads.get(6).unwrap(),
            &Quadruple::Print("PrintSL".to_string(), ("\"IfStt".to_string(), 130002))
        );
        assert_eq!(
            evaluator.quads.get(7).unwrap(),
            &Quadruple::Op(
                "==".to_string(),
                ("i".to_string(), 30000),
                ("True".to_string(), 120000),
                ("temp3".to_string(), 90002)
            )
        );
        assert_eq!(
            evaluator.quads.get(8).unwrap(),
            &Quadruple::GoToF(("temp3".to_string(), 90002), 11)
        );
        assert_eq!(
            evaluator.quads.get(9).unwrap(),
            &Quadruple::Print("PrintSL".to_string(), ("\"While".to_string(), 130003))
        );
        assert_eq!(evaluator.quads.get(10).unwrap(), &Quadruple::GoTo(7));
        assert_eq!(
            evaluator.quads.get(11).unwrap(),
            &Quadruple::Print("PrintSL".to_string(), ("\"ExitWhile".to_string(), 130004))
        );
        assert_eq!(
            evaluator.quads.get(12).unwrap(),
            &Quadruple::Print("PrintSL".to_string(), ("\"IfEnd".to_string(), 130005))
        );
        assert_eq!(evaluator.quads.get(13).unwrap(), &Quadruple::GoTo(20));
        assert_eq!(
            evaluator.quads.get(14).unwrap(),
            &Quadruple::Print("PrintSL".to_string(), ("\"ElseStt".to_string(), 130006))
        );
        assert_eq!(
            evaluator.quads.get(15).unwrap(),
            &Quadruple::Op(
                "==".to_string(),
                ("i".to_string(), 30000),
                ("False".to_string(), 120001),
                ("temp4".to_string(), 90003)
            )
        );
        assert_eq!(
            evaluator.quads.get(16).unwrap(),
            &Quadruple::GoToF(("temp4".to_string(), 90003), 18)
        );
        assert_eq!(
            evaluator.quads.get(17).unwrap(),
            &Quadruple::Print("PrintSL".to_string(), ("\"If2".to_string(), 130007))
        );
        assert_eq!(
            evaluator.quads.get(18).unwrap(),
            &Quadruple::Print("PrintSL".to_string(), ("\"ExitIf2".to_string(), 130008))
        );
        assert_eq!(
            evaluator.quads.get(19).unwrap(),
            &Quadruple::Print("PrintSL".to_string(), ("\"ElseEnd".to_string(), 130009))
        );
        assert_eq!(
            evaluator.quads.get(20).unwrap(),
            &Quadruple::Print("PrintSL".to_string(), ("\"ForEnd".to_string(), 130010))
        );
        assert_eq!(
            evaluator.quads.get(21).unwrap(),
            &Quadruple::Assign(
                "=".to_string(),
                ("True".to_string(), 120000),
                ("i".to_string(), 30000)
            )
        );
        assert_eq!(evaluator.quads.get(22).unwrap(), &Quadruple::GoTo(1));
        assert_eq!(
            evaluator.quads.get(23).unwrap(),
            &Quadruple::Print("PrintSL".to_string(), ("\"ProgramEnd".to_string(), 130011))
        );
    }
}
