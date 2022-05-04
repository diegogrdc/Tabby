// File used to define the evaluator
// of an AST, adding the nerve
// points needed to generate
// intermediate code, store
// directory functions,
// evaluate expressions, etc
use crate::ast;
use crate::dir_func::*;
use crate::dir_var::*;
use crate::quadruples::IdAddr;
use crate::quadruples::Quadruple;
use crate::semantic_cube::*;
use crate::tipo::*;
use crate::vir_mem::*;

/*
Parameters used in our AST Evaluator
where we generate IC (Intermediate Code)
and check semantics
Params:
- dir_func
    Directory Function, where
    we store important information about functions
    and their variables
- sem_cube
    Semantic cube, generated to check semantic
    operations are correct and used to detect
    type mismatchs
- vir_mem_alloc
    Allocator for virtual memory. It takes care
    of memory logic and addresses used in quadruples
    generation
- st_ops
    Stack used to store operation pending eval
    in IC generation
- st_vals
    Stack used to store values pending eval
    in IC generation
- st_tips
    Stack used to store value types pending eval
    in IC generation
- quads
    List of Quadruples that is generated by the AST
- next_cnt
    Counter to index temp variables used in
    IC Code Generation
- glob_scope
    Optional string with the name of the current
    global scope to fetch fn and global vars names
    and types, to validate their usage
- loc_scope
    Optional string with name of the current local
    scope (function) to fetch local var names and types
    to validate their usage

*/
pub struct AstEvaluator {
    pub dir_func: DirFunc,
    pub sem_cube: SemanticCube,
    pub vir_mem_alloc: VirMemAllocator,
    pub st_ops: Vec<String>,
    pub st_vals: Vec<String>,
    pub st_tips: Vec<Tipo>,
    pub quads: Vec<Quadruple>,
    pub next_cnt: i32,
    pub glob_scope: Option<String>,
    pub loc_scope: Option<String>,
}

// ASTEvaluator Builder
impl AstEvaluator {
    pub fn new() -> AstEvaluator {
        AstEvaluator {
            dir_func: DirFunc::new(),
            sem_cube: SemanticCube::new(),
            vir_mem_alloc: VirMemAllocator::new(),
            st_ops: Vec::new(),
            st_vals: Vec::new(),
            st_tips: Vec::new(),
            quads: Vec::new(),
            next_cnt: 1,
            glob_scope: None,
            loc_scope: None,
        }
    }
}

/*
Implementation of AST evaluation and
neuralgic points implementation to
check semantics and generate IC code
*/
impl AstEvaluator {
    pub fn eval_program(&mut self, program: Box<ast::Program>) -> bool {
        match *program {
            ast::Program::Program(id, vardecs, functions, tabby) => {
                // Set global scope name
                self.glob_scope = Some(id.clone());

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
                // This call adds an extra
                // call element in our stack
                // that we need to delete to avoid
                // garbage accumulating here
                self.st_vals.pop();
                self.st_tips.pop();
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
                            addr: self.vir_mem_alloc.get_global_addr(&tipo_from_type(&typ)),
                        },
                    );
                }
            }
            ast::Vardec::Arr(typ, id, _sz) => {
                self.check_multiple_dec_var(&id, &vars);
                vars.insert(
                    id,
                    VarInfo {
                        tipo: tipo_from_type(&typ),
                        addr: self.vir_mem_alloc.get_global_addr(&tipo_from_type(&typ)),
                    },
                );
            }
            ast::Vardec::Mat(typ, id, _sz1, _sz2) => {
                self.check_multiple_dec_var(&id, &vars);
                vars.insert(
                    id,
                    VarInfo {
                        tipo: tipo_from_type(&typ),
                        addr: self.vir_mem_alloc.get_global_addr(&tipo_from_type(&typ)),
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
                    self.vir_mem_alloc.reset_local();
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
        let fn_block: Box<ast::Block>;

        // Eval params when needed and get
        // relevant info like id, type and block
        match *function {
            ast::Function::FnParams(typ, id, params, block) => {
                fn_id = id.clone();
                fn_typ = tipo_from_type(&typ);
                fn_block = block;

                self.eval_params(params, &mut vars);
            }
            ast::Function::FnVoidParams(id, params, block) => {
                fn_id = id.clone();
                fn_typ = Tipo::Void;
                fn_block = block;

                self.eval_params(params, &mut vars);
            }
            ast::Function::Fn(typ, id, block) => {
                fn_id = id.clone();
                fn_typ = tipo_from_type(&typ);
                fn_block = block;
            }
            ast::Function::FnVoid(id, block) => {
                fn_id = id.clone();
                fn_typ = Tipo::Void;
                fn_block = block;
            }
        };

        // Check fn name is unique
        self.check_multiple_dec_fns(&fn_id);
        // Create local var table
        self.dir_func.insert(
            fn_id.clone(),
            FuncInfo {
                tipo: fn_typ,
                dir_var: vars,
            },
        );
        // Set current local scope
        self.loc_scope = Some(fn_id);
        // Eval fn block
        self.eval_block(fn_block);
        // Remove current local scope
        self.loc_scope = None;
        true
    }

    pub fn eval_params(&mut self, params: Box<ast::Params>, vars: &mut DirVar) -> bool {
        // Remember arrs and mats are sent as references
        match *params {
            ast::Params::Param(typ, id) => {
                self.add_param(id, tipo_from_type(&typ), vars);
            }
            ast::Params::ArrParam(typ, id) => {
                self.add_param(id, tipo_from_type(&typ), vars);
            }
            ast::Params::ParamAnd(typ, id, params) => {
                self.add_param(id, tipo_from_type(&typ), vars);

                self.eval_params(params, vars);
            }
            ast::Params::ArrParamAnd(typ, id, params) => {
                self.add_param(id, tipo_from_type(&typ), vars);

                self.eval_params(params, vars);
            }
        };
        true
    }

    pub fn add_param(&mut self, id: String, tip: Tipo, vars: &mut DirVar) {
        self.check_multiple_dec_var(&id, &vars);
        vars.insert(
            id,
            VarInfo {
                tipo: tip.clone(),
                addr: self.vir_mem_alloc.get_local_addr(&tip),
            },
        );
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
                // Create the quadruple for this read
                // and clean info from stacks
                let id = self.st_vals.pop().unwrap();
                let tip = self.st_tips.pop().unwrap();
                let id_addr = self.get_id_addr(&id, &tip);
                self.quads
                    .push(Quadruple::Read("Read".to_string(), id_addr));
            }
        };
        true
    }

    pub fn eval_assignment(&mut self, assignment: Box<ast::Assignment>) -> bool {
        match *assignment {
            ast::Assignment::Assign(var, exp) => {
                self.eval_variable(var);
                // Push operator
                self.st_ops.push("=".to_string());
                // Solve exp
                self.eval_exp(exp);
                // Create quadruple
                assert!(self.st_ops.last().unwrap() == &"=".to_string());
                self.st_ops.pop();
                let rt_val: String = self.st_vals.pop().unwrap();
                let lt_val: String = self.st_vals.pop().unwrap();
                let rt_tipo: Tipo = self.st_tips.pop().unwrap();
                let lt_tipo: Tipo = self.st_tips.pop().unwrap();
                // Get result type to assure no type mismatch
                self.get_result_tipo(&lt_tipo, &rt_tipo, "=");
                let lt_id_addr = self.get_id_addr(&lt_val, &lt_tipo);
                let rt_id_addr = self.get_id_addr(&rt_val, &rt_tipo);
                self.quads
                    .push(Quadruple::Assign("=".to_string(), rt_id_addr, lt_id_addr));
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
                // Print quad after eval
                let id = self.st_vals.pop().unwrap();
                let tip = self.st_tips.pop().unwrap();
                let id_addr = self.get_id_addr(&id, &tip);
                self.quads
                    .push(Quadruple::Print("Print".to_string(), id_addr));
            }
            ast::PrintVars::StrLit(sl) => {
                // Print string lit
                let id_addr = self.get_id_addr(&format!("\"{}", sl), &Tipo::StrLit);
                self.quads
                    .push(Quadruple::Print("PrintSL".to_string(), id_addr));
            }
            ast::PrintVars::ExpPV(exp, pv) => {
                self.eval_exp(exp);
                // Print quad after eval
                let id = self.st_vals.pop().unwrap();
                let tip = self.st_tips.pop().unwrap();
                let id_addr = self.get_id_addr(&id, &tip);
                self.quads
                    .push(Quadruple::Print("Print".to_string(), id_addr));
                // Eval others
                self.eval_print_vars(pv);
            }
            ast::PrintVars::StrLitPV(sl, pv) => {
                // Print string lit
                let id_addr = self.get_id_addr(&format!("\"{}", sl), &Tipo::StrLit);
                self.quads
                    .push(Quadruple::Print("PrintSL".to_string(), id_addr));
                self.eval_print_vars(pv);
            }
        };
        true
    }

    pub fn eval_variable(&mut self, variable: Box<ast::Variable>) -> bool {
        match *variable {
            ast::Variable::Id(id) => {
                let tipo = self.get_var_tipo_from_id(&id);
                self.st_vals.push(id.clone());
                self.st_tips.push(tipo);
            }
            // TODO
            ast::Variable::Arr(_id, _esz1) => {}
            // TODO
            ast::Variable::Mat(_id, _esz1, _esz2) => {}
        };
        true
    }

    pub fn eval_call(&mut self, call: Box<ast::Call>) -> bool {
        match *call {
            ast::Call::Call(id, exp_vec) => {
                // We get the return type
                // of the function to the expression
                // to check semantics are ok
                let fn_tipo: Tipo = self.get_fn_tipo_from_id(&id);
                self.st_vals.push(format!("{}()", id));
                self.st_tips.push(fn_tipo);
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

                let id = self.st_vals.pop().unwrap();
                let tip = self.st_tips.pop().unwrap();
                let id_addr = self.get_id_addr(&id, &tip);
                self.check_tipo_can_eval_as_bool(&tip);
                let pos_pending_if = self.quads.len();
                self.quads.push(Quadruple::Temp());

                self.eval_block(block);

                self.quads.insert(
                    pos_pending_if,
                    Quadruple::GoToF(id_addr, self.quads.len() as i32),
                );
                self.quads.remove(pos_pending_if + 1);
            }
            ast::Cond::IfElse(exp, block_if, block_else) => {
                self.eval_exp(exp);

                let id = self.st_vals.pop().unwrap();
                let tip = self.st_tips.pop().unwrap();
                let id_addr = self.get_id_addr(&id, &tip);
                self.check_tipo_can_eval_as_bool(&tip);
                let pos_pending_f = self.quads.len();
                self.quads.push(Quadruple::Temp());

                self.eval_block(block_if);

                let pos_pending_t = self.quads.len();
                self.quads.push(Quadruple::Temp());
                self.quads.insert(
                    pos_pending_f,
                    Quadruple::GoToF(id_addr, self.quads.len() as i32),
                );
                self.quads.remove(pos_pending_f + 1);

                self.eval_block(block_else);

                self.quads
                    .insert(pos_pending_t, Quadruple::GoTo(self.quads.len() as i32));
                self.quads.remove(pos_pending_t + 1);
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
                // Flag for first, as we start using
                // "Or" from second gexp
                let mut fst: bool = true;
                for texp in texp_vec {
                    // Push operator if not first
                    if fst == false {
                        self.st_ops.push("Or".to_string());
                    }
                    // Eval texp
                    self.eval_texp(texp);
                    // Solve operator if not firsts
                    if fst == false {
                        self.solve_exp_quadruple("Or".to_string());
                    }
                    fst = false;
                }
            }
        };
        true
    }

    pub fn eval_texp(&mut self, texp: Box<ast::Texp>) -> bool {
        match *texp {
            ast::Texp::Gexp(gexp_vec) => {
                // Flag for first, as we start using
                // "And" from second gexp
                let mut fst: bool = true;
                for gexp in gexp_vec {
                    // Push operator if not first
                    if fst == false {
                        self.st_ops.push("And".to_string());
                    }
                    // Eval gexp
                    self.eval_gexp(gexp);
                    // Solve operator if not firsts
                    if fst == false {
                        self.solve_exp_quadruple("And".to_string());
                    }
                    fst = false;
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
            ast::Gexp::Comp(mexp1, comp, mexp2) => {
                self.eval_mexp(mexp1);
                self.st_ops.push(self.ast_comp_to_string(&comp));
                self.eval_mexp(mexp2);
                self.solve_exp_quadruple(self.ast_comp_to_string(&comp));
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
                self.st_ops.push("+".to_string());
                self.eval_term(term);
                self.solve_exp_quadruple("+".to_string());
            }
            ast::Mexp::Sub(mexp, term) => {
                self.eval_mexp(mexp);
                self.st_ops.push("-".to_string());
                self.eval_term(term);
                self.solve_exp_quadruple("-".to_string());
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
                self.st_ops.push("*".to_string());
                self.eval_fact(fact);
                self.solve_exp_quadruple("*".to_string());
            }
            ast::Term::Div(term, fact) => {
                self.eval_term(term);
                self.st_ops.push("/".to_string());
                self.eval_fact(fact);
                self.solve_exp_quadruple("/".to_string());
            }
        };
        true
    }

    pub fn eval_fact(&mut self, fact: Box<ast::Fact>) -> bool {
        match *fact {
            ast::Fact::Parentheses(exp) => {
                // We dont need to push anythin to the
                // stacks to keep us from
                // doing unvalid stuff
                // as the precedence in
                // elements solves it alone
                self.eval_exp(exp);
            }
            ast::Fact::Int(num) => {
                self.st_vals.push(num.to_string());
                self.st_tips.push(Tipo::Int);
            }
            ast::Fact::Float(num) => {
                self.st_vals.push(num.to_string());
                self.st_tips.push(Tipo::Float);
            }
            ast::Fact::Bool(num) => {
                self.st_vals.push(if num == true {
                    "True".to_string()
                } else {
                    "False".to_string()
                });
                self.st_tips.push(Tipo::Bool);
            }
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

    pub fn solve_exp_quadruple(&mut self, op: String) {
        assert!(self.st_ops.last().unwrap() == &op);
        let rt_val: String = self.st_vals.pop().unwrap();
        let lt_val: String = self.st_vals.pop().unwrap();
        let rt_tipo: Tipo = self.st_tips.pop().unwrap();
        let lt_tipo: Tipo = self.st_tips.pop().unwrap();
        self.st_ops.pop();
        let res_tipo = self.get_result_tipo(&lt_tipo, &rt_tipo, &op);
        let res_id = self.get_next_temp();
        let lt_id_addr = self.get_id_addr(&lt_val, &lt_tipo);
        let rt_id_addr = self.get_id_addr(&rt_val, &rt_tipo);
        let res_id_addr = self.get_id_addr(&res_id, &res_tipo);
        self.quads
            .push(Quadruple::Op(op, lt_id_addr, rt_id_addr, res_id_addr));
        self.st_vals.push(res_id);
        self.st_tips.push(res_tipo)
    }

    pub fn get_result_tipo(&self, left: &Tipo, right: &Tipo, op: &str) -> Tipo {
        let res_tipo = self.sem_cube.table.get(&(
            SC::tipo_to_val(&left),
            SC::tipo_to_val(&right),
            SC::op_to_val(op),
        ));
        if res_tipo == None {
            panic!(
                "ERROR: Type Mismatch! Operation \"{}\" Between {:?} and {:?} is not allowed",
                op, left, right
            );
        }
        SC::val_to_tipo(*res_tipo.unwrap())
    }

    pub fn get_next_temp(&mut self) -> String {
        self.next_cnt = self.next_cnt + 1;
        format!("#temp{}", self.next_cnt - 1)
    }

    pub fn get_fn_tipo_from_id(&self, id: &String) -> Tipo {
        if let None = self.dir_func.get(id) {
            panic!("ERROR: Undeclared function call: \"{}\"", id);
        }
        self.dir_func.get(id).unwrap().tipo.clone()
    }

    pub fn get_var_tipo_from_id(&self, id: &String) -> Tipo {
        // First check local scope
        if let Some(loc_scope) = self.loc_scope.as_ref() {
            // Get local var table reference
            let vars = &self.dir_func.get(loc_scope).unwrap().dir_var;
            // Query var
            if let Some(var) = vars.get(id) {
                return var.tipo.clone();
            }
        }
        // If local not found
        // we check global scope
        let vars = &self
            .dir_func
            .get(self.glob_scope.as_ref().unwrap())
            .unwrap()
            .dir_var;
        if let None = vars.get(id) {
            // Not found either locally or globally
            // It is undeclared
            let place: &String = if self.loc_scope == None {
                self.glob_scope.as_ref().unwrap()
            } else {
                self.loc_scope.as_ref().unwrap()
            };
            panic!("ERROR: Undeclared variable \"{}\" at \"{}\"", id, place);
        }
        vars.get(id).unwrap().tipo.clone()
    }

    pub fn check_tipo_can_eval_as_bool(&self, tipo: &Tipo) {
        // Just Int and Bool can eval as Bool
        if tipo != &Tipo::Bool && tipo != &Tipo::Int {
            panic!(
                "ERROR: Cannot eval a conditional expression. Expected Bool or Int. Got {:?}",
                tipo
            );
        }
    }

    pub fn ast_comp_to_string(&self, comp: &ast::Comp) -> String {
        match comp {
            ast::Comp::Greater => ">".to_string(),
            ast::Comp::GreaterEq => ">=".to_string(),
            ast::Comp::Smaller => "<".to_string(),
            ast::Comp::SmallerEq => "<=".to_string(),
            ast::Comp::NotEqual => "!=".to_string(),
            ast::Comp::Equal => "==".to_string(),
        }
    }

    pub fn get_id_addr(&mut self, id: &String, tip: &Tipo) -> IdAddr {
        // Temp?
        //   Identify with #
        if id.contains("#") {
            return (
                id[1..].to_string(),
                self.vir_mem_alloc.get_temp_addr(id, tip),
            );
        }

        // Call?
        //   Identify with ()
        //   TODO: Replace?
        if id.contains("(") {
            return (id.clone(), -1);
        }

        // Constant?
        // Numeric (Int, Float, Bool)
        // String Literals
        if id.contains("\"")
            || id.chars().all(|c| !char::is_alphabetic(c))
            || id.chars().nth(0).unwrap() == 'T'
            || id.chars().nth(0).unwrap() == 'F'
        {
            return (id.clone(), self.vir_mem_alloc.get_cnst_addr(&id, tip));
        }

        // Variable?
        // First check local scope
        if let Some(loc_scope) = self.loc_scope.as_ref() {
            // Get local var table reference
            let vars = &self.dir_func.get(loc_scope).unwrap().dir_var;
            // Query var
            if let Some(var) = vars.get(id) {
                return (id.clone(), var.addr);
            }
        }
        // If local not found
        // we check global scope
        // It must exist, as we already checked
        // on eval_variable
        let vars = &self
            .dir_func
            .get(self.glob_scope.as_ref().unwrap())
            .unwrap()
            .dir_var;
        let var = vars.get(id);
        if let None = var {
            panic!("DEV ERROR: Variable with id {} should exist", id);
        }
        let var = var.unwrap();
        (id.clone(), var.addr)
    }
}
