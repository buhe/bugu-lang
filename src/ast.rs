#[derive(Debug, Clone)]
pub struct Prog {
  pub func: Func,
}

#[derive(Debug, Clone)]
pub struct Func {
  pub name: String,
  pub stmt: Vec<Block>,
}

#[derive(Debug, Clone)]
pub struct Decl {
  pub t: Type,
  pub name: String,
  pub expr: Option<Expr>,
}
#[derive(Debug, Clone)]
pub enum Block {
    Stmt(Stmt),
    Decl(Decl),
}
#[derive(Debug, Clone)]
pub enum Stmt {
  Ret(Expr),
  Expr(Option<Expr>),
  If(Expr, Box<Stmt>, Option<Box<Stmt>>),
  // Decl(Decl),
}

#[derive(Debug, Clone)]
pub enum Type {
  Integer  
}

#[derive(Debug, Clone)]
pub enum Expr {
  Unary(Unary),
  Add(Box<Expr>, Box<Expr>),// left right
  Sub(Box<Expr>, Box<Expr>),
  Mul(Box<Expr>, Box<Expr>),
  Div(Box<Expr>, Box<Expr>),
  Mod(Box<Expr>, Box<Expr>),
  Lt(Box<Expr>, Box<Expr>),
  Gt(Box<Expr>, Box<Expr>),
  Let(Box<Expr>, Box<Expr>),
  Get(Box<Expr>, Box<Expr>),
  And(Box<Expr>, Box<Expr>),
  Or(Box<Expr>, Box<Expr>),
  NotEquals(Box<Expr>, Box<Expr>),
  Equals(Box<Expr>, Box<Expr>),
  Assign(Box<String>, Box<Expr>),
  Cond(Box<Expr>, Box<Expr>, Box<Expr>),
  Null,
}
#[derive(Debug, Clone)]
pub enum Unary {
  Int(i32),
  Neg(Box<Unary>),
  Primary(Box<Expr>),
  Identifier(Box<String>),
}
