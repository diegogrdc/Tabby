// File used to define enums 
// that will be used to store 
// and build AST with tokens 
// parsed by Parser

// Non Terminals 

pub PROGRAM: String = {
    "Program" <id:ID> ";" <v:VARDECS> <f:FUNCTIONS> <t:TABBY> => "ok".to_string(),
};

pub BLOCK: String = {
    "{" <s:STATUTES> "}" => s,
};

pub STATUTES: String  = {
    <_v:(<STATUTE>)*> => "ok".to_string(),
};

pub STATUTE: String  = {
    <a:ASSIGNMENT> ";" => a,
    <c:CALL> ";" => c,
    <r:READ> ";" => r,
    <p:PRINT> ";"=> p,
    <r:RETURN> ";" => r,
    <w:CICLEW> => w,
    <f:CICLEF> => f,
    <c:COND> => c,
};

pub VARDECS: String  = {
    <_v:(VARDEC)*> => "ok".to_string(),
};

pub VARDEC: String  = {
    "Var" TYPE <_v:(ID ",")*> <id:ID> ";" => id,
    "Var" "Arr" TYPE <id:ID> "[" INT "]" ";" => id,
    "Var" "Arr" TYPE <id:ID> "[" INT "]" "[" INT "]" ";" => id,
};

pub FUNCTIONS: String = {
    <_v:(<FUNCTION>)*> => "ok".to_string(),
};

pub FUNCTION: String = {
    "Fn" TYPE <id:ID> "(" PARAMS ")" BLOCK => id,
    "Fn" "Void" <id:ID> "(" PARAMS ")" BLOCK => id,
};

pub PARAMS: String = {
    <_v:("Arr"? <TYPE> <ID> ",")*> <e:("Arr"? <TYPE> <ID>)?> => "ok".to_string(),
};

pub TABBY: String = {
    "Tabby" "(" ")" <b:BLOCK> => b,
};

pub READ: String = {
    "Read" "(" <v:VARIABLE> ")" => v,
};

pub ASSIGNMENT: String = {
    <v:VARIABLE> "=" EXP => v,
};

pub PRINT: String = {
    "Write" "(" <p:PRINTVARS> ")" => p,
};

pub PRINTVARS: String = {
    <e:EXP> => e,
    <s:STRINGLIT> => s,
    <e:EXP> "," PRINTVARS => e,
    <s:STRINGLIT> "," PRINTVARS => s,
}

pub VARIABLE: String = {
    <id:ID> => id,
    <id:ID> "[" EXP "]" => id,
    <id:ID> "[" EXP "]" "[" EXP "]" => id,
};

pub CALL: String = {
    <id:ID> "(" <_v:(EXP ",")*> EXP? ")" => id,
};

pub CICLEW: String = {
    "While" "(" EXP ")" <b:BLOCK> => b,
};

pub CICLEF: String = {
    "For" "(" EXP ";" <a:ASSIGNMENT> ")" BLOCK => a,
};

pub COND: String = {
    "If" "(" EXP ")" <b:BLOCK> => b,
    "If" "(" EXP ")" <b:BLOCK> "Else" BLOCK => b,
};

pub RETURN: String = {
    "Return" <e:EXP> => e,
};

pub COMP: String = {
    ">" => ">".to_string(),
    "<" => "<".to_string(),
    ">=" => ">=".to_string(),
    "<=" => "<=".to_string(),
    "!=" => "!=".to_string(),
    "==" => "==".to_string(),
};

pub TYPE: String = {
    "Int" => "Int".to_string(),
    "Float" => "Float".to_string(),
    "Char" => "Char".to_string(),
    "Bool" => "Bool".to_string(),
};

pub EXP: String = {
    <_v:(TEXP "Or")*> <t:TEXP> => t,
};

pub TEXP: String = {
    <_v:(GEXP "And")*> <t:GEXP> => t,
};

pub GEXP: String = {
    <m:MEXP> => m,
    <m:MEXP> COMP MEXP => m, 
};

pub MEXP: String = {
    <t:TERM> => t,
    MEXP "+" <t:TERM> => t,
    MEXP "-" <t:TERM> => t,
};

pub TERM: String = {
    <f:FACT> => f,
    TERM "*" <f:FACT> => f,
    TERM "/" <f:FACT> => f,
};

pub enum Fact {
    Parentheses(Box<Expr>),
    Int(i32),
    Float(f64),
    Bool(bool),
}

pub FACT: String = {
    "(" <e:EXP> ")" => e,
    <i:INT> => i.to_string(),
    <f:FLOAT> => f.to_string(),
    <b:BOOL> => b.to_string(),
    <c:CHAR> => c.to_string(),
    <c:CALL> => c,
    <v:VARIABLE> => v,
}
