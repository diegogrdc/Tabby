// File used to define the evaluator
// of an AST, adding the nerve
// points needed to generate
// intermediate code, store
// directory functions,
// evaluate expressions, etc
use crate::ast;
use crate::dir_func::*;
use crate::dir_var::*;
use crate::tipo::*;

pub struct AstEvaluator {
    pub dir_func: DirFunc,
}

// Builder
impl AstEvaluator {
    pub fn new() -> AstEvaluator {
        AstEvaluator {
            dir_func: DirFunc::new(),
        }
    }
}

// Fns for each kind of structure, having access to all the information
impl AstEvaluator {
    pub fn eval_program(&mut self, program: Box<ast::Program>) -> bool {
        match *program {
            ast::Program::Program(id, vardecs, functions, tabby) => {
                // Generate global directory of vars
                // before adding row to dir func
                // as this cannot be mutated
                // after creation
                // due to borrowing rules
                // and immutable structs in Rust
                let mut glob_vars = DirVar::new();
                self.eval_var_decs(vardecs, &mut glob_vars);

                // Create our "global scope" function in
                // directory of functions, with id name,
                // type void, and an our variable dir
                self.dir_func.insert(
                    id.clone(),
                    FuncInfo {
                        tipo: Tipo::Program,
                        dir_var: glob_vars,
                    },
                );

                self.eval_functions(functions);
                self.eval_tabby(tabby);
            }
        };
        true
    }

    pub fn eval_block(&mut self, block: Box<ast::Block>) -> bool {
        match *block {
            ast::Block::Block(statutes) => {
                self.eval_statutes(statutes);
            }
        };
        true
    }

    pub fn eval_statutes(&mut self, statutes: Box<ast::Statutes>) -> bool {
        match *statutes {
            ast::Statutes::Statutes(statute_vec) => {
                for statute in statute_vec {
                    self.eval_statute(statute);
                }
            }
        };
        true
    }

    pub fn eval_statute(&mut self, statute: Box<ast::Statute>) -> bool {
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

    pub fn eval_var_decs(&mut self, vardecs: Box<ast::Vardecs>, vars: &mut DirVar) -> bool {
        match *vardecs {
            ast::Vardecs::Vardecs(vardec_vec) => {
                for vardec in vardec_vec {
                    self.eval_var_dec(vardec, vars);
                }
            }
        };
        true
    }

    pub fn eval_var_dec(&mut self, vardec: Box<ast::Vardec>, vars: &mut DirVar) -> bool {
        match *vardec {
            ast::Vardec::Vars(typ, id_vec) => {
                for id in id_vec {
                    self.check_multiple_dec_var(&id, &vars);
                    vars.insert(
                        id,
                        VarInfo {
                            tipo: tipo_from_type(&typ),
                        },
                    );
                }
            }
            ast::Vardec::Arr(typ, id, sz) => {
                self.check_multiple_dec_var(&id, &vars);
                vars.insert(
                    id,
                    VarInfo {
                        tipo: arr_tipo_from_type(&typ, sz),
                    },
                );
            }
            ast::Vardec::Mat(typ, id, sz1, sz2) => {
                self.check_multiple_dec_var(&id, &vars);
                vars.insert(
                    id,
                    VarInfo {
                        tipo: mat_tipo_from_type(&typ, sz1, sz2),
                    },
                );
            }
        };
        true
    }

    pub fn eval_functions(&mut self, functions: Box<ast::Functions>) -> bool {
        match *functions {
            ast::Functions::Fns(function_vec) => {
                for function in function_vec {
                    self.eval_function(function);
                }
            }
        };
        true
    }

    pub fn eval_function(&mut self, function: Box<ast::Function>) -> bool {
        // For any function, we must create its entry on
        // Directory of Functions, and create its local
        // variables (params) in a Directory of Variables
        let mut vars = DirVar::new();
        let fn_id: String;
        let fn_typ: Tipo;

        match *function {
            ast::Function::FnParams(typ, id, params, block) => {
                fn_id = id.clone();
                fn_typ = tipo_from_type(&typ);

                self.eval_params(params, &mut vars);
                self.eval_block(block);
            }
            ast::Function::FnVoidParams(id, params, block) => {
                fn_id = id.clone();
                fn_typ = Tipo::Void;

                self.eval_params(params, &mut vars);
                self.eval_block(block);
            }
            ast::Function::Fn(typ, id, block) => {
                fn_id = id.clone();
                fn_typ = tipo_from_type(&typ);

                self.eval_block(block);
            }
            ast::Function::FnVoid(id, block) => {
                fn_id = id.clone();
                fn_typ = Tipo::Void;

                self.eval_block(block);
            }
        };

        self.check_multiple_dec_fns(&fn_id);
        self.dir_func.insert(
            fn_id,
            FuncInfo {
                tipo: fn_typ,
                dir_var: vars,
            },
        );
        true
    }

    pub fn eval_params(&mut self, params: Box<ast::Params>, vars: &mut DirVar) -> bool {
        // Remember arrs and mats are sent as references
        match *params {
            ast::Params::Param(typ, id) => {
                self.add_param(id, tipo_from_type(&typ), vars);
            }
            ast::Params::ArrParam(_typ, id) => {
                self.add_param(id, Tipo::ParamRef, vars);
            }
            ast::Params::ParamAnd(typ, id, params) => {
                self.add_param(id, tipo_from_type(&typ), vars);

                self.eval_params(params, vars);
            }
            ast::Params::ArrParamAnd(_typ, id, params) => {
                self.add_param(id, Tipo::ParamRef, vars);

                self.eval_params(params, vars);
            }
        };
        true
    }

    pub fn add_param(&mut self, id: String, typ: Tipo, vars: &mut DirVar) {
        self.check_multiple_dec_var(&id, &vars);
        vars.insert(id, VarInfo { tipo: typ });
    }

    pub fn eval_tabby(&mut self, tabby: Box<ast::Tabby>) -> bool {
        match *tabby {
            ast::Tabby::Tabby(block) => {
                self.eval_block(block);
            }
        };
        true
    }

    pub fn eval_read(&mut self, read: Box<ast::Read>) -> bool {
        match *read {
            ast::Read::Read(var) => {
                self.eval_variable(var);
            }
        };
        true
    }

    pub fn eval_assignment(&mut self, assignment: Box<ast::Assignment>) -> bool {
        match *assignment {
            ast::Assignment::Assign(var, exp) => {
                self.eval_variable(var);
                self.eval_exp(exp);
            }
        };
        true
    }

    pub fn eval_print(&mut self, print: Box<ast::Print>) -> bool {
        match *print {
            ast::Print::Print(print_vars) => {
                self.eval_print_vars(print_vars);
            }
        };
        true
    }

    pub fn eval_print_vars(&mut self, print_vars: Box<ast::PrintVars>) -> bool {
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

    pub fn eval_variable(&mut self, variable: Box<ast::Variable>) -> bool {
        match *variable {
            ast::Variable::Id(_id) => {}
            ast::Variable::Arr(_id, _sz1) => {}
            ast::Variable::Mat(_id, _sz1, _sz2) => {}
        };
        true
    }

    pub fn eval_call(&mut self, call: Box<ast::Call>) -> bool {
        match *call {
            ast::Call::Call(_id, exp_vec) => {
                for exp in exp_vec {
                    self.eval_exp(exp);
                }
            }
        };
        true
    }

    pub fn eval_ciclew(&mut self, ciclew: Box<ast::Ciclew>) -> bool {
        match *ciclew {
            ast::Ciclew::While(exp, block) => {
                self.eval_exp(exp);
                self.eval_block(block);
            }
        };
        true
    }

    pub fn eval_ciclef(&mut self, ciclef: Box<ast::Ciclef>) -> bool {
        match *ciclef {
            ast::Ciclef::For(exp, assignment, block) => {
                self.eval_exp(exp);
                self.eval_assignment(assignment);
                self.eval_block(block);
            }
        };
        true
    }

    pub fn eval_cond(&mut self, cond: Box<ast::Cond>) -> bool {
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

    pub fn eval_return(&mut self, ret: Box<ast::Return>) -> bool {
        match *ret {
            ast::Return::Return(exp) => {
                self.eval_exp(exp);
            }
        };
        true
    }

    pub fn eval_exp(&mut self, exp: Box<ast::Exp>) -> bool {
        match *exp {
            ast::Exp::Texp(texp_vec) => {
                for texp in texp_vec {
                    self.eval_texp(texp);
                }
            }
        };
        true
    }

    pub fn eval_texp(&mut self, texp: Box<ast::Texp>) -> bool {
        match *texp {
            ast::Texp::Gexp(gexp_vec) => {
                for gexp in gexp_vec {
                    self.eval_gexp(gexp);
                }
            }
        };
        true
    }

    pub fn eval_gexp(&mut self, gexp: Box<ast::Gexp>) -> bool {
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

    pub fn eval_mexp(&mut self, mexp: Box<ast::Mexp>) -> bool {
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

    pub fn eval_term(&mut self, term: Box<ast::Term>) -> bool {
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

    pub fn eval_fact(&mut self, fact: Box<ast::Fact>) -> bool {
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

    pub fn check_multiple_dec_var(&self, id: &String, vars: &DirVar) {
        if let Some(_) = vars.get(id) {
            panic!(
                "ERROR: Multiple variable declaration: \"{}\" was declared multiple times",
                id
            );
        }
    }

    pub fn check_multiple_dec_fns(&self, id: &String) {
        if let Some(_) = self.dir_func.get(id) {
            panic!(
                "ERROR: Multiple function declaration: \"{}\" was declared multiple times",
                id
            );
        }
    }
}
