// File used to define enums
// that will be used to store
// and build AST with tokens
// parsed by Parser

#[derive(Debug)]
pub enum Program {
    Program(String, Box<Vardecs>, Box<Functions>, Box<Tabby>),
}

#[derive(Debug)]
pub enum Block {
    Block(Box<Statutes>),
}

#[derive(Debug)]
pub enum Statutes {
    Statutes(Vec<Box<Statute>>),
}

#[derive(Debug)]
pub enum Statute {
    Assignment(Box<Assignment>),
    Call(Box<Call>),
    Read(Box<Read>),
    Print(Box<Print>),
    Return(Box<Return>),
    Ciclew(Box<Ciclew>),
    Ciclef(Box<Ciclef>),
    Cond(Box<Cond>),
    Plot(Box<Plot>),
}

#[derive(Debug)]
pub enum Vardecs {
    Vardecs(Vec<Box<Vardec>>),
}

#[derive(Debug)]
pub enum Vardec {
    Vars(Type, Vec<String>),
    Arr(Type, String, i32),
    Mat(Type, String, i32, i32),
}

#[derive(Debug)]
pub enum Functions {
    Fns(Vec<Box<Function>>),
}

#[derive(Debug)]
pub enum Function {
    FnParams(Type, String, Box<Params>, Box<Vardecs>, Box<Block>),
    FnVoidParams(String, Box<Params>, Box<Vardecs>, Box<Block>),
    Fn(Type, String, Box<Vardecs>, Box<Block>),
    FnVoid(String, Box<Vardecs>, Box<Block>),
}

#[derive(Debug)]
pub enum Params {
    Param(Type, String),
    ParamAnd(Type, String, Box<Params>),
}

#[derive(Debug)]
pub enum Tabby {
    Tabby(Box<Block>),
}

#[derive(Debug)]
pub enum Read {
    Read(Box<Variable>),
}

#[derive(Debug)]
pub enum Assignment {
    Assign(Box<Variable>, Box<Exp>),
}

#[derive(Debug)]
pub enum Print {
    Print(Box<PrintVars>),
}

#[derive(Debug)]
pub enum PrintVars {
    Exp(Box<Exp>),
    StrLit(String),
    ExpPV(Box<Exp>, Box<PrintVars>),
    StrLitPV(String, Box<PrintVars>),
}

#[derive(Debug)]
pub enum Variable {
    Id(String),
    Arr(String, Box<Exp>),
    Mat(String, Box<Exp>, Box<Exp>),
}

#[derive(Debug)]
pub enum Call {
    Call(String, Vec<Box<Exp>>),
}

#[derive(Debug)]
pub enum Ciclew {
    While(Box<Exp>, Box<Block>),
}

#[derive(Debug)]
pub enum Ciclef {
    For(Box<Exp>, Box<Assignment>, Box<Block>),
}

#[derive(Debug)]
pub enum Cond {
    If(Box<Exp>, Box<Block>),
    IfElse(Box<Exp>, Box<Block>, Box<Block>),
}

#[derive(Debug)]
pub enum Return {
    Return(Box<Exp>),
}

#[derive(Debug)]
pub enum Comp {
    Greater,
    Smaller,
    GreaterEq,
    SmallerEq,
    NotEqual,
    Equal,
}

#[derive(Debug)]
pub enum Type {
    Int,
    Float,
    Bool,
}

#[derive(Debug)]
pub enum Exp {
    Texp(Vec<Box<Texp>>),
}

#[derive(Debug)]
pub enum Texp {
    Gexp(Vec<Box<Gexp>>),
}

#[derive(Debug)]
pub enum Gexp {
    Mexp(Box<Mexp>),
    Comp(Box<Mexp>, Comp, Box<Mexp>),
}

#[derive(Debug)]
pub enum Mexp {
    Term(Box<Term>),
    Sum(Box<Mexp>, Box<Term>),
    Sub(Box<Mexp>, Box<Term>),
}

#[derive(Debug)]
pub enum Term {
    Fact(Box<Fact>),
    Mul(Box<Term>, Box<Fact>),
    Div(Box<Term>, Box<Fact>),
}

#[derive(Debug)]
pub enum Fact {
    Parentheses(Box<Exp>),
    Int(i32),
    Float(f64),
    Bool(bool),
    Call(Box<Call>),
    Variable(Box<Variable>),
    Statistics(Box<Statistics>),
    RandInt(),
    RandFlt(),
}

#[derive(Debug)]
pub enum Statistics {
    MinFlt(String, Box<Exp>, Box<Exp>),
    MinInt(String, Box<Exp>, Box<Exp>),
    MaxFlt(String, Box<Exp>, Box<Exp>),
    MaxInt(String, Box<Exp>, Box<Exp>),
    RangeFlt(String, Box<Exp>, Box<Exp>),
    RangeInt(String, Box<Exp>, Box<Exp>),
    MeanFlt(String, Box<Exp>, Box<Exp>),
    MeanInt(String, Box<Exp>, Box<Exp>),
    ModeInt(String, Box<Exp>, Box<Exp>),
    MedianFlt(String, Box<Exp>, Box<Exp>),
    MedianInt(String, Box<Exp>, Box<Exp>),
    StdDevFlt(String, Box<Exp>, Box<Exp>),
    StdDevInt(String, Box<Exp>, Box<Exp>),
    VarianceFlt(String, Box<Exp>, Box<Exp>),
    VarianceInt(String, Box<Exp>, Box<Exp>),
}

#[derive(Debug)]
pub enum Plot {
    Histogram(String, i32, Box<Exp>, String),
    Line(String, String, Box<Exp>, String),
    Scatter(String, String, Box<Exp>, String),
}
