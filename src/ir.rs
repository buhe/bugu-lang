use std::{collections::{VecDeque, HashMap}};

use crate::{ast::*, symbols::SymTab, regeister::{VirtualRegeister, ArgTunnel}};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LabelType {
  Other,
  // Else,
  Cond,
  Update,
}
pub struct Label {
  pub name: String,
  pub lt: LabelType,
}

impl Label {
  fn new(name: String,lt: LabelType) -> Self {
    Self {name, lt}
  }
}

pub struct BranchLabel {
  counter: u32,
  branch_stack: Vec<HashMap<LabelType, VecDeque<Label>>>,
  // queue: HashMap<LabelType, VecDeque<Label>>,
}

impl BranchLabel {
  fn init() -> Self {
    Self {
      counter: 0,
      branch_stack: vec![],
      // queue: HashMap::new(),
    }
  }

  fn create(&mut self, lt: LabelType) -> String {
    self.counter += 1;
    let name  = format!("L{}",self.counter);
    let queue = self.branch_stack.get_mut(0).unwrap();
    let n = name.clone();
    if queue.contains_key(&lt){}else{
      queue.insert(lt.clone(), VecDeque::new());
    }
    let bq = queue.get_mut(&lt).unwrap();
    bq.push_front(Label::new(name, lt));
    n
  }

  pub fn label(&self, lt: LabelType) -> &Label {
    let queue = self.branch_stack.get(0).unwrap();
    let q = queue.get(&lt).unwrap();
    // get last
    q.get(q.len() - 1).unwrap()
  }

  pub fn enter_branch(&mut self) {
    // stack
    self.branch_stack.push(HashMap::new());
  }

  pub fn leave_branch(&mut self) {
    // self.queue.clear();
    self.branch_stack.pop().unwrap();
  }

}

#[derive(Debug, Clone)]
pub struct IrProg {
  pub funcs: Vec<IrFunc>,
  pub global_vars: Vec<IrStmt>,
}

#[derive(Debug, Clone)]
pub struct IrFunc {
  pub name: String,
  pub params: Vec<IrStmt>,
  pub stmts: Vec<IrStmt>,
}

#[derive(Debug, Clone)]
pub enum IrStmt {
  Add(String, String, String),
  Sub(String, String, String),
  Mul(String, String, String),
  Div(String, String, String),
  Mod(String, String, String),
  Neg(String, String),
  Or(String, String, String),
  And(String, String, String),
  Equal(String, String, String, String),
  NotEqual(String, String, String, String),
  Lt(String, String, String),
  Let(String, String, String, String, String, String),
  Gt(String, String, String),
  Get(String, String, String, String, String, String),
  Ldc(i32, String),
  Ret(String),
  // env, id, reg
  Assign(Vec<u32>, String, String),
  // env, id
  Ref(Vec<u32>, String),
  // reg label
  Beq(String, String),
  // jump label
  Jmp(String),
  Label(String),
  // scope var name func name
  Param(Vec<u32>, String, String),
  // params (tmp reg, arg reg) label return reg
  Call(Vec<(String, String)>, String, String),
  // scope,name,reg, base reg, num
  Load(Vec<u32>, String, String, String, u32),
  // reg, global var name
  LoadSymbol(String, String),
  // var name val
  DeclGlobal(String, i32),
  // var name, 
  DeclGlobalArray(String, VecDeque<i32>),
  // scope name reg size
  Alloc(Vec<u32>, String, String, u32),
}

pub fn ast2ir(p: &Prog, s: &mut SymTab) -> IrProg {
  let mut bl = BranchLabel::init();
  let mut r = VirtualRegeister::init();
  let mut tunnel = ArgTunnel::init();
  let mut funcs = vec![];
  let mut global_vars = vec![];
  for f in &p.funcs {
    let func = func(&mut tunnel, f, s, &mut bl, &mut r);
    funcs.push(func);
  }
  for g in &p.global_vars {
    if !g.indexes.is_empty() {
      let var_name = g.name.clone();
      let indexes = g.indexes.clone();
      global_vars.push(IrStmt::DeclGlobalArray(var_name, indexes));
    } else {
      if g.expr.is_some() {
        let var_name = g.name.clone();
        let val: i32;
        let e = g.expr.as_ref().unwrap();
        match e {
            Expr::Unary(Unary::Int(i)) => {
              val = *i;
            }
            _ => panic!("initializer element is not constant")
        }
        global_vars.push(IrStmt::DeclGlobal(var_name, val));
      } else {
        panic!("global var is not inited");
      }
    }
  }
  IrProg {
    funcs, global_vars
  }
}

fn func(tunnel: &mut ArgTunnel, f: &Func, table: &mut SymTab, bl: &mut BranchLabel,r: &mut VirtualRegeister) -> IrFunc {
  let mut stmts = Vec::new();
  let mut params = Vec::new();
  // params.push(IrStmt::Label(f.name.clone()));
  // &f.params -> params
  arg(tunnel, f.name.clone(), &mut params, &f.params, table, bl);

  block(tunnel, &mut stmts, &f.stmt, table, bl, r);
  IrFunc {
    name: f.name.clone(),
    stmts,
    params,
  }
}

fn arg(tunnel: &mut ArgTunnel,func_name: String, params: &mut Vec<IrStmt>,ps: &Vec<Param>, table: &mut SymTab, _bl: &mut BranchLabel) {
  for s in ps {
    let n = &s.name;
    let scope = &s.scope;
    // alloc reg
    let entry = table.entry(scope, n);
    entry.and_modify(|s| {
      if s.alloc_phy_reg == false {
        let t = tunnel.set_arg(&func_name);
        s.reg = Some(t.to_string());
        s.alloc_virtual_reg = true;
        s.alloc_phy_reg = true; 
      } 
    });
    params.push(IrStmt::Param(scope.to_vec(), n.to_string(), func_name.to_string()))
  }
}

fn block(tunnel: &mut ArgTunnel,stmts: &mut Vec<IrStmt>,bts: &Vec<BlockItem>, table: &mut SymTab, bl: &mut BranchLabel,r: &mut VirtualRegeister) {
  for s in bts.iter() {
    match s {
        BlockItem::Stmt(s) => {
          stmt(tunnel, stmts,s,table, bl, r);
        },
        BlockItem::Decl(d) => {
          let name = &d.name;
          let scope = &d.scope;
          if !d.indexes.is_empty() {
            // temp array decl
            let mut size = 4;
            d.indexes.iter().for_each(|i| size *= i);
            let t = r.eat();
            // alloc reg
            let entry = table.entry(scope, name);
              entry.and_modify(|s| {
              if s.alloc_virtual_reg == false {
                s.reg = Some(t.to_string());
                s.alloc_virtual_reg = true; 
              } 
            });
            // alloc c 
            stmts.push(IrStmt::Alloc(scope.clone(),name.clone(), t,size.try_into().unwrap()));
          }
          if let Some(ex) = &d.expr { //when assign
       
            expr(tunnel, stmts, ex, table, bl, r);
            let t2 = r.near();// todo, noy use near api
             // alloc reg
            let entry = table.entry(scope, name);
            entry.and_modify(|s| {
              if s.alloc_virtual_reg == false {
                let t = r.eat();
                s.reg = Some(t.to_string());
                s.alloc_virtual_reg = true; 
              } 
            });
            
            stmts.push(IrStmt::Assign(scope.to_vec(), name.to_string(),t2));
          }
        },
    }
  }
}

fn stmt(tunnel: &mut ArgTunnel,stmts: &mut Vec<IrStmt>,s: &Stmt, table: &mut SymTab,bl: &mut BranchLabel,r: &mut VirtualRegeister) {
  match s {
      Stmt::Ret(e) => {
        expr(tunnel, stmts, e, table, bl, r);
        let t = r.near();
        stmts.push(IrStmt::Ret(t));
      }
      Stmt::Expr(e) => {
        if let Some(ex) = e {
          expr(tunnel, stmts, ex, table, bl, r);
        }
      },
      Stmt::If(e, t, l) => {
        // 1. create label
        // 2. add beq ir
        // 3. when has else, add jmp ir
        bl.enter_branch();
        let other_branch = bl.create(LabelType::Other);
        expr(tunnel, stmts, e, table, bl, r);
        let reg = r.near();
        
        stmt(tunnel, stmts, t, table, bl, r);
        if l.is_some() {
          let else_branch = bl.create(LabelType::Other); 
          stmts.push(IrStmt::Beq(reg, bl.label(LabelType::Other).name.clone()));
          let s1 = l.as_ref().unwrap();
          stmts.push(IrStmt::Jmp(bl.label(LabelType::Other).name.clone()));//when has else, jump to other
          stmts.push(IrStmt::Label(else_branch));
          stmt(tunnel, stmts, s1, table, bl, r)
        } else {
          stmts.push(IrStmt::Beq(reg, bl.label(LabelType::Other).name.clone()));
        }
        stmts.push(IrStmt::Label(other_branch));
        bl.leave_branch();
      },
      Stmt::Block(bts) => {
        block(tunnel, stmts, bts, table, bl, r)
      },
    Stmt::For(init, cond, update, s) => {
      bl.enter_branch();
       let other_branch = bl.create(LabelType::Other); 
       let cond_branch = bl.create(LabelType::Cond); 
      if let Some(ex) = init {
          expr(tunnel, stmts, ex, table, bl, r);
        }
        stmts.push(IrStmt::Label(bl.label(LabelType::Cond).name.clone()));// <--
        if let Some(ex) = cond {
          expr(tunnel, stmts, ex, table, bl, r);
        }
        stmts.push(IrStmt::Beq(r.near(), other_branch));//--------------------------
        stmt(tunnel, stmts, s, table, bl, r);// body
        bl.create(LabelType::Update);
        stmts.push(IrStmt::Label(bl.label(LabelType::Update).name.clone()));
        if let Some(ex) = update {
          expr(tunnel, stmts, ex, table, bl, r);
        }
        stmts.push(IrStmt::Jmp(cond_branch));//----------------------------
        stmts.push(IrStmt::Label(bl.label(LabelType::Other).name.clone()));// <----------
        bl.leave_branch();
    },
    Stmt::While(cond,s) => {
      bl.enter_branch();
      let other_branch = bl.create(LabelType::Other); 
      let cond_branch = bl.create(LabelType::Cond); 
      bl.create(LabelType::Update); 

      stmts.push(IrStmt::Label(bl.label(LabelType::Cond).name.clone()));
      stmts.push(IrStmt::Label(bl.label(LabelType::Update).name.clone()));
      expr(tunnel, stmts, cond, table, bl, r);
      stmts.push(IrStmt::Beq(r.near(), other_branch));
      stmt(tunnel, stmts, s, table, bl, r);
      stmts.push(IrStmt::Jmp(cond_branch));
      stmts.push(IrStmt::Label(bl.label(LabelType::Other).name.clone()));
      bl.leave_branch();
    },
    Stmt::Continue => {
      stmts.push(IrStmt::Jmp(bl.label(LabelType::Update).name.clone()));
    },
    Stmt::Break => {
      stmts.push(IrStmt::Jmp(bl.label(LabelType::Other).name.clone()));
    },
  }
}

fn expr(tunnel: &mut ArgTunnel,stmts: &mut Vec<IrStmt>, e: &Expr, table: &mut SymTab, bl: &mut BranchLabel,r: &mut VirtualRegeister) {
  bin_op(tunnel, stmts, e, table, bl, r)
}

fn bin_op(tunnel: &mut ArgTunnel,stmts: &mut Vec<IrStmt>,m: &Expr, table: &mut SymTab, bl: &mut BranchLabel,r: &mut VirtualRegeister) {
  match m {
    Expr::Mul(u, m1) => {
      bin_op(tunnel, stmts, u, table, bl, r);
      bin_op(tunnel, stmts, m1, table, bl, r);
      let t1 = r.near();
      let t2 = r.near();
      let t = r.eat();
      stmts.push(IrStmt::Mul(t1, t2, t));
    },
    Expr::Div(u, m1) => {
      bin_op(tunnel, stmts, u, table, bl, r);
      bin_op(tunnel, stmts, m1, table, bl, r);
      let t1 = r.near();
      let t2 = r.near();
      let t = r.eat();
      stmts.push(IrStmt::Div(t1, t2, t));
    },
    Expr::Mod(u, m1) => {
      bin_op(tunnel, stmts, u, table, bl, r);
      bin_op(tunnel, stmts, m1, table, bl, r);
      let t1 = r.near();
      let t2 = r.near();
      let t = r.eat();
      stmts.push(IrStmt::Mod(t1, t2, t));
    },
    Expr::Add(m,a1) => {
      bin_op(tunnel, stmts, m, table, bl, r);
      bin_op(tunnel, stmts, a1, table, bl, r);
      let t1 = r.near();
      let t2 = r.near();
      let t = r.eat();
      stmts.push(IrStmt::Add(t1, t2, t));
    },
    Expr::Sub(m,a1)=> {
      bin_op(tunnel, stmts, m, table, bl, r);
      bin_op(tunnel, stmts, a1, table, bl, r);
      let t1 = r.near();
      let t2 = r.near();
      let t = r.eat();
      stmts.push(IrStmt::Sub(t1, t2, t));
    },
    Expr::Unary(u) => unary(tunnel, stmts, u, table, bl, r),
    Expr::Lt(e, e1) => {
      bin_op(tunnel, stmts, e, table, bl, r);
      bin_op(tunnel, stmts, e1, table, bl, r);
      let t1 = r.near();
      let t2 = r.near();
      let t = r.eat();
      stmts.push(IrStmt::Lt(t1, t2, t));
    }
    Expr::Gt(e, e1) => {
      bin_op(tunnel, stmts, e, table, bl, r);
      bin_op(tunnel, stmts, e1, table, bl, r);
      let t1 = r.near();
      let t2 = r.near();
      let t = r.eat();
      stmts.push(IrStmt::Gt(t1, t2, t));
    }
    Expr::Let(e, e1) => {
      bin_op(tunnel, stmts, e, table, bl, r);
      bin_op(tunnel, stmts, e1, table, bl, r);
      let t1 = r.near();
      let t2 = r.near();
      let t = r.eat();

      let t3 = r.eat();

      let t4 = r.eat();

      let t5 = r.eat();
      stmts.push(IrStmt::Let(t1, t2, t, t3, t4, t5));
    }
    Expr::Get(e, e1) => {
      bin_op(tunnel, stmts, e, table, bl, r);
      bin_op(tunnel, stmts, e1, table, bl, r);
      let t1 = r.near();
      let t2 = r.near();
      let t = r.eat();

      let t3 = r.eat();

      let t4 = r.eat();

      let t5 = r.eat();
      stmts.push(IrStmt::Get(t1, t2, t, t3, t4, t5));
    }
    Expr::And(e, e1) => {
      bin_op(tunnel, stmts, e, table, bl, r);
      bin_op(tunnel, stmts, e1, table, bl, r);
      let t1 = r.near(); // s2
      let t2 = r.near(); // s1
      let t = r.eat(); // d
      stmts.push(IrStmt::And(t1, t2, t));
    }
    Expr::Or(e, e1) => {
      bin_op(tunnel, stmts, e, table, bl, r);
      bin_op(tunnel, stmts, e1, table, bl, r);
          let t1 = r.near();
          let t2 = r.near();
          let t = r.eat();
      stmts.push(IrStmt::Or(t1, t2, t));
    }
    Expr::NotEquals(e, e1) => {
      bin_op(tunnel, stmts, e, table, bl, r);
      bin_op(tunnel, stmts, e1, table, bl, r);
      let t1 = r.near();
      let t2 = r.near();
      let t = r.eat();
      let t3 = r.eat();
      stmts.push(IrStmt::NotEqual(t1, t2, t, t3));
    }
    Expr::Equals(e, e1) => {
      bin_op(tunnel, stmts, e, table, bl, r);
      bin_op(tunnel, stmts, e1, table, bl, r);
      let t1 = r.near();
      let t2 = r.near();
      let t = r.eat();

      let t3 = r.eat();
      stmts.push(IrStmt::Equal(t1, t2, t, t3));
    }
    Expr::Assign(env,id, e) => {
      let name = &**id;
      let n = &**env;
      bin_op(tunnel, stmts, e, table, bl, r);
      let t2 = r.near();// todo, noy use near api
        let entry = table.entry(n, name);
      entry.and_modify(|s| {
        if s.alloc_virtual_reg == false {
          let t = r.eat();
          s.reg = Some(t.to_string());
          s.alloc_virtual_reg = true; 
        } 
      });
      stmts.push(IrStmt::Assign(n.to_vec(), name.to_string(),t2));
    },
    Expr::Null => {},
    Expr::Cond(condition, then, other) => {
      // like if-else
      bl.enter_branch();
      let else_branch = bl.create(LabelType::Other);
      let other_branch = bl.create(LabelType::Other);
      expr(tunnel, stmts, condition, table, bl, r);
      let reg = r.near();
      stmts.push(IrStmt::Beq(reg, bl.label(LabelType::Other).name.clone()));
      expr(tunnel, stmts, then, table, bl, r);
      stmts.push(IrStmt::Jmp(bl.label(LabelType::Other).name.clone()));
      stmts.push(IrStmt::Label(else_branch));
      expr(tunnel, stmts, other, table, bl, r);
      stmts.push(IrStmt::Label(other_branch));
      bl.leave_branch();
    },
  }
}

fn unary(tunnel: &mut ArgTunnel,stmts: &mut Vec<IrStmt>, u: &Unary, table: &mut SymTab, bl: &mut BranchLabel,r: &mut VirtualRegeister) {
  match u {
        Unary::Int(y) => {
          let t = r.eat();
          stmts.push(IrStmt::Ldc(*y, t))
        },
        Unary::Neg(y) => { 
          unary(tunnel, stmts, &*y, table, bl, r);
          let t1 = r.near();
          let t2 = r.eat();
          stmts.push(IrStmt::Neg(t1, t2));
        },
        Unary::Primary(y) => {
          expr(tunnel, stmts, &*y, table, bl, r)
        }
        Unary::Identifier(env, id) => {
          // check decl, table exist
          // match x + y
          let name = &**id;
          // use var
          let var = table.extis(env, name);
          assert!(var.0);
          if var.1 == vec![1] {
            let reg = r.eat();
            // global var
            stmts.push(IrStmt::LoadSymbol(reg.clone(), name.clone()));
                  // alloc reg
            let entry = table.entry(&vec![1], name);
            entry.and_modify(|s| {
              if s.alloc_virtual_reg == false {
                s.reg = Some(reg.clone());
                s.alloc_virtual_reg = true;
              } 
            });
            stmts.push(IrStmt::Load(vec![1],name.clone(),r.eat(), reg.clone(), 0));
            r.put_near(reg.clone());
          } else {
            let reg = table.get(&var.1, name).reg.as_ref().unwrap();
            r.put_near(reg.clone());

            stmts.push(IrStmt::Ref(env.to_vec(), name.to_string()));
          }
          // println!("t:{:?} env {:?} var.1 {:?} n {}", table, env, var.1, name);
          
        },
        Unary::Call(call) => {
          // match func(1, 2)
          let mut params = vec![];
          let label = &call.name;
          for e in &call.params {
            expr(tunnel, stmts, e, table, bl, r);
            params.push((r.near(), tunnel.get_arg(label)));
          }
          assert!(tunnel.is_match(label));
          stmts.push(IrStmt::Call(params, label.to_string(), r.eat()));
        }
        Unary::Index(env, i) => {
          let name = &i.name;
          // use var
          let var = table.extis(env, name);
          assert!(var.0);
          // off = (range + index) * 4
          let mut offset = 0;
          let mut range = table.get(env, name).array_range.clone();
          for index in &i.index {
            range.pop_front().unwrap(); // high -> low
            let mut result = 1;
            range.iter().for_each(|r| result *= r);
            offset += index * result;
          }
          offset *= 4;
          if var.1 == vec![1] {
            let base_reg = r.eat();
            stmts.push(IrStmt::LoadSymbol(base_reg.clone(), name.clone()));
             // alloc reg
            let entry = table.entry(&vec![1], name);
            entry.and_modify(|s| {
              if s.alloc_virtual_reg == false {
                s.reg = Some(base_reg.clone());
                s.alloc_virtual_reg = true;
              } 
            });
            stmts.push(IrStmt::Load(vec![1],name.clone(),r.eat(),base_reg.clone(), offset.try_into().unwrap()));
            // global var
            
          } else {
            // temp var
            let base_reg = table.get(&var.1, name).reg.as_ref().unwrap();
            stmts.push(IrStmt::Load(var.1,name.clone(),r.eat(),base_reg.clone(), offset.try_into().unwrap()));
          }
        }
    }
}
