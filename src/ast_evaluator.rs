// File used to define the evaluator
// of an AST, adding the nerve
// points needed to generate
// intermediate code, store
// directory functions,
// evaluate expressions, etc
use crate::ast;

pub struct AstEvaluator {
    pub var: i32,
}

// Builder
impl AstEvaluator {
    pub fn new() -> AstEvaluator {
        AstEvaluator { var: 3 }
    }
}

// Fns for each kind of structure, having access to all the information
impl AstEvaluator {
    pub fn eval_program(&self, program: Box<ast::Program>) -> bool {
        match *program {
            ast::Program::Program(_id, vardecs, functions, tabby) => {
                self.eval_var_decs(vardecs);
                self.eval_functions(functions);
                self.eval_tabby(tabby);
            }
        };
        true
    }

    pub fn eval_block(&self, block: Box<ast::Block>) -> bool {
        match *block {
            ast::Block::Block(statutes) => {
                self.eval_statutes(statutes);
            }
        };
        true
    }

    pub fn eval_statutes(&self, statutes: Box<ast::Statutes>) -> bool {
        match *statutes {
            ast::Statutes::Statutes(statute_vec) => {
                for statute in statute_vec {
                    self.eval_statute(statute);
                }
            }
        };
        true
    }

    pub fn eval_statute(&self, statute: Box<ast::Statute>) -> bool {
        match *statute {
            ast::Statute::Assignment(assignment) => {
                self.eval_assignment(assignment);
            }
            ast::Statute::Call(call) => {
                self.eval_call(call);
            }
            ast::Statute::Read(read) => {
                self.eval_read(read);
            }
            ast::Statute::Print(print) => {
                self.eval_print(print);
            }
            ast::Statute::Return(ret) => {
                self.eval_return(ret);
            }
            ast::Statute::Ciclew(ciclew) => {
                self.eval_ciclew(ciclew);
            }
            ast::Statute::Ciclef(ciclef) => {
                self.eval_ciclef(ciclef);
            }
            ast::Statute::Cond(cond) => {
                self.eval_cond(cond);
            }
        };
        true
    }

    pub fn eval_var_decs(&self, vardecs: Box<ast::Vardecs>) -> bool {
        match *vardecs {
            ast::Vardecs::Vardecs(vardec_vec) => {
                for vardec in vardec_vec {
                    self.eval_var_dec(vardec);
                }
            }
        };
        true
    }

    pub fn eval_var_dec(&self, vardec: Box<ast::Vardec>) -> bool {
        match *vardec {
            ast::Vardec::Vars(_typ, _id_vec) => {}
            ast::Vardec::Arr(_typ, _id, _s1) => {}
            ast::Vardec::Mat(_typ, _id, _s1, _s2) => {}
        };
        true
    }

    pub fn eval_functions(&self, functions: Box<ast::Functions>) -> bool {
        match *functions {
            ast::Functions::Fns(function_vec) => {
                for function in function_vec {
                    self.eval_function(function);
                }
            }
        };
        true
    }

    pub fn eval_function(&self, function: Box<ast::Function>) -> bool {
        match *function {
            ast::Function::FnParams(_typ, _id, params, block) => {
                self.eval_params(params);
                self.eval_block(block);
            }
            ast::Function::FnVoidParams(_id, params, block) => {
                self.eval_params(params);
                self.eval_block(block);
            }
            ast::Function::Fn(_typ, _id, block) => {
                self.eval_block(block);
            }
            ast::Function::FnVoid(_id, block) => {
                self.eval_block(block);
            }
        };
        true
    }

    pub fn eval_params(&self, params: Box<ast::Params>) -> bool {
        match *params {
            ast::Params::Param(_typ, _id) => {}
            ast::Params::ArrParam(_typ, _id) => {}
            ast::Params::ParamAnd(_typ, _id, params) => {
                self.eval_params(params);
            }
            ast::Params::ArrParamAnd(_typ, _id, params) => {
                self.eval_params(params);
            }
        };
        true
    }

    pub fn eval_tabby(&self, tabby: Box<ast::Tabby>) -> bool {
        match *tabby {
            ast::Tabby::Tabby(block) => {
                self.eval_block(block);
            }
        };
        true
    }

    pub fn eval_read(&self, read: Box<ast::Read>) -> bool {
        match *read {
            ast::Read::Read(var) => {
                self.eval_variable(var);
            }
        };
        true
    }

    pub fn eval_assignment(&self, assignment: Box<ast::Assignment>) -> bool {
        match *assignment {
            ast::Assignment::Assign(var, exp) => {
                self.eval_variable(var);
                self.eval_exp(exp);
            }
        };
        true
    }

    pub fn eval_print(&self, print: Box<ast::Print>) -> bool {
        match *print {
            ast::Print::Print(print_vars) => {
                self.eval_print_vars(print_vars);
            }
        };
        true
    }

    pub fn eval_print_vars(&self, print_vars: Box<ast::PrintVars>) -> bool {
        match *print_vars {
            ast::PrintVars::Exp(exp) => {
                self.eval_exp(exp);
            }
            ast::PrintVars::StrLit(_sl) => {}
            ast::PrintVars::ExpPV(exp, pv) => {
                self.eval_exp(exp);
                self.eval_print_vars(pv);
            }
            ast::PrintVars::StrLitPV(_sl, pv) => {
                self.eval_print_vars(pv);
            }
        };
        true
    }

    pub fn eval_variable(&self, variable: Box<ast::Variable>) -> bool {
        match *variable {
            ast::Variable::Id(_id) => {}
            ast::Variable::Arr(_id, _sz1) => {}
            ast::Variable::Mat(_id, _sz1, _sz2) => {}
        };
        true
    }

    pub fn eval_call(&self, call: Box<ast::Call>) -> bool {
        match *call {
            ast::Call::Call(_id, exp_vec) => {
                for exp in exp_vec {
                    self.eval_exp(exp);
                }
            }
        };
        true
    }

    pub fn eval_ciclew(&self, ciclew: Box<ast::Ciclew>) -> bool {
        match *ciclew {
            ast::Ciclew::While(exp, block) => {
                self.eval_exp(exp);
                self.eval_block(block);
            }
        };
        true
    }

    pub fn eval_ciclef(&self, ciclef: Box<ast::Ciclef>) -> bool {
        match *ciclef {
            ast::Ciclef::For(exp, assignment, block) => {
                self.eval_exp(exp);
                self.eval_assignment(assignment);
                self.eval_block(block);
            }
        };
        true
    }

    pub fn eval_cond(&self, cond: Box<ast::Cond>) -> bool {
        match *cond {
            ast::Cond::If(exp, block) => {
                self.eval_exp(exp);
                self.eval_block(block);
            }
            ast::Cond::IfElse(exp, block_if, block_else) => {
                self.eval_exp(exp);
                self.eval_block(block_if);
                self.eval_block(block_else);
            }
        };
        true
    }

    pub fn eval_return(&self, ret: Box<ast::Return>) -> bool {
        match *ret {
            ast::Return::Return(exp) => {
                self.eval_exp(exp);
            }
        };
        true
    }

    pub fn eval_exp(&self, exp: Box<ast::Exp>) -> bool {
        match *exp {
            ast::Exp::Texp(texp_vec) => {
                for texp in texp_vec {
                    self.eval_texp(texp);
                }
            }
        };
        true
    }

    pub fn eval_texp(&self, texp: Box<ast::Texp>) -> bool {
        match *texp {
            ast::Texp::Gexp(gexp_vec) => {
                for gexp in gexp_vec {
                    self.eval_gexp(gexp);
                }
            }
        };
        true
    }

    pub fn eval_gexp(&self, gexp: Box<ast::Gexp>) -> bool {
        match *gexp {
            ast::Gexp::Mexp(mexp) => {
                self.eval_mexp(mexp);
            }
            ast::Gexp::Comp(mexp1, _comp, mexp2) => {
                self.eval_mexp(mexp1);
                self.eval_mexp(mexp2);
            }
        };
        true
    }

    pub fn eval_mexp(&self, mexp: Box<ast::Mexp>) -> bool {
        match *mexp {
            ast::Mexp::Term(term) => {
                self.eval_term(term);
            }
            ast::Mexp::Sum(mexp, term) => {
                self.eval_mexp(mexp);
                self.eval_term(term);
            }
            ast::Mexp::Sub(mexp, term) => {
                self.eval_mexp(mexp);
                self.eval_term(term);
            }
        };
        true
    }

    pub fn eval_term(&self, term: Box<ast::Term>) -> bool {
        match *term {
            ast::Term::Fact(fact) => {
                self.eval_fact(fact);
            }
            ast::Term::Mul(term, fact) => {
                self.eval_term(term);
                self.eval_fact(fact);
            }
            ast::Term::Div(term, fact) => {
                self.eval_term(term);
                self.eval_fact(fact);
            }
        };
        true
    }

    pub fn eval_fact(&self, fact: Box<ast::Fact>) -> bool {
        match *fact {
            ast::Fact::Parentheses(exp) => {
                self.eval_exp(exp);
            }
            ast::Fact::Int(_) => {}
            ast::Fact::Float(_) => {}
            ast::Fact::Bool(_) => {}
            ast::Fact::Call(call) => {
                self.eval_call(call);
            }
            ast::Fact::Variable(var) => {
                self.eval_variable(var);
            }
        };
        true
    }
}
