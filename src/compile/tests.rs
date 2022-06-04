#[cfg(test)]
mod tests_compile {
    use crate::ast_evaluator::AstEvaluator;
    use crate::quadruples::Quadruple;
    use crate::tabby;
    use crate::tipo::Tipo;
    use crate::vir_mem::*;
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
    C
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
        let filename = "./tests_compile/dirs_ok_1.tabby";
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
        assert!(fn_dir.dir_var.get("int").is_some());
        assert_eq!(fn_dir.dir_var.get("int").unwrap().tipo, Tipo::Int);
        assert!(fn_dir.dir_var.get("bool").is_some());
        assert_eq!(fn_dir.dir_var.get("bool").unwrap().tipo, Tipo::Bool);
        assert!(fn_dir.dir_var.get("float").is_some());
        assert_eq!(fn_dir.dir_var.get("float").unwrap().tipo, Tipo::Float);
    }

    #[test]
    #[should_panic]
    fn test_dirs_fail_1() {
        let filename = "./tests_compile/dirs_fail_1.tabby";
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
        let filename = "./tests_compile/dirs_fail_2.tabby";
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
        let filename = "./tests_compile/dirs_fail_3.tabby";
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
        let filename = "./tests_compile/undeclared_var_fail_1.tabby";
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
        let filename = "./tests_compile/undeclared_var_fail_2.tabby";
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
        let filename = "./tests_compile/undeclared_var_fail_3.tabby";
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
        let filename = "./tests_compile/undeclared_fn_fail_1.tabby";
        let contents = fs::read_to_string(filename).unwrap();
        let res = tabby::PROGRAMParser::new().parse(&contents);
        assert!(res.is_ok());
        let my_ast = res.unwrap();
        let mut evaluator = AstEvaluator::new();
        evaluator.eval_program(my_ast);
    }

    #[test]
    fn test_quads_ok_1() {
        let filename = "./tests_compile/quads_ok_1.tabby";
        let contents = fs::read_to_string(filename).unwrap();
        let res = tabby::PROGRAMParser::new().parse(&contents);
        assert!(res.is_ok());
        let my_ast = res.unwrap();
        let mut evaluator = AstEvaluator::new();
        evaluator.eval_program(my_ast);
        assert_eq!(evaluator.quads.len(), 38);
        assert_eq!(evaluator.quads.get(0).unwrap(), &Quadruple::GoTo(3));
        assert_eq!(
            evaluator.quads.get(1).unwrap(),
            &Quadruple::Assign(
                "=".to_string(),
                ("0".to_string(), (CNST_START_INT)),
                ("Ret".to_string(), (GTEMP_START_INT))
            )
        );
        assert_eq!(evaluator.quads.get(2).unwrap(), &Quadruple::EndFunc());
        assert_eq!(
            evaluator.quads.get(3).unwrap(),
            &Quadruple::Op(
                "+".to_string(),
                ("a".to_string(), (GLOBAL_START + GLOBAL_INT_OFFSET)),
                ("a".to_string(), (GLOBAL_START + GLOBAL_INT_OFFSET)),
                ("temp1".to_string(), (LTEMP_START + LOCAL_INT_OFFSET))
            )
        );
        assert_eq!(
            evaluator.quads.get(4).unwrap(),
            &Quadruple::Assign(
                "=".to_string(),
                ("temp1".to_string(), (LTEMP_START + LOCAL_INT_OFFSET)),
                ("a".to_string(), (GLOBAL_START + GLOBAL_INT_OFFSET))
            )
        );
        assert_eq!(
            evaluator.quads.get(5).unwrap(),
            &Quadruple::Op(
                "-".to_string(),
                ("b".to_string(), (GLOBAL_START + GLOBAL_FLOAT_OFFSET)),
                ("a".to_string(), (GLOBAL_START + GLOBAL_INT_OFFSET)),
                ("temp2".to_string(), (LTEMP_START + LOCAL_FLOAT_OFFSET))
            )
        );
        assert_eq!(
            evaluator.quads.get(6).unwrap(),
            &Quadruple::Assign(
                "=".to_string(),
                ("temp2".to_string(), (LTEMP_START + LOCAL_FLOAT_OFFSET)),
                ("b".to_string(), (GLOBAL_START + GLOBAL_FLOAT_OFFSET))
            )
        );
        assert_eq!(
            evaluator.quads.get(7).unwrap(),
            &Quadruple::Op(
                "*".to_string(),
                ("c".to_string(), (GLOBAL_START + GLOBAL_BOOL_OFFSET)),
                ("a".to_string(), (GLOBAL_START + GLOBAL_INT_OFFSET)),
                ("temp3".to_string(), (LTEMP_START + LOCAL_INT_OFFSET + 1))
            )
        );
        assert_eq!(
            evaluator.quads.get(8).unwrap(),
            &Quadruple::Assign(
                "=".to_string(),
                ("temp3".to_string(), (LTEMP_START + LOCAL_INT_OFFSET + 1)),
                ("b".to_string(), (GLOBAL_START + GLOBAL_FLOAT_OFFSET))
            )
        );
        assert_eq!(
            evaluator.quads.get(9).unwrap(),
            &Quadruple::Op(
                "/".to_string(),
                ("a".to_string(), (GLOBAL_START + GLOBAL_INT_OFFSET)),
                ("c".to_string(), (GLOBAL_START + GLOBAL_BOOL_OFFSET)),
                ("temp4".to_string(), (LTEMP_START + LOCAL_INT_OFFSET + 2)),
            )
        );
        assert_eq!(
            evaluator.quads.get(10).unwrap(),
            &Quadruple::Assign(
                "=".to_string(),
                ("temp4".to_string(), (LTEMP_START + LOCAL_INT_OFFSET + 2)),
                ("c".to_string(), (GLOBAL_START + GLOBAL_BOOL_OFFSET))
            )
        );
        assert_eq!(
            evaluator.quads.get(11).unwrap(),
            &Quadruple::Op(
                ">".to_string(),
                ("12".to_string(), (CNST_START + LOCAL_INT_OFFSET + 1)),
                ("5".to_string(), (CNST_START + LOCAL_INT_OFFSET + 2)),
                ("temp5".to_string(), (LTEMP_START + LOCAL_BOOL_OFFSET))
            )
        );
        assert_eq!(
            evaluator.quads.get(12).unwrap(),
            &Quadruple::Assign(
                "=".to_string(),
                ("temp5".to_string(), (LTEMP_START + LOCAL_BOOL_OFFSET)),
                ("a".to_string(), (GLOBAL_START + GLOBAL_INT_OFFSET))
            )
        );
        assert_eq!(
            evaluator.quads.get(13).unwrap(),
            &Quadruple::Op(
                "==".to_string(),
                ("b".to_string(), (GLOBAL_START + GLOBAL_FLOAT_OFFSET)),
                ("b".to_string(), (GLOBAL_START + GLOBAL_FLOAT_OFFSET)),
                ("temp6".to_string(), (LTEMP_START + LOCAL_BOOL_OFFSET + 1))
            )
        );
        assert_eq!(
            evaluator.quads.get(14).unwrap(),
            &Quadruple::Assign(
                "=".to_string(),
                ("temp6".to_string(), (LTEMP_START + LOCAL_BOOL_OFFSET + 1)),
                ("c".to_string(), (GLOBAL_START + GLOBAL_BOOL_OFFSET))
            )
        );
        assert_eq!(
            evaluator.quads.get(15).unwrap(),
            &Quadruple::Op(
                "And".to_string(),
                ("True".to_string(), (CNST_START + LOCAL_BOOL_OFFSET)),
                ("False".to_string(), (CNST_START + LOCAL_BOOL_OFFSET + 1)),
                ("temp7".to_string(), (LTEMP_START + LOCAL_BOOL_OFFSET + 2))
            )
        );
        assert_eq!(
            evaluator.quads.get(16).unwrap(),
            &Quadruple::Assign(
                "=".to_string(),
                ("temp7".to_string(), (LTEMP_START + LOCAL_BOOL_OFFSET + 2)),
                ("c".to_string(), (GLOBAL_START + GLOBAL_BOOL_OFFSET))
            )
        );
        assert_eq!(
            evaluator.quads.get(17).unwrap(),
            &Quadruple::Op(
                "Or".to_string(),
                ("False".to_string(), (CNST_START + LOCAL_BOOL_OFFSET + 1)),
                ("True".to_string(), (CNST_START + LOCAL_BOOL_OFFSET)),
                ("temp8".to_string(), (LTEMP_START + LOCAL_BOOL_OFFSET + 3))
            )
        );
        assert_eq!(
            evaluator.quads.get(18).unwrap(),
            &Quadruple::Assign(
                "=".to_string(),
                ("temp8".to_string(), (LTEMP_START + LOCAL_BOOL_OFFSET + 3)),
                ("a".to_string(), (GLOBAL_START + GLOBAL_INT_OFFSET))
            )
        );
        assert_eq!(
            evaluator.quads.get(19).unwrap(),
            &Quadruple::Era("fn".to_string())
        );
        assert_eq!(
            evaluator.quads.get(20).unwrap(),
            &Quadruple::GoSub("fn".to_string(), 1)
        );
        assert_eq!(
            evaluator.quads.get(21).unwrap(),
            &Quadruple::Assign(
                "=".to_string(),
                ("Ret".to_string(), GTEMP_START),
                ("temp9".to_string(), LTEMP_START + LOCAL_INT_OFFSET + 3)
            )
        );
        assert_eq!(
            evaluator.quads.get(22).unwrap(),
            &Quadruple::Assign(
                "=".to_string(),
                ("temp9".to_string(), (LTEMP_START + LOCAL_INT_OFFSET + 3)),
                ("a".to_string(), (GLOBAL_START + GLOBAL_INT_OFFSET))
            )
        );
        assert_eq!(
            evaluator.quads.get(23).unwrap(),
            &Quadruple::Era("fn".to_string())
        );
        assert_eq!(
            evaluator.quads.get(24).unwrap(),
            &Quadruple::GoSub("fn".to_string(), 1)
        );
        assert_eq!(
            evaluator.quads.get(25).unwrap(),
            &Quadruple::Assign(
                "=".to_string(),
                ("Ret".to_string(), (GTEMP_START)),
                ("temp10".to_string(), (LTEMP_START + LOCAL_INT_OFFSET + 4))
            )
        );
        assert_eq!(
            evaluator.quads.get(26).unwrap(),
            &Quadruple::Op(
                "*".to_string(),
                ("temp10".to_string(), (LTEMP_START + LOCAL_INT_OFFSET + 4)),
                ("5".to_string(), (CNST_START + LOCAL_INT_OFFSET + 2)),
                ("temp11".to_string(), (LTEMP_START + LOCAL_INT_OFFSET + 5))
            )
        );
        assert_eq!(
            evaluator.quads.get(27).unwrap(),
            &Quadruple::Assign(
                "=".to_string(),
                ("temp11".to_string(), (LTEMP_START + LOCAL_INT_OFFSET + 5)),
                ("b".to_string(), (GLOBAL_START + GLOBAL_FLOAT_OFFSET))
            )
        );
        assert_eq!(
            evaluator.quads.get(28).unwrap(),
            &Quadruple::Read(
                "Read".to_string(),
                ("a".to_string(), (GLOBAL_START + GLOBAL_INT_OFFSET))
            )
        );
        assert_eq!(
            evaluator.quads.get(29).unwrap(),
            &Quadruple::Read(
                "Read".to_string(),
                ("b".to_string(), (GLOBAL_START + GLOBAL_FLOAT_OFFSET))
            )
        );
        assert_eq!(
            evaluator.quads.get(30).unwrap(),
            &Quadruple::Op(
                "*".to_string(),
                ("True".to_string(), (CNST_START + LOCAL_BOOL_OFFSET)),
                ("c".to_string(), (GLOBAL_START + GLOBAL_BOOL_OFFSET)),
                ("temp12".to_string(), (LTEMP_START + LOCAL_INT_OFFSET + 6))
            )
        );
        assert_eq!(
            evaluator.quads.get(31).unwrap(),
            &Quadruple::Op(
                "+".to_string(),
                ("a".to_string(), (GLOBAL_START + GLOBAL_INT_OFFSET)),
                ("temp12".to_string(), (LTEMP_START + LOCAL_INT_OFFSET + 6)),
                ("temp13".to_string(), (LTEMP_START + LOCAL_INT_OFFSET + 7))
            )
        );
        assert_eq!(
            evaluator.quads.get(32).unwrap(),
            &Quadruple::Print(
                "Print".to_string(),
                ("temp13".to_string(), (LTEMP_START + LOCAL_INT_OFFSET + 7))
            )
        );
        assert_eq!(
            evaluator.quads.get(33).unwrap(),
            &Quadruple::Print(
                "PrintSL".to_string(),
                ("\"Hello".to_string(), (CNST_START + STRLIT_OFFSET))
            )
        );
        assert_eq!(
            evaluator.quads.get(34).unwrap(),
            &Quadruple::Print(
                "PrintSL".to_string(),
                ("\"wow!".to_string(), (CNST_START + STRLIT_OFFSET + 1))
            )
        );
        assert_eq!(
            evaluator.quads.get(35).unwrap(),
            &Quadruple::Op(
                "*".to_string(),
                ("a".to_string(), (GLOBAL_START + GLOBAL_INT_OFFSET)),
                ("b".to_string(), (GLOBAL_START + GLOBAL_FLOAT_OFFSET)),
                ("temp14".to_string(), (LTEMP_START + LOCAL_FLOAT_OFFSET + 1))
            )
        );
        assert_eq!(
            evaluator.quads.get(36).unwrap(),
            &Quadruple::Print(
                "Print".to_string(),
                ("temp14".to_string(), (LTEMP_START + LOCAL_FLOAT_OFFSET + 1))
            )
        );
        assert_eq!(
            evaluator.quads.get(37).unwrap(),
            &Quadruple::Print(
                "PrintSL".to_string(),
                ("\"nope".to_string(), (CNST_START + STRLIT_OFFSET + 2))
            )
        );
    }

    #[test]
    fn test_quads_ok_2() {
        let filename = "./tests_compile/quads_ok_2.tabby";
        let contents = fs::read_to_string(filename).unwrap();
        let res = tabby::PROGRAMParser::new().parse(&contents);
        assert!(res.is_ok());
        let my_ast = res.unwrap();
        let mut evaluator = AstEvaluator::new();
        evaluator.eval_program(my_ast);
        assert_eq!(evaluator.quads.len(), 13);
        assert_eq!(evaluator.quads.get(0).unwrap(), &Quadruple::GoTo(1));
        assert_eq!(
            evaluator.quads.get(1).unwrap(),
            &Quadruple::Op(
                "+".to_string(),
                ("2".to_string(), (CNST_START + LOCAL_INT_OFFSET)),
                ("b".to_string(), (GLOBAL_START + GLOBAL_BOOL_OFFSET)),
                ("temp1".to_string(), (LTEMP_START + LOCAL_INT_OFFSET))
            )
        );
        assert_eq!(
            evaluator.quads.get(2).unwrap(),
            &Quadruple::Op(
                "*".to_string(),
                ("temp1".to_string(), (LTEMP_START + LOCAL_INT_OFFSET)),
                ("b".to_string(), (GLOBAL_START + GLOBAL_BOOL_OFFSET)),
                ("temp2".to_string(), (LTEMP_START + LOCAL_INT_OFFSET + 1))
            )
        );
        assert_eq!(
            evaluator.quads.get(3).unwrap(),
            &Quadruple::Op(
                "/".to_string(),
                ("i".to_string(), (GLOBAL_START + GLOBAL_INT_OFFSET)),
                ("i".to_string(), (GLOBAL_START + GLOBAL_INT_OFFSET)),
                ("temp3".to_string(), (LTEMP_START + LOCAL_INT_OFFSET + 2))
            )
        );
        assert_eq!(
            evaluator.quads.get(4).unwrap(),
            &Quadruple::Op(
                "-".to_string(),
                ("temp2".to_string(), (LTEMP_START + LOCAL_INT_OFFSET + 1)),
                ("temp3".to_string(), (LTEMP_START + LOCAL_INT_OFFSET + 2)),
                ("temp4".to_string(), (LTEMP_START + LOCAL_INT_OFFSET + 3))
            )
        );
        assert_eq!(
            evaluator.quads.get(5).unwrap(),
            &Quadruple::Op(
                "+".to_string(),
                ("temp4".to_string(), (LTEMP_START + LOCAL_INT_OFFSET + 3)),
                ("b".to_string(), (GLOBAL_START + GLOBAL_BOOL_OFFSET)),
                ("temp5".to_string(), (LTEMP_START + LOCAL_INT_OFFSET + 4))
            )
        );
        assert_eq!(
            evaluator.quads.get(6).unwrap(),
            &Quadruple::Op(
                ">".to_string(),
                ("temp5".to_string(), (LTEMP_START + LOCAL_INT_OFFSET + 4)),
                ("False".to_string(), (CNST_START + LOCAL_BOOL_OFFSET)),
                ("temp6".to_string(), (LTEMP_START + LOCAL_BOOL_OFFSET))
            )
        );
        assert_eq!(
            evaluator.quads.get(7).unwrap(),
            &Quadruple::Op(
                "!=".to_string(),
                ("10".to_string(), (CNST_START + LOCAL_INT_OFFSET + 1)),
                ("i".to_string(), (GLOBAL_START + GLOBAL_INT_OFFSET)),
                ("temp7".to_string(), (LTEMP_START + LOCAL_BOOL_OFFSET + 1))
            )
        );
        assert_eq!(
            evaluator.quads.get(8).unwrap(),
            &Quadruple::Op(
                "Or".to_string(),
                ("temp7".to_string(), (LTEMP_START + LOCAL_BOOL_OFFSET + 1)),
                ("True".to_string(), (CNST_START + LOCAL_BOOL_OFFSET + 1)),
                ("temp8".to_string(), (LTEMP_START + LOCAL_BOOL_OFFSET + 2))
            )
        );
        assert_eq!(
            evaluator.quads.get(9).unwrap(),
            &Quadruple::Op(
                "And".to_string(),
                ("temp6".to_string(), (LTEMP_START + LOCAL_BOOL_OFFSET)),
                ("temp8".to_string(), (LTEMP_START + LOCAL_BOOL_OFFSET + 2)),
                ("temp9".to_string(), (LTEMP_START + LOCAL_BOOL_OFFSET + 3))
            )
        );
        assert_eq!(
            evaluator.quads.get(10).unwrap(),
            &Quadruple::Op(
                "Or".to_string(),
                ("temp9".to_string(), (LTEMP_START + LOCAL_BOOL_OFFSET + 3)),
                ("b".to_string(), (GLOBAL_START + GLOBAL_BOOL_OFFSET)),
                ("temp10".to_string(), (LTEMP_START + LOCAL_BOOL_OFFSET + 4))
            )
        );
        assert_eq!(
            evaluator.quads.get(11).unwrap(),
            &Quadruple::Op(
                "Or".to_string(),
                ("temp10".to_string(), (LTEMP_START + LOCAL_BOOL_OFFSET + 4)),
                ("i".to_string(), (GLOBAL_START + GLOBAL_INT_OFFSET)),
                ("temp11".to_string(), (LTEMP_START + LOCAL_BOOL_OFFSET + 5))
            )
        );
        assert_eq!(
            evaluator.quads.get(12).unwrap(),
            &Quadruple::Assign(
                "=".to_string(),
                ("temp11".to_string(), (LTEMP_START + LOCAL_BOOL_OFFSET + 5)),
                ("res".to_string(), (GLOBAL_START + GLOBAL_BOOL_OFFSET + 1))
            )
        );
    }

    #[test]
    #[should_panic]
    fn test_quads_fail_1() {
        let filename = "./tests_compile/quads_fail_1.tabby";
        let contents = fs::read_to_string(filename).unwrap();
        let res = tabby::PROGRAMParser::new().parse(&contents);
        assert!(res.is_ok());
        let my_ast = res.unwrap();
        let mut evaluator = AstEvaluator::new();
        evaluator.eval_program(my_ast);
    }

    #[test]
    fn test_quads_call_ok_1() {
        let filename = "./tests_compile/quads_call_ok_1.tabby";
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
        let filename = "./tests_compile/quads_if_ok_1.tabby";
        let contents = fs::read_to_string(filename).unwrap();
        let res = tabby::PROGRAMParser::new().parse(&contents);
        assert!(res.is_ok());
        let my_ast = res.unwrap();
        let mut evaluator = AstEvaluator::new();
        evaluator.eval_program(my_ast);
        assert_eq!(evaluator.quads.len(), 10);
        assert_eq!(evaluator.quads.get(0).unwrap(), &Quadruple::GoTo(1));
        assert_eq!(
            evaluator.quads.get(1).unwrap(),
            &Quadruple::Read(
                "Read".to_string(),
                ("a".to_string(), (GLOBAL_START + GLOBAL_INT_OFFSET))
            )
        );
        assert_eq!(
            evaluator.quads.get(2).unwrap(),
            &Quadruple::Op(
                "*".to_string(),
                ("12".to_string(), (CNST_START + LOCAL_INT_OFFSET)),
                ("a".to_string(), (GLOBAL_START + GLOBAL_INT_OFFSET)),
                ("temp1".to_string(), (LTEMP_START + LOCAL_INT_OFFSET))
            )
        );
        assert_eq!(
            evaluator.quads.get(3).unwrap(),
            &Quadruple::Op(
                ">".to_string(),
                ("temp1".to_string(), (LTEMP_START + LOCAL_INT_OFFSET)),
                ("18".to_string(), (CNST_START + LOCAL_INT_OFFSET + 1)),
                ("temp2".to_string(), (LTEMP_START + LOCAL_BOOL_OFFSET))
            )
        );
        assert_eq!(
            evaluator.quads.get(4).unwrap(),
            &Quadruple::GoToF(("temp2".to_string(), (LTEMP_START + LOCAL_BOOL_OFFSET)), 8)
        );
        assert_eq!(
            evaluator.quads.get(5).unwrap(),
            &Quadruple::Op(
                "*".to_string(),
                ("a".to_string(), (GLOBAL_START + GLOBAL_INT_OFFSET)),
                ("12".to_string(), (CNST_START + LOCAL_INT_OFFSET)),
                ("temp3".to_string(), (LTEMP_START + LOCAL_INT_OFFSET + 1))
            )
        );
        assert_eq!(
            evaluator.quads.get(6).unwrap(),
            &Quadruple::Assign(
                "=".to_string(),
                ("temp3".to_string(), (LTEMP_START + LOCAL_INT_OFFSET + 1)),
                ("a".to_string(), (GLOBAL_START + GLOBAL_INT_OFFSET))
            )
        );
        assert_eq!(
            evaluator.quads.get(7).unwrap(),
            &Quadruple::Print(
                "PrintSL".to_string(),
                ("\"If".to_string(), (CNST_START + STRLIT_OFFSET))
            )
        );
        assert_eq!(
            evaluator.quads.get(8).unwrap(),
            &Quadruple::Print(
                "PrintSL".to_string(),
                ("\"End".to_string(), (CNST_START + STRLIT_OFFSET + 1))
            )
        );
        assert_eq!(
            evaluator.quads.get(9).unwrap(),
            &Quadruple::Print(
                "Print".to_string(),
                ("a".to_string(), (GLOBAL_START + GLOBAL_INT_OFFSET))
            )
        );
    }

    #[test]
    fn test_quads_if_else_ok_1() {
        let filename = "./tests_compile/quads_if_else_ok_1.tabby";
        let contents = fs::read_to_string(filename).unwrap();
        let res = tabby::PROGRAMParser::new().parse(&contents);
        assert!(res.is_ok());
        let my_ast = res.unwrap();
        let mut evaluator = AstEvaluator::new();
        evaluator.eval_program(my_ast);
        assert_eq!(evaluator.quads.len(), 19);
        assert_eq!(evaluator.quads.get(0).unwrap(), &Quadruple::GoTo(1));
        assert_eq!(
            evaluator.quads.get(1).unwrap(),
            &Quadruple::Read(
                "Read".to_string(),
                ("a".to_string(), (GLOBAL_START + GLOBAL_INT_OFFSET))
            )
        );
        assert_eq!(
            evaluator.quads.get(2).unwrap(),
            &Quadruple::Read(
                "Read".to_string(),
                ("b".to_string(), (GLOBAL_START + GLOBAL_INT_OFFSET + 1))
            )
        );
        assert_eq!(
            evaluator.quads.get(3).unwrap(),
            &Quadruple::Op(
                "*".to_string(),
                ("a".to_string(), (GLOBAL_START + GLOBAL_INT_OFFSET)),
                ("3".to_string(), (CNST_START + LOCAL_INT_OFFSET)),
                ("temp1".to_string(), (LTEMP_START + LOCAL_INT_OFFSET))
            )
        );
        assert_eq!(
            evaluator.quads.get(4).unwrap(),
            &Quadruple::Op(
                "*".to_string(),
                ("b".to_string(), (GLOBAL_START + GLOBAL_INT_OFFSET + 1)),
                ("5".to_string(), (CNST_START + LOCAL_INT_OFFSET + 1)),
                ("temp2".to_string(), (LTEMP_START + LOCAL_INT_OFFSET + 1))
            )
        );
        assert_eq!(
            evaluator.quads.get(5).unwrap(),
            &Quadruple::Op(
                "!=".to_string(),
                ("temp1".to_string(), (LTEMP_START + LOCAL_INT_OFFSET)),
                ("temp2".to_string(), (LTEMP_START + LOCAL_INT_OFFSET + 1)),
                ("temp3".to_string(), (LTEMP_START + LOCAL_BOOL_OFFSET))
            )
        );
        assert_eq!(
            evaluator.quads.get(6).unwrap(),
            &Quadruple::GoToF(("temp3".to_string(), (LTEMP_START + LOCAL_BOOL_OFFSET)), 11)
        );
        assert_eq!(
            evaluator.quads.get(7).unwrap(),
            &Quadruple::Op(
                "+".to_string(),
                ("a".to_string(), (GLOBAL_START + GLOBAL_INT_OFFSET)),
                ("1".to_string(), (CNST_START + LOCAL_INT_OFFSET + 2)),
                ("temp4".to_string(), (LTEMP_START + LOCAL_INT_OFFSET + 2))
            )
        );
        assert_eq!(
            evaluator.quads.get(8).unwrap(),
            &Quadruple::Assign(
                "=".to_string(),
                ("temp4".to_string(), (LTEMP_START + LOCAL_INT_OFFSET + 2)),
                ("a".to_string(), (GLOBAL_START + GLOBAL_INT_OFFSET))
            )
        );
        assert_eq!(
            evaluator.quads.get(9).unwrap(),
            &Quadruple::Print(
                "Print".to_string(),
                ("a".to_string(), (GLOBAL_START + GLOBAL_INT_OFFSET))
            )
        );
        assert_eq!(evaluator.quads.get(10).unwrap(), &Quadruple::GoTo(15));
        assert_eq!(
            evaluator.quads.get(11).unwrap(),
            &Quadruple::Op(
                "+".to_string(),
                ("a".to_string(), (GLOBAL_START + GLOBAL_INT_OFFSET)),
                ("b".to_string(), (GLOBAL_START + GLOBAL_INT_OFFSET + 1)),
                ("temp5".to_string(), (LTEMP_START + LOCAL_INT_OFFSET + 3))
            )
        );
        assert_eq!(
            evaluator.quads.get(12).unwrap(),
            &Quadruple::Op(
                "+".to_string(),
                ("temp5".to_string(), (LTEMP_START + LOCAL_INT_OFFSET + 3)),
                ("1".to_string(), (CNST_START + LOCAL_INT_OFFSET + 2)),
                ("temp6".to_string(), (LTEMP_START + LOCAL_INT_OFFSET + 4))
            )
        );
        assert_eq!(
            evaluator.quads.get(13).unwrap(),
            &Quadruple::Assign(
                "=".to_string(),
                ("temp6".to_string(), (LTEMP_START + LOCAL_INT_OFFSET + 4)),
                ("b".to_string(), (GLOBAL_START + GLOBAL_INT_OFFSET + 1))
            )
        );
        assert_eq!(
            evaluator.quads.get(14).unwrap(),
            &Quadruple::Print(
                "Print".to_string(),
                ("b".to_string(), (GLOBAL_START + GLOBAL_INT_OFFSET + 1))
            )
        );
        assert_eq!(
            evaluator.quads.get(15).unwrap(),
            &Quadruple::Op(
                "*".to_string(),
                ("a".to_string(), (GLOBAL_START + GLOBAL_INT_OFFSET)),
                ("b".to_string(), (GLOBAL_START + GLOBAL_INT_OFFSET + 1)),
                ("temp7".to_string(), (LTEMP_START + LOCAL_INT_OFFSET + 5))
            )
        );
        assert_eq!(
            evaluator.quads.get(16).unwrap(),
            &Quadruple::Op(
                "+".to_string(),
                ("temp7".to_string(), (LTEMP_START + LOCAL_INT_OFFSET + 5)),
                ("2".to_string(), (CNST_START + LOCAL_INT_OFFSET + 3)),
                ("temp8".to_string(), (LTEMP_START + LOCAL_INT_OFFSET + 6))
            )
        );
        assert_eq!(
            evaluator.quads.get(17).unwrap(),
            &Quadruple::Assign(
                "=".to_string(),
                ("temp8".to_string(), (LTEMP_START + LOCAL_INT_OFFSET + 6)),
                ("a".to_string(), (GLOBAL_START + GLOBAL_INT_OFFSET))
            )
        );
        assert_eq!(
            evaluator.quads.get(18).unwrap(),
            &Quadruple::Print(
                "Print".to_string(),
                ("a".to_string(), (GLOBAL_START + GLOBAL_INT_OFFSET))
            )
        );
    }

    #[test]
    #[should_panic]
    fn test_if_fail_1() {
        let filename = "./tests_compile/if_fail_1.tabby";
        let contents = fs::read_to_string(filename).unwrap();
        let res = tabby::PROGRAMParser::new().parse(&contents);
        assert!(res.is_ok());
        let my_ast = res.unwrap();
        let mut evaluator = AstEvaluator::new();
        evaluator.eval_program(my_ast);
    }

    #[test]
    fn test_vir_mem_ok_1() {
        let filename = "./tests_compile/vir_mem_ok_1.tabby";
        let contents = fs::read_to_string(filename).unwrap();
        let res = tabby::PROGRAMParser::new().parse(&contents);
        assert!(res.is_ok());
        let my_ast = res.unwrap();
        let mut evaluator = AstEvaluator::new();
        evaluator.eval_program(my_ast);
        // Test vir mem addresses
        // Globals
        let global_map = &evaluator.dir_func.get("testVirMemAlloc").unwrap().dir_var;
        assert_eq!(
            global_map.get("a").unwrap().addr,
            GLOBAL_START + GLOBAL_INT_OFFSET
        );
        assert_eq!(
            global_map.get("b").unwrap().addr,
            GLOBAL_START + GLOBAL_INT_OFFSET + 1
        );
        assert_eq!(
            global_map.get("c").unwrap().addr,
            GLOBAL_START + GLOBAL_INT_OFFSET + 2
        );
        assert_eq!(
            global_map.get("d").unwrap().addr,
            GLOBAL_START + GLOBAL_FLOAT_OFFSET
        );
        assert_eq!(
            global_map.get("e").unwrap().addr,
            GLOBAL_START + GLOBAL_FLOAT_OFFSET + 1
        );
        assert_eq!(
            global_map.get("f").unwrap().addr,
            GLOBAL_START + GLOBAL_FLOAT_OFFSET + 2
        );
        assert_eq!(
            global_map.get("g").unwrap().addr,
            GLOBAL_START + GLOBAL_BOOL_OFFSET
        );
        assert_eq!(
            global_map.get("h").unwrap().addr,
            GLOBAL_START + GLOBAL_BOOL_OFFSET + 1
        );
        assert_eq!(
            global_map.get("i").unwrap().addr,
            GLOBAL_START + GLOBAL_BOOL_OFFSET + 2
        );
        // Locals
        let fn_one_map = &evaluator.dir_func.get("fnOne").unwrap().dir_var;
        assert_eq!(
            fn_one_map.get("a").unwrap().addr,
            LOCAL_START + LOCAL_INT_OFFSET
        );
        assert_eq!(
            fn_one_map.get("b").unwrap().addr,
            LOCAL_START + LOCAL_FLOAT_OFFSET
        );
        assert_eq!(
            fn_one_map.get("c").unwrap().addr,
            LOCAL_START + LOCAL_BOOL_OFFSET
        );
        let fn_two_map = &evaluator.dir_func.get("fnOne").unwrap().dir_var;
        assert_eq!(
            fn_two_map.get("a").unwrap().addr,
            LOCAL_START + LOCAL_INT_OFFSET
        );
        assert_eq!(
            fn_two_map.get("b").unwrap().addr,
            LOCAL_START + LOCAL_FLOAT_OFFSET
        );
        assert_eq!(
            fn_two_map.get("c").unwrap().addr,
            LOCAL_START + LOCAL_BOOL_OFFSET
        );
        // Test Quads
        assert_eq!(evaluator.quads.len(), 13);
        assert_eq!(evaluator.quads.get(0).unwrap(), &Quadruple::GoTo(3));
        assert_eq!(evaluator.quads.get(1).unwrap(), &Quadruple::EndFunc());
        assert_eq!(evaluator.quads.get(2).unwrap(), &Quadruple::EndFunc());
        assert_eq!(
            evaluator.quads.get(3).unwrap(),
            &Quadruple::Op(
                "*".to_string(),
                ("b".to_string(), (GLOBAL_START + GLOBAL_INT_OFFSET + 1)),
                ("c".to_string(), (GLOBAL_START + GLOBAL_INT_OFFSET + 2)),
                ("temp1".to_string(), (LTEMP_START + LOCAL_INT_OFFSET))
            )
        );
        assert_eq!(
            evaluator.quads.get(4).unwrap(),
            &Quadruple::Op(
                "*".to_string(),
                ("temp1".to_string(), (LTEMP_START + LOCAL_INT_OFFSET)),
                ("1".to_string(), (CNST_START + LOCAL_INT_OFFSET)),
                ("temp2".to_string(), (LTEMP_START + LOCAL_INT_OFFSET + 1))
            )
        );
        assert_eq!(
            evaluator.quads.get(5).unwrap(),
            &Quadruple::Assign(
                "=".to_string(),
                ("temp2".to_string(), (LTEMP_START + LOCAL_INT_OFFSET + 1)),
                ("a".to_string(), (GLOBAL_START + GLOBAL_INT_OFFSET))
            )
        );
        assert_eq!(
            evaluator.quads.get(6).unwrap(),
            &Quadruple::Op(
                "/".to_string(),
                ("e".to_string(), (GLOBAL_START + GLOBAL_FLOAT_OFFSET + 1)),
                ("f".to_string(), (GLOBAL_START + GLOBAL_FLOAT_OFFSET + 2)),
                ("temp3".to_string(), (LTEMP_START + LOCAL_FLOAT_OFFSET))
            )
        );
        assert_eq!(
            evaluator.quads.get(7).unwrap(),
            &Quadruple::Op(
                "*".to_string(),
                ("temp3".to_string(), (LTEMP_START + LOCAL_FLOAT_OFFSET)),
                ("1.2".to_string(), (CNST_START + LOCAL_FLOAT_OFFSET)),
                ("temp4".to_string(), (LTEMP_START + LOCAL_FLOAT_OFFSET + 1))
            )
        );
        assert_eq!(
            evaluator.quads.get(8).unwrap(),
            &Quadruple::Assign(
                "=".to_string(),
                ("temp4".to_string(), (LTEMP_START + LOCAL_FLOAT_OFFSET + 1)),
                ("d".to_string(), (GLOBAL_START + GLOBAL_FLOAT_OFFSET))
            )
        );
        assert_eq!(
            evaluator.quads.get(9).unwrap(),
            &Quadruple::Op(
                "And".to_string(),
                ("h".to_string(), (GLOBAL_START + GLOBAL_BOOL_OFFSET + 1)),
                ("i".to_string(), (GLOBAL_START + GLOBAL_BOOL_OFFSET + 2)),
                ("temp5".to_string(), (LTEMP_START + LOCAL_BOOL_OFFSET))
            )
        );
        assert_eq!(
            evaluator.quads.get(10).unwrap(),
            &Quadruple::Op(
                "Or".to_string(),
                ("temp5".to_string(), (LTEMP_START + LOCAL_BOOL_OFFSET)),
                ("True".to_string(), (CNST_START + LOCAL_BOOL_OFFSET)),
                ("temp6".to_string(), (LTEMP_START + LOCAL_BOOL_OFFSET + 1))
            )
        );
        assert_eq!(
            evaluator.quads.get(11).unwrap(),
            &Quadruple::Assign(
                "=".to_string(),
                ("temp6".to_string(), (LTEMP_START + LOCAL_BOOL_OFFSET + 1)),
                ("g".to_string(), (GLOBAL_START + GLOBAL_BOOL_OFFSET))
            )
        );
        assert_eq!(
            evaluator.quads.get(12).unwrap(),
            &Quadruple::Print(
                "PrintSL".to_string(),
                ("\"Wow".to_string(), (CNST_START + STRLIT_OFFSET))
            )
        );
    }

    #[test]
    fn test_vir_mem_ok_2() {
        let filename = "./tests_compile/vir_mem_ok_2.tabby";
        let contents = fs::read_to_string(filename).unwrap();
        let res = tabby::PROGRAMParser::new().parse(&contents);
        assert!(res.is_ok());
        let my_ast = res.unwrap();
        let mut evaluator = AstEvaluator::new();
        evaluator.eval_program(my_ast);
        // Check global sizes
        let gb_info = evaluator.dir_func.get("testVirMemAllocNonAtomic").unwrap();
        assert_eq!(gb_info.size_loc, [925, 2031, 176]);
        assert_eq!(gb_info.size_tmp, [1, 1, 1]);
        // Test vir mem addresses
        // Globals
        let global_map = &evaluator
            .dir_func
            .get("testVirMemAllocNonAtomic")
            .unwrap()
            .dir_var;
        assert_eq!(
            global_map.get("ia").unwrap().addr,
            GLOBAL_START + GLOBAL_INT_OFFSET
        );
        assert_eq!(
            global_map.get("ib").unwrap().addr,
            GLOBAL_START + GLOBAL_INT_OFFSET + 20
        );
        assert_eq!(
            global_map.get("ic").unwrap().addr,
            GLOBAL_START + GLOBAL_INT_OFFSET + 320
        );
        assert_eq!(
            global_map.get("id").unwrap().addr,
            GLOBAL_START + GLOBAL_INT_OFFSET + 920
        );
        assert_eq!(
            global_map.get("fa").unwrap().addr,
            GLOBAL_START + GLOBAL_FLOAT_OFFSET
        );
        assert_eq!(
            global_map.get("fb").unwrap().addr,
            GLOBAL_START + GLOBAL_FLOAT_OFFSET + 3
        );
        assert_eq!(
            global_map.get("fc").unwrap().addr,
            GLOBAL_START + GLOBAL_FLOAT_OFFSET + 19
        );
        assert_eq!(
            global_map.get("fd").unwrap().addr,
            GLOBAL_START + GLOBAL_FLOAT_OFFSET + 2019
        );
        assert_eq!(
            global_map.get("ba").unwrap().addr,
            GLOBAL_START + GLOBAL_BOOL_OFFSET
        );
        assert_eq!(
            global_map.get("bb").unwrap().addr,
            GLOBAL_START + GLOBAL_BOOL_OFFSET + 13
        );
        assert_eq!(
            global_map.get("bc").unwrap().addr,
            GLOBAL_START + GLOBAL_BOOL_OFFSET + 167
        );
        assert_eq!(
            global_map.get("bd").unwrap().addr,
            GLOBAL_START + GLOBAL_BOOL_OFFSET + 170
        );
    }

    #[test]
    fn test_quads_while_ok_1() {
        let filename = "./tests_compile/quads_while_ok_1.tabby";
        let contents = fs::read_to_string(filename).unwrap();
        let res = tabby::PROGRAMParser::new().parse(&contents);
        assert!(res.is_ok());
        let my_ast = res.unwrap();
        let mut evaluator = AstEvaluator::new();
        evaluator.eval_program(my_ast);
        assert_eq!(evaluator.quads.len(), 12);
        assert_eq!(evaluator.quads.get(0).unwrap(), &Quadruple::GoTo(1));
        assert_eq!(
            evaluator.quads.get(1).unwrap(),
            &Quadruple::Read(
                "Read".to_string(),
                ("a".to_string(), (GLOBAL_START + GLOBAL_INT_OFFSET))
            )
        );
        assert_eq!(
            evaluator.quads.get(2).unwrap(),
            &Quadruple::Op(
                "+".to_string(),
                ("a".to_string(), (GLOBAL_START + GLOBAL_INT_OFFSET)),
                ("5".to_string(), (CNST_START + GLOBAL_INT_OFFSET)),
                ("temp1".to_string(), (LTEMP_START + LOCAL_INT_OFFSET))
            )
        );
        assert_eq!(
            evaluator.quads.get(3).unwrap(),
            &Quadruple::Op(
                "*".to_string(),
                ("10".to_string(), (CNST_START + GLOBAL_INT_OFFSET + 1)),
                ("2".to_string(), (CNST_START + GLOBAL_INT_OFFSET + 2)),
                ("temp2".to_string(), (LTEMP_START + LOCAL_INT_OFFSET + 1))
            )
        );
        assert_eq!(
            evaluator.quads.get(4).unwrap(),
            &Quadruple::Op(
                "<".to_string(),
                ("temp1".to_string(), (LTEMP_START + LOCAL_INT_OFFSET)),
                ("temp2".to_string(), (LTEMP_START + LOCAL_INT_OFFSET + 1)),
                ("temp3".to_string(), (LTEMP_START + LOCAL_BOOL_OFFSET))
            )
        );
        assert_eq!(
            evaluator.quads.get(5).unwrap(),
            &Quadruple::GoToF(("temp3".to_string(), (LTEMP_START + LOCAL_BOOL_OFFSET)), 10)
        );
        assert_eq!(
            evaluator.quads.get(6).unwrap(),
            &Quadruple::Print(
                "Print".to_string(),
                ("a".to_string(), (GLOBAL_START + GLOBAL_INT_OFFSET))
            )
        );
        assert_eq!(
            evaluator.quads.get(7).unwrap(),
            &Quadruple::Op(
                "+".to_string(),
                ("a".to_string(), (GLOBAL_START + GLOBAL_INT_OFFSET)),
                ("1".to_string(), (CNST_START + GLOBAL_INT_OFFSET + 3)),
                ("temp4".to_string(), (LTEMP_START + LOCAL_INT_OFFSET + 2))
            )
        );
        assert_eq!(
            evaluator.quads.get(8).unwrap(),
            &Quadruple::Assign(
                "=".to_string(),
                ("temp4".to_string(), (LTEMP_START + LOCAL_INT_OFFSET + 2)),
                ("a".to_string(), (GLOBAL_START + GLOBAL_INT_OFFSET))
            )
        );
        assert_eq!(evaluator.quads.get(9).unwrap(), &Quadruple::GoTo(2));
        assert_eq!(
            evaluator.quads.get(10).unwrap(),
            &Quadruple::Print(
                "PrintSL".to_string(),
                ("\"Finish".to_string(), (CNST_START + STRLIT_OFFSET))
            )
        );
        assert_eq!(
            evaluator.quads.get(11).unwrap(),
            &Quadruple::Print(
                "Print".to_string(),
                ("a".to_string(), (GLOBAL_START + GLOBAL_INT_OFFSET))
            )
        );
    }

    #[test]
    fn test_quads_for_ok_1() {
        let filename = "./tests_compile/quads_for_ok_1.tabby";
        let contents = fs::read_to_string(filename).unwrap();
        let res = tabby::PROGRAMParser::new().parse(&contents);
        assert!(res.is_ok());
        let my_ast = res.unwrap();
        let mut evaluator = AstEvaluator::new();
        evaluator.eval_program(my_ast);
        assert_eq!(evaluator.quads.len(), 11);
        assert_eq!(evaluator.quads.get(0).unwrap(), &Quadruple::GoTo(1));
        assert_eq!(
            evaluator.quads.get(1).unwrap(),
            &Quadruple::Read(
                "Read".to_string(),
                ("i".to_string(), (GLOBAL_START + GLOBAL_INT_OFFSET))
            )
        );
        assert_eq!(
            evaluator.quads.get(2).unwrap(),
            &Quadruple::Op(
                "+".to_string(),
                ("i".to_string(), (GLOBAL_START + GLOBAL_INT_OFFSET)),
                ("1".to_string(), (CNST_START + LOCAL_INT_OFFSET)),
                ("temp1".to_string(), (LTEMP_START + LOCAL_INT_OFFSET))
            )
        );
        assert_eq!(
            evaluator.quads.get(3).unwrap(),
            &Quadruple::Op(
                ">".to_string(),
                ("temp1".to_string(), (LTEMP_START + LOCAL_INT_OFFSET)),
                ("5".to_string(), (CNST_START + LOCAL_INT_OFFSET + 1)),
                ("temp2".to_string(), (LTEMP_START + LOCAL_BOOL_OFFSET))
            )
        );
        assert_eq!(
            evaluator.quads.get(4).unwrap(),
            &Quadruple::GoToF(("temp2".to_string(), (LTEMP_START + LOCAL_BOOL_OFFSET)), 10)
        );
        assert_eq!(
            evaluator.quads.get(5).unwrap(),
            &Quadruple::Print(
                "PrintSL".to_string(),
                ("\"Loop".to_string(), (CNST_START + STRLIT_OFFSET))
            )
        );
        assert_eq!(
            evaluator.quads.get(6).unwrap(),
            &Quadruple::Print(
                "Print".to_string(),
                ("i".to_string(), (GLOBAL_START + GLOBAL_INT_OFFSET))
            )
        );
        assert_eq!(
            evaluator.quads.get(7).unwrap(),
            &Quadruple::Op(
                "*".to_string(),
                ("2".to_string(), (CNST_START + LOCAL_INT_OFFSET + 2)),
                ("i".to_string(), (GLOBAL_START + GLOBAL_INT_OFFSET)),
                ("temp3".to_string(), (LTEMP_START + LOCAL_INT_OFFSET + 1))
            )
        );
        assert_eq!(
            evaluator.quads.get(8).unwrap(),
            &Quadruple::Assign(
                "=".to_string(),
                ("temp3".to_string(), (LTEMP_START + LOCAL_INT_OFFSET + 1)),
                ("i".to_string(), (GLOBAL_START + GLOBAL_INT_OFFSET))
            )
        );
        assert_eq!(evaluator.quads.get(9).unwrap(), &Quadruple::GoTo(2));
        assert_eq!(
            evaluator.quads.get(10).unwrap(),
            &Quadruple::Print(
                "PrintSL".to_string(),
                ("\"Finish".to_string(), (CNST_START + STRLIT_OFFSET + 1))
            )
        );
    }

    #[test]
    fn test_quads_non_linear_ok_1() {
        let filename = "./tests_compile/quads_non_linear_ok_1.tabby";
        let contents = fs::read_to_string(filename).unwrap();
        let res = tabby::PROGRAMParser::new().parse(&contents);
        assert!(res.is_ok());
        let my_ast = res.unwrap();
        let mut evaluator = AstEvaluator::new();
        evaluator.eval_program(my_ast);
        assert_eq!(evaluator.quads.len(), 25);
        assert_eq!(evaluator.quads.get(0).unwrap(), &Quadruple::GoTo(1));
        assert_eq!(
            evaluator.quads.get(1).unwrap(),
            &Quadruple::Print(
                "PrintSL".to_string(),
                ("\"ProgramStt".to_string(), (CNST_START + STRLIT_OFFSET))
            )
        );
        assert_eq!(
            evaluator.quads.get(2).unwrap(),
            &Quadruple::Op(
                "==".to_string(),
                ("i".to_string(), (GLOBAL_START + GLOBAL_BOOL_OFFSET)),
                ("True".to_string(), (CNST_START + LOCAL_BOOL_OFFSET)),
                ("temp1".to_string(), (LTEMP_START + LOCAL_BOOL_OFFSET))
            )
        );
        assert_eq!(
            evaluator.quads.get(3).unwrap(),
            &Quadruple::GoToF(("temp1".to_string(), (LTEMP_START + LOCAL_BOOL_OFFSET)), 24)
        );
        assert_eq!(
            evaluator.quads.get(4).unwrap(),
            &Quadruple::Print(
                "PrintSL".to_string(),
                ("\"ForStt".to_string(), (CNST_START + STRLIT_OFFSET + 1))
            )
        );
        assert_eq!(
            evaluator.quads.get(5).unwrap(),
            &Quadruple::Op(
                "==".to_string(),
                ("i".to_string(), (GLOBAL_START + GLOBAL_BOOL_OFFSET)),
                ("True".to_string(), (CNST_START + LOCAL_BOOL_OFFSET)),
                ("temp2".to_string(), (LTEMP_START + LOCAL_BOOL_OFFSET + 1))
            )
        );
        assert_eq!(
            evaluator.quads.get(6).unwrap(),
            &Quadruple::GoToF(
                ("temp2".to_string(), (LTEMP_START + LOCAL_BOOL_OFFSET + 1)),
                15
            )
        );
        assert_eq!(
            evaluator.quads.get(7).unwrap(),
            &Quadruple::Print(
                "PrintSL".to_string(),
                ("\"IfStt".to_string(), (CNST_START + STRLIT_OFFSET + 2))
            )
        );
        assert_eq!(
            evaluator.quads.get(8).unwrap(),
            &Quadruple::Op(
                "==".to_string(),
                ("i".to_string(), (GLOBAL_START + GLOBAL_BOOL_OFFSET)),
                ("True".to_string(), (CNST_START + LOCAL_BOOL_OFFSET)),
                ("temp3".to_string(), (LTEMP_START + LOCAL_BOOL_OFFSET + 2))
            )
        );
        assert_eq!(
            evaluator.quads.get(9).unwrap(),
            &Quadruple::GoToF(
                ("temp3".to_string(), (LTEMP_START + LOCAL_BOOL_OFFSET + 2)),
                12
            )
        );
        assert_eq!(
            evaluator.quads.get(10).unwrap(),
            &Quadruple::Print(
                "PrintSL".to_string(),
                ("\"While".to_string(), (CNST_START + STRLIT_OFFSET + 3))
            )
        );
        assert_eq!(evaluator.quads.get(11).unwrap(), &Quadruple::GoTo(8));
        assert_eq!(
            evaluator.quads.get(12).unwrap(),
            &Quadruple::Print(
                "PrintSL".to_string(),
                ("\"ExitWhile".to_string(), (CNST_START + STRLIT_OFFSET + 4))
            )
        );
        assert_eq!(
            evaluator.quads.get(13).unwrap(),
            &Quadruple::Print(
                "PrintSL".to_string(),
                ("\"IfEnd".to_string(), (CNST_START + STRLIT_OFFSET + 5))
            )
        );
        assert_eq!(evaluator.quads.get(14).unwrap(), &Quadruple::GoTo(21));
        assert_eq!(
            evaluator.quads.get(15).unwrap(),
            &Quadruple::Print(
                "PrintSL".to_string(),
                ("\"ElseStt".to_string(), (CNST_START + STRLIT_OFFSET + 6))
            )
        );
        assert_eq!(
            evaluator.quads.get(16).unwrap(),
            &Quadruple::Op(
                "==".to_string(),
                ("i".to_string(), (GLOBAL_START + GLOBAL_BOOL_OFFSET)),
                ("False".to_string(), (CNST_START + LOCAL_BOOL_OFFSET + 1)),
                ("temp4".to_string(), (LTEMP_START + LOCAL_BOOL_OFFSET + 3))
            )
        );
        assert_eq!(
            evaluator.quads.get(17).unwrap(),
            &Quadruple::GoToF(
                ("temp4".to_string(), (LTEMP_START + LOCAL_BOOL_OFFSET + 3)),
                19
            )
        );
        assert_eq!(
            evaluator.quads.get(18).unwrap(),
            &Quadruple::Print(
                "PrintSL".to_string(),
                ("\"If2".to_string(), (CNST_START + STRLIT_OFFSET + 7))
            )
        );
        assert_eq!(
            evaluator.quads.get(19).unwrap(),
            &Quadruple::Print(
                "PrintSL".to_string(),
                ("\"ExitIf2".to_string(), (CNST_START + STRLIT_OFFSET + 8))
            )
        );
        assert_eq!(
            evaluator.quads.get(20).unwrap(),
            &Quadruple::Print(
                "PrintSL".to_string(),
                ("\"ElseEnd".to_string(), (CNST_START + STRLIT_OFFSET + 9))
            )
        );
        assert_eq!(
            evaluator.quads.get(21).unwrap(),
            &Quadruple::Print(
                "PrintSL".to_string(),
                ("\"ForEnd".to_string(), (CNST_START + STRLIT_OFFSET + 10))
            )
        );
        assert_eq!(
            evaluator.quads.get(22).unwrap(),
            &Quadruple::Assign(
                "=".to_string(),
                ("True".to_string(), (CNST_START + LOCAL_BOOL_OFFSET)),
                ("i".to_string(), (GLOBAL_START + GLOBAL_BOOL_OFFSET))
            )
        );
        assert_eq!(evaluator.quads.get(23).unwrap(), &Quadruple::GoTo(2));
        assert_eq!(
            evaluator.quads.get(24).unwrap(),
            &Quadruple::Print(
                "PrintSL".to_string(),
                (
                    "\"ProgramEnd".to_string(),
                    (CNST_START + STRLIT_OFFSET + 11)
                )
            )
        );
    }

    #[test]
    fn test_fns_param_list_ok_1() {
        let filename = "./tests_compile/fns_param_list_ok_1.tabby";
        let contents = fs::read_to_string(filename).unwrap();
        let res = tabby::PROGRAMParser::new().parse(&contents);
        assert!(res.is_ok());
        let my_ast = res.unwrap();
        let mut evaluator = AstEvaluator::new();
        evaluator.eval_program(my_ast);
        let fn_info = evaluator.dir_func.get("fnVoidParams").unwrap();
        assert_eq!(fn_info.size_loc, [1, 1, 1]);
        assert_eq!(fn_info.params, [Tipo::Int, Tipo::Bool, Tipo::Float]);
        let fn_info = evaluator.dir_func.get("fnIntParams").unwrap();
        assert_eq!(fn_info.size_loc, [1, 1, 2]);
        assert_eq!(
            fn_info.params,
            [Tipo::Bool, Tipo::Float, Tipo::Int, Tipo::Bool]
        );
        let fn_info = evaluator.dir_func.get("fnVoid").unwrap();
        assert_eq!(fn_info.size_loc, [0, 0, 0]);
        assert_eq!(fn_info.params, []);
        let fn_info = evaluator.dir_func.get("fnFloat").unwrap();
        assert_eq!(fn_info.size_loc, [0, 0, 0]);
        assert_eq!(fn_info.params, []);
        let fn_info = evaluator.dir_func.get("Tabby").unwrap();
        assert_eq!(fn_info.size_loc, [0, 0, 0]);
        assert_eq!(fn_info.params, []);
    }

    #[test]
    fn test_fns_size_ok_1() {
        let filename = "./tests_compile/fns_size_ok_1.tabby";
        let contents = fs::read_to_string(filename).unwrap();
        let res = tabby::PROGRAMParser::new().parse(&contents);
        assert!(res.is_ok());
        let my_ast = res.unwrap();
        let mut evaluator = AstEvaluator::new();
        evaluator.eval_program(my_ast);
        let fn_info = evaluator.dir_func.get("testFunctionSize").unwrap();
        assert_eq!(fn_info.params, []);
        assert_eq!(fn_info.size_loc, [3, 0, 0]);
        assert_eq!(fn_info.size_tmp, [1, 1, 1]);
        let fn_info = evaluator.dir_func.get("fnVoidParams").unwrap();
        assert_eq!(fn_info.params, [Tipo::Int, Tipo::Int]);
        assert_eq!(fn_info.size_loc, [2, 0, 0]);
        assert_eq!(fn_info.size_tmp, [2, 0, 1]);
        let fn_info = evaluator.dir_func.get("fnIntParams").unwrap();
        assert_eq!(fn_info.params, [Tipo::Bool, Tipo::Float, Tipo::Int]);
        assert_eq!(fn_info.size_loc, [1, 1, 1]);
        assert_eq!(fn_info.size_tmp, [0, 1, 2]);
        let fn_info = evaluator.dir_func.get("fnVoid").unwrap();
        assert_eq!(fn_info.params, []);
        assert_eq!(fn_info.size_loc, [0, 0, 0]);
        assert_eq!(fn_info.size_tmp, [2, 0, 2]);
        let fn_info = evaluator.dir_func.get("fnFloat").unwrap();
        assert_eq!(fn_info.params, []);
        assert_eq!(fn_info.size_loc, [0, 0, 0]);
        let fn_info = evaluator.dir_func.get("Tabby").unwrap();
        assert_eq!(fn_info.params, []);
        assert_eq!(fn_info.size_loc, [0, 0, 0]);
        assert_eq!(fn_info.size_tmp, [0, 0, 1]);
    }

    #[test]
    fn test_quads_fns_ok_1() {
        let filename = "./tests_compile/quads_fns_ok_1.tabby";
        let contents = fs::read_to_string(filename).unwrap();
        let res = tabby::PROGRAMParser::new().parse(&contents);
        assert!(res.is_ok());
        let my_ast = res.unwrap();
        let mut evaluator = AstEvaluator::new();
        evaluator.eval_program(my_ast);
        assert_eq!(evaluator.quads.len(), 19);
        assert_eq!(evaluator.quads.get(0).unwrap(), &Quadruple::GoTo(19));
        // fnVoid starts and has 5 ops
        assert_eq!(evaluator.quads.get(6).unwrap(), &Quadruple::EndFunc());
        // fnInit starts and has 5 irrelevant quads
        assert_eq!(
            evaluator.quads.get(12).unwrap(),
            &Quadruple::Assign(
                "=".to_string(),
                ("b".to_string(), (GLOBAL_START + GLOBAL_INT_OFFSET + 1)),
                ("Ret".to_string(), (GTEMP_START))
            )
        );
        assert_eq!(evaluator.quads.get(13).unwrap(), &Quadruple::Return());
        assert_eq!(
            evaluator.quads.get(14).unwrap(),
            &Quadruple::Assign(
                "=".to_string(),
                ("0".to_string(), (CNST_START + LOCAL_INT_OFFSET + 3)),
                ("Ret".to_string(), (GTEMP_START_INT))
            )
        );
        assert_eq!(evaluator.quads.get(15).unwrap(), &Quadruple::EndFunc());
        // fnFloat starts and has 1 irrelevant quad
        assert_eq!(
            evaluator.quads.get(17).unwrap(),
            &Quadruple::Assign(
                "=".to_string(),
                ("0.0".to_string(), (CNST_START + LOCAL_FLOAT_OFFSET + 1)),
                ("Ret".to_string(), (GTEMP_START_FLOAT))
            )
        );
        assert_eq!(evaluator.quads.get(18).unwrap(), &Quadruple::EndFunc());

        let fn_info = evaluator.dir_func.get("fnVoid").unwrap();
        assert_eq!(fn_info.pos_init, 1);
        let fn_info = evaluator.dir_func.get("fnInt").unwrap();
        assert_eq!(fn_info.pos_init, 7);
        let fn_info = evaluator.dir_func.get("fnFloat").unwrap();
        assert_eq!(fn_info.pos_init, 16);
        let fn_info = evaluator.dir_func.get("Tabby").unwrap();
        assert_eq!(fn_info.pos_init, 19);
    }

    #[test]
    #[should_panic]
    fn test_return_fail_1() {
        let filename = "./tests_compile/return_fail_1.tabby";
        let contents = fs::read_to_string(filename).unwrap();
        let res = tabby::PROGRAMParser::new().parse(&contents);
        assert!(res.is_ok());
        let my_ast = res.unwrap();
        let mut evaluator = AstEvaluator::new();
        evaluator.eval_program(my_ast);
    }

    #[test]
    #[should_panic]
    fn test_return_fail_2() {
        let filename = "./tests_compile/return_fail_2.tabby";
        let contents = fs::read_to_string(filename).unwrap();
        let res = tabby::PROGRAMParser::new().parse(&contents);
        assert!(res.is_ok());
        let my_ast = res.unwrap();
        let mut evaluator = AstEvaluator::new();
        evaluator.eval_program(my_ast);
    }

    #[test]
    #[should_panic]
    fn test_return_fail_3() {
        let filename = "./tests_compile/return_fail_3.tabby";
        let contents = fs::read_to_string(filename).unwrap();
        let res = tabby::PROGRAMParser::new().parse(&contents);
        assert!(res.is_ok());
        let my_ast = res.unwrap();
        let mut evaluator = AstEvaluator::new();
        evaluator.eval_program(my_ast);
    }

    #[test]
    #[should_panic]
    fn test_return_fail_4() {
        let filename = "./tests_compile/return_fail_4.tabby";
        let contents = fs::read_to_string(filename).unwrap();
        let res = tabby::PROGRAMParser::new().parse(&contents);
        assert!(res.is_ok());
        let my_ast = res.unwrap();
        let mut evaluator = AstEvaluator::new();
        evaluator.eval_program(my_ast);
    }

    #[test]
    fn test_quads_ret_ok_1() {
        let filename = "./tests_compile/quads_ret_ok_1.tabby";
        let contents = fs::read_to_string(filename).unwrap();
        let res = tabby::PROGRAMParser::new().parse(&contents);
        assert!(res.is_ok());
        let my_ast = res.unwrap();
        let mut evaluator = AstEvaluator::new();
        evaluator.eval_program(my_ast);
        assert_eq!(evaluator.quads.len(), 21);
        assert_eq!(evaluator.quads.get(0).unwrap(), &Quadruple::GoTo(15));
        assert_eq!(
            evaluator.quads.get(1).unwrap(),
            &Quadruple::Op(
                "==".to_string(),
                ("x".to_string(), (LOCAL_START + LOCAL_INT_OFFSET)),
                ("0".to_string(), (CNST_START + LOCAL_INT_OFFSET)),
                ("temp1".to_string(), (LTEMP_START + LOCAL_BOOL_OFFSET)),
            )
        );
        assert_eq!(
            evaluator.quads.get(2).unwrap(),
            &Quadruple::GoToF(("temp1".to_string(), (LTEMP_START + LOCAL_BOOL_OFFSET)), 5)
        );
        assert_eq!(
            evaluator.quads.get(3).unwrap(),
            &Quadruple::Assign(
                "=".to_string(),
                ("1".to_string(), (CNST_START + LOCAL_INT_OFFSET + 1)),
                ("Ret".to_string(), (GTEMP_START))
            )
        );
        assert_eq!(evaluator.quads.get(4).unwrap(), &Quadruple::Return());
        assert_eq!(
            evaluator.quads.get(5).unwrap(),
            &Quadruple::Era("fib".to_string())
        );
        assert_eq!(
            evaluator.quads.get(6).unwrap(),
            &Quadruple::Op(
                "-".to_string(),
                ("x".to_string(), (LOCAL_START + LOCAL_INT_OFFSET)),
                ("1".to_string(), (CNST_START + LOCAL_INT_OFFSET + 1)),
                ("temp2".to_string(), (LTEMP_START + LOCAL_INT_OFFSET)),
            )
        );
        assert_eq!(
            evaluator.quads.get(7).unwrap(),
            &Quadruple::Parameter(
                ("temp2".to_string(), (LTEMP_START + LOCAL_INT_OFFSET)),
                LOCAL_START_INT
            )
        );
        assert_eq!(
            evaluator.quads.get(8).unwrap(),
            &Quadruple::GoSub("fib".to_string(), 1)
        );
        assert_eq!(
            evaluator.quads.get(9).unwrap(),
            &Quadruple::Assign(
                "=".to_string(),
                ("Ret".to_string(), (GTEMP_START)),
                ("temp3".to_string(), (LTEMP_START + LOCAL_INT_OFFSET + 1))
            )
        );
        assert_eq!(
            evaluator.quads.get(10).unwrap(),
            &Quadruple::Op(
                "*".to_string(),
                ("temp3".to_string(), (LTEMP_START + LOCAL_INT_OFFSET + 1)),
                ("x".to_string(), (LOCAL_START + LOCAL_INT_OFFSET)),
                ("temp4".to_string(), (LTEMP_START + LOCAL_INT_OFFSET + 2)),
            )
        );
        assert_eq!(
            evaluator.quads.get(11).unwrap(),
            &Quadruple::Assign(
                "=".to_string(),
                ("temp4".to_string(), (LTEMP_START + LOCAL_INT_OFFSET + 2)),
                ("Ret".to_string(), (GTEMP_START))
            )
        );
        assert_eq!(evaluator.quads.get(12).unwrap(), &Quadruple::Return());
        assert_eq!(
            evaluator.quads.get(13).unwrap(),
            &Quadruple::Assign(
                "=".to_string(),
                ("0".to_string(), (CNST_START_INT)),
                ("Ret".to_string(), (GTEMP_START_INT))
            )
        );
        assert_eq!(evaluator.quads.get(14).unwrap(), &Quadruple::EndFunc());
        assert_eq!(
            evaluator.quads.get(15).unwrap(),
            &Quadruple::Read(
                "Read".to_string(),
                ("n".to_string(), (GLOBAL_START + GLOBAL_INT_OFFSET))
            )
        );
        assert_eq!(
            evaluator.quads.get(16).unwrap(),
            &Quadruple::Era("fib".to_string())
        );
        assert_eq!(
            evaluator.quads.get(17).unwrap(),
            &Quadruple::Parameter(
                ("n".to_string(), (GLOBAL_START + GLOBAL_INT_OFFSET)),
                LOCAL_START_INT
            )
        );
        assert_eq!(
            evaluator.quads.get(18).unwrap(),
            &Quadruple::GoSub("fib".to_string(), 1)
        );
        assert_eq!(
            evaluator.quads.get(19).unwrap(),
            &Quadruple::Assign(
                "=".to_string(),
                ("Ret".to_string(), (GTEMP_START)),
                ("temp1".to_string(), (LTEMP_START + LOCAL_INT_OFFSET))
            )
        );
        assert_eq!(
            evaluator.quads.get(20).unwrap(),
            &Quadruple::Print(
                "Print".to_string(),
                ("temp1".to_string(), (LTEMP_START + LOCAL_INT_OFFSET))
            )
        );
    }

    #[test]
    fn test_quads_ret_ok_2() {
        let filename = "./tests_compile/quads_ret_ok_2.tabby";
        let contents = fs::read_to_string(filename).unwrap();
        let res = tabby::PROGRAMParser::new().parse(&contents);
        assert!(res.is_ok());
        let my_ast = res.unwrap();
        let mut evaluator = AstEvaluator::new();
        evaluator.eval_program(my_ast);
        assert_eq!(evaluator.quads.len(), 18);
        assert_eq!(evaluator.quads.get(0).unwrap(), &Quadruple::GoTo(10));
        assert_eq!(
            evaluator.quads.get(1).unwrap(),
            &Quadruple::Assign(
                "=".to_string(),
                ("1.2".to_string(), (CNST_START + LOCAL_FLOAT_OFFSET)),
                ("Ret".to_string(), (GTEMP_START + 1))
            )
        );
        assert_eq!(evaluator.quads.get(2).unwrap(), &Quadruple::Return());
        assert_eq!(
            evaluator.quads.get(3).unwrap(),
            &Quadruple::Assign(
                "=".to_string(),
                ("0.0".to_string(), (CNST_START + LOCAL_FLOAT_OFFSET + 1)),
                ("Ret".to_string(), (GTEMP_START + 1))
            )
        );
        assert_eq!(evaluator.quads.get(4).unwrap(), &Quadruple::EndFunc());
        assert_eq!(
            evaluator.quads.get(5).unwrap(),
            &Quadruple::Assign(
                "=".to_string(),
                ("True".to_string(), (CNST_START + LOCAL_BOOL_OFFSET)),
                ("Ret".to_string(), (GTEMP_START + 2))
            )
        );
        assert_eq!(evaluator.quads.get(6).unwrap(), &Quadruple::Return());
        assert_eq!(
            evaluator.quads.get(7).unwrap(),
            &Quadruple::Assign(
                "=".to_string(),
                ("False".to_string(), (CNST_START + LOCAL_BOOL_OFFSET + 1)),
                ("Ret".to_string(), (GTEMP_START + 2))
            )
        );
        assert_eq!(evaluator.quads.get(8).unwrap(), &Quadruple::EndFunc());
        assert_eq!(evaluator.quads.get(9).unwrap(), &Quadruple::EndFunc());
        assert_eq!(
            evaluator.quads.get(10).unwrap(),
            &Quadruple::Era("float".to_string())
        );
        assert_eq!(
            evaluator.quads.get(11).unwrap(),
            &Quadruple::GoSub("float".to_string(), 1)
        );
        assert_eq!(
            evaluator.quads.get(12).unwrap(),
            &Quadruple::Assign(
                "=".to_string(),
                ("Ret".to_string(), (GTEMP_START + 1)),
                ("temp1".to_string(), (LTEMP_START + LOCAL_FLOAT_OFFSET))
            )
        );
        assert_eq!(
            evaluator.quads.get(13).unwrap(),
            &Quadruple::Era("bool".to_string())
        );
        assert_eq!(
            evaluator.quads.get(14).unwrap(),
            &Quadruple::GoSub("bool".to_string(), 5)
        );
        assert_eq!(
            evaluator.quads.get(15).unwrap(),
            &Quadruple::Assign(
                "=".to_string(),
                ("Ret".to_string(), (GTEMP_START + 2)),
                ("temp2".to_string(), (LTEMP_START + LOCAL_BOOL_OFFSET))
            )
        );
        assert_eq!(
            evaluator.quads.get(16).unwrap(),
            &Quadruple::Era("void".to_string())
        );
        assert_eq!(
            evaluator.quads.get(17).unwrap(),
            &Quadruple::GoSub("void".to_string(), 9)
        );
    }

    #[test]
    #[should_panic]
    fn test_dims_fail_1() {
        let filename = "./tests_compile/dims_fail_1.tabby";
        let contents = fs::read_to_string(filename).unwrap();
        let res = tabby::PROGRAMParser::new().parse(&contents);
        assert!(res.is_ok());
        let my_ast = res.unwrap();
        let mut evaluator = AstEvaluator::new();
        evaluator.eval_program(my_ast);
    }

    #[test]
    #[should_panic]
    fn test_too_many_vars_fail_1() {
        let filename = "./tests_compile/too_many_vars_fail_1.tabby";
        let contents = fs::read_to_string(filename).unwrap();
        let res = tabby::PROGRAMParser::new().parse(&contents);
        assert!(res.is_ok());
        let my_ast = res.unwrap();
        let mut evaluator = AstEvaluator::new();
        evaluator.eval_program(my_ast);
    }

    #[test]
    #[should_panic]
    fn test_arr_dec_fail_1() {
        let filename = "./tests_compile/arr_dec_fail_1.tabby";
        let contents = fs::read_to_string(filename).unwrap();
        let res = tabby::PROGRAMParser::new().parse(&contents);
        assert!(res.is_ok());
        let my_ast = res.unwrap();
        let mut evaluator = AstEvaluator::new();
        evaluator.eval_program(my_ast);
    }

    #[test]
    #[should_panic]
    fn test_mat_dec_fail_1() {
        let filename = "./tests_compile/mat_dec_fail_1.tabby";
        let contents = fs::read_to_string(filename).unwrap();
        let res = tabby::PROGRAMParser::new().parse(&contents);
        assert!(res.is_ok());
        let my_ast = res.unwrap();
        let mut evaluator = AstEvaluator::new();
        evaluator.eval_program(my_ast);
    }

    #[test]
    #[should_panic]
    fn test_undec_arr_fail_1() {
        let filename = "./tests_compile/undec_arr_fail_1.tabby";
        let contents = fs::read_to_string(filename).unwrap();
        let res = tabby::PROGRAMParser::new().parse(&contents);
        assert!(res.is_ok());
        let my_ast = res.unwrap();
        let mut evaluator = AstEvaluator::new();
        evaluator.eval_program(my_ast);
    }

    #[test]
    #[should_panic]
    fn test_undec_mat_fail_1() {
        let filename = "./tests_compile/undec_mat_fail_1.tabby";
        let contents = fs::read_to_string(filename).unwrap();
        let res = tabby::PROGRAMParser::new().parse(&contents);
        assert!(res.is_ok());
        let my_ast = res.unwrap();
        let mut evaluator = AstEvaluator::new();
        evaluator.eval_program(my_ast);
    }

    #[test]
    fn test_quads_arr_ok_1() {
        let filename = "./tests_compile/quads_arr_ok_1.tabby";
        let contents = fs::read_to_string(filename).unwrap();
        let res = tabby::PROGRAMParser::new().parse(&contents);
        assert!(res.is_ok());
        let my_ast = res.unwrap();
        let mut evaluator = AstEvaluator::new();
        evaluator.eval_program(my_ast);
        assert_eq!(evaluator.quads.len(), 25);
        assert_eq!(evaluator.quads.get(0).unwrap(), &Quadruple::GoTo(1));
        // Read(n), i = 0, i < N, GoToF, i + 1
        assert_eq!(
            evaluator.quads.get(6).unwrap(),
            &Quadruple::Verify(("temp2".to_string(), LTEMP_START + LOCAL_INT_OFFSET), 100)
        );
        assert_eq!(
            evaluator.quads.get(7).unwrap(),
            &Quadruple::Op(
                "+".to_string(),
                ("temp2".to_string(), LTEMP_START + LOCAL_INT_OFFSET),
                (
                    (GLOBAL_START + GLOBAL_INT_OFFSET).to_string(),
                    CNST_START + LOCAL_INT_OFFSET
                ),
                ("temp3".to_string(), LTEMP_START + LOCAL_INT_OFFSET + 1),
            )
        );
        assert_eq!(
            evaluator.quads.get(8).unwrap(),
            &Quadruple::Deref(
                ("temp3".to_string(), LTEMP_START + LOCAL_INT_OFFSET + 1),
                ("temp4".to_string(), (LTEMP_START + LOCAL_INT_OFFSET + 2))
            )
        );
        assert_eq!(
            evaluator.quads.get(9).unwrap(),
            &Quadruple::Read(
                "Read".to_string(),
                ("temp4".to_string(), (LTEMP_START + LOCAL_INT_OFFSET + 2))
            )
        );
        assert_eq!(
            evaluator.quads.get(10).unwrap(),
            &Quadruple::Verify(
                ("i".to_string(), GLOBAL_START + GLOBAL_INT_OFFSET + 101),
                100
            )
        );
        assert_eq!(
            evaluator.quads.get(11).unwrap(),
            &Quadruple::Op(
                "+".to_string(),
                ("i".to_string(), GLOBAL_START + GLOBAL_INT_OFFSET + 101),
                (
                    (GLOBAL_START + GLOBAL_INT_OFFSET).to_string(),
                    CNST_START + LOCAL_INT_OFFSET
                ),
                ("temp5".to_string(), LTEMP_START + LOCAL_INT_OFFSET + 3),
            )
        );
        assert_eq!(
            evaluator.quads.get(12).unwrap(),
            &Quadruple::Deref(
                ("temp5".to_string(), LTEMP_START + LOCAL_INT_OFFSET + 3),
                ("temp6".to_string(), (LTEMP_START + LOCAL_INT_OFFSET + 4))
            )
        );
        assert_eq!(
            evaluator.quads.get(13).unwrap(),
            &Quadruple::Verify(
                ("i".to_string(), GLOBAL_START + GLOBAL_INT_OFFSET + 101),
                100
            )
        );
        assert_eq!(
            evaluator.quads.get(14).unwrap(),
            &Quadruple::Op(
                "+".to_string(),
                ("i".to_string(), GLOBAL_START + GLOBAL_INT_OFFSET + 101),
                (
                    (GLOBAL_START + GLOBAL_INT_OFFSET).to_string(),
                    CNST_START + LOCAL_INT_OFFSET
                ),
                ("temp7".to_string(), LTEMP_START + LOCAL_INT_OFFSET + 5),
            )
        );
        assert_eq!(
            evaluator.quads.get(15).unwrap(),
            &Quadruple::Deref(
                ("temp7".to_string(), LTEMP_START + LOCAL_INT_OFFSET + 5),
                ("temp8".to_string(), (LTEMP_START + LOCAL_INT_OFFSET + 6))
            )
        );
        assert_eq!(
            evaluator.quads.get(16).unwrap(),
            &Quadruple::Op(
                "+".to_string(),
                ("temp8".to_string(), LTEMP_START + LOCAL_INT_OFFSET + 6),
                ("1".to_string(), (CNST_START + LOCAL_INT_OFFSET + 1)),
                ("temp9".to_string(), LTEMP_START + LOCAL_INT_OFFSET + 7)
            )
        );

        assert_eq!(
            evaluator.quads.get(17).unwrap(),
            &Quadruple::Assign(
                "=".to_string(),
                ("temp9".to_string(), LTEMP_START + LOCAL_INT_OFFSET + 7),
                ("temp6".to_string(), (LTEMP_START + LOCAL_INT_OFFSET + 4))
            )
        );
    }

    #[test]
    fn test_quads_mat_ok_1() {
        let filename = "./tests_compile/quads_mat_ok_1.tabby";
        let contents = fs::read_to_string(filename).unwrap();
        let res = tabby::PROGRAMParser::new().parse(&contents);
        assert!(res.is_ok());
        let my_ast = res.unwrap();
        let mut evaluator = AstEvaluator::new();
        evaluator.eval_program(my_ast);
        assert_eq!(evaluator.quads.len(), 18);
        assert_eq!(evaluator.quads.get(0).unwrap(), &Quadruple::GoTo(1));
        assert_eq!(
            evaluator.quads.get(1).unwrap(),
            &Quadruple::Op(
                "+".to_string(),
                ("i".to_string(), GLOBAL_START + GLOBAL_INT_OFFSET + 26),
                ("1".to_string(), CNST_START + LOCAL_INT_OFFSET),
                ("temp1".to_string(), LTEMP_START + LOCAL_INT_OFFSET)
            )
        );
        assert_eq!(
            evaluator.quads.get(2).unwrap(),
            &Quadruple::Verify(("temp1".to_string(), LTEMP_START + LOCAL_INT_OFFSET), 5)
        );
        assert_eq!(
            evaluator.quads.get(3).unwrap(),
            &Quadruple::Op(
                "*".to_string(),
                ("2".to_string(), CNST_START + LOCAL_INT_OFFSET + 1),
                ("j".to_string(), GLOBAL_START + GLOBAL_INT_OFFSET + 25),
                ("temp2".to_string(), LTEMP_START + LOCAL_INT_OFFSET + 1),
            )
        );
        assert_eq!(
            evaluator.quads.get(4).unwrap(),
            &Quadruple::Op(
                "+".to_string(),
                ("temp2".to_string(), LTEMP_START + LOCAL_INT_OFFSET + 1),
                ("3".to_string(), CNST_START + LOCAL_INT_OFFSET + 2),
                ("temp3".to_string(), LTEMP_START + LOCAL_INT_OFFSET + 2),
            )
        );
        assert_eq!(
            evaluator.quads.get(5).unwrap(),
            &Quadruple::Verify(("temp3".to_string(), LTEMP_START + LOCAL_INT_OFFSET + 2), 5)
        );
        assert_eq!(
            evaluator.quads.get(6).unwrap(),
            &Quadruple::Op(
                "*".to_string(),
                ("temp1".to_string(), LTEMP_START + LOCAL_INT_OFFSET),
                ("5".to_string(), CNST_START + LOCAL_INT_OFFSET + 3),
                ("temp4".to_string(), LTEMP_START + LOCAL_INT_OFFSET + 3),
            )
        );
        assert_eq!(
            evaluator.quads.get(7).unwrap(),
            &Quadruple::Op(
                "+".to_string(),
                ("temp4".to_string(), LTEMP_START + LOCAL_INT_OFFSET + 3),
                ("temp3".to_string(), LTEMP_START + LOCAL_INT_OFFSET + 2),
                ("temp4".to_string(), LTEMP_START + LOCAL_INT_OFFSET + 3),
            )
        );
        assert_eq!(
            evaluator.quads.get(8).unwrap(),
            &Quadruple::Op(
                "+".to_string(),
                ("temp4".to_string(), LTEMP_START + LOCAL_INT_OFFSET + 3),
                (
                    (GLOBAL_START + GLOBAL_INT_OFFSET).to_string(),
                    CNST_START + LOCAL_INT_OFFSET + 4
                ),
                ("temp4".to_string(), LTEMP_START + LOCAL_INT_OFFSET + 3),
            )
        );
        assert_eq!(
            evaluator.quads.get(9).unwrap(),
            &Quadruple::Deref(
                ("temp4".to_string(), LTEMP_START + LOCAL_INT_OFFSET + 3),
                ("temp5".to_string(), (LTEMP_START + LOCAL_INT_OFFSET + 4))
            )
        );
        assert_eq!(
            evaluator.quads.get(10).unwrap(),
            &Quadruple::Assign(
                "=".to_string(),
                ("i".to_string(), GLOBAL_START + GLOBAL_INT_OFFSET + 26),
                ("temp5".to_string(), (LTEMP_START + LOCAL_INT_OFFSET + 4))
            )
        );
    }
}
