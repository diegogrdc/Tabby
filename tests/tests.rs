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

        assert!(tabby::FACTParser::new().parse("((wow)").is_err());
    }
}
