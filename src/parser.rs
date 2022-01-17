use std::collections::VecDeque;

use crate::ast::*;
use crate::lexer::{Token, TokenType};
use crate::symbols::{SymTab, Symbol};

pub fn parsing(tokens: &Vec<Token>) -> (Prog, SymTab) {
  let mut parser = Parser::new(tokens.to_vec());
  parser.prog();
  if let Some(prog) = parser.prog {
    return (prog, parser.symbols);
  } else {
    panic!("Error in parsing");
  }
}

pub struct Parser {
  tokens: Vec<Token>,
  pos: usize,
  prog: Option<Prog>,
  pub symbols: SymTab,
}

impl Parser {
  pub fn new(tokens: Vec<Token>) -> Self {
    Parser {
      tokens,
      pos: 0,
      prog: None,
      symbols: SymTab::init(),
    }
  }

  // pub fn get_table(&self) -> SymTab {
  //   self.symbols
  // }

  pub fn bad_token(&self, msg: &str) -> ! {
    panic!("{}", msg);
  }

  fn expect(&mut self, ty: TokenType) {
    let t = &self.tokens[self.pos];
    if t.ty != ty {
      self.bad_token(&format!("{:?} expected , actual {:?}", ty, t.ty));
    }
    self.pos += 1;
  }

  fn expr(&mut self) -> Expr {
    self.assignment()
  }

  fn assignment(&mut self) -> Expr {
    let t = &self.tokens[self.pos];
    if let TokenType::Ident(id) = &t.ty { // refactor to &&
      self.pos += 1; // eat id
      let t2 = &self.tokens[self.pos];
      if let TokenType::Equal = &t2.ty {
          self.pos += 1; // eat =
          let t3 = &self.tokens[self.pos];
          if let TokenType::Equal = &t3.ty {
            self.pos -= 2; // rollback = and id
            self.condition()
          } else {
            let scope = self.symbols.extis(&self.symbols.current_scope, id).1;
            Expr::Assign(Box::new(scope),Box::new(id.to_string()), Box::new(self.expr()))
          }
      } else {
          self.pos -= 1; // rollback id
          self.condition()
      }
    } else {
        self.condition()
    }
  }

  fn condition(&mut self) -> Expr {
    let or = self.logical_or();
    let t = &self.tokens[self.pos];
    match t.ty {
      TokenType::Ques => {
        self.expect(TokenType::Ques);
        let e = self.expr();
        self.expect(TokenType::Colon);
        let c = self.condition();
        Expr::Cond(Box::new(or), Box::new(e), Box::new(c))
      },
      _ => or
    }

  }

  fn logical_or(&mut self) -> Expr {
    let and = self.logical_and();
    self.rest3(and) 
  }

  fn rest3(&mut self, e: Expr) -> Expr {
    let t = &self.tokens[self.pos];
    match t.ty {
      TokenType::Or => {
        self.pos += 1;
        let next_t = &self.tokens[self.pos];
        match next_t.ty {
            TokenType::Or => {
              self.pos += 1;
              let and = self.logical_and();
              let e1 = Expr::Or(Box::new(e), Box::new(and));
              self.rest3(e1)
            }
            _ => e
        }
      }
      _ => e
    } 
  }

  fn logical_and(&mut self) -> Expr {
    let eq = self.equality();
    self.rest4(eq)
  }

  fn rest4(&mut self, e: Expr) -> Expr {
    let t = &self.tokens[self.pos];
    match t.ty {
      TokenType::And => {
        self.pos += 1;
        let next_t = &self.tokens[self.pos];
        match next_t.ty {
            TokenType::And => {
              self.pos += 1;
              let eq = self.equality();
              let e1 = Expr::And(Box::new(e), Box::new(eq));
              self.rest4(e1)
            }
            _ => e
        }
      }
      _ => e
    }
  }

  fn equality(&mut self) -> Expr {
    let r = self.relational();
    self.rest5(r)
  }

  fn rest5(&mut self, e: Expr) -> Expr {
    let t = &self.tokens[self.pos];
    match t.ty {
      TokenType::Equal => {
        self.pos += 1;
        let next_t = &self.tokens[self.pos];
        match next_t.ty {
          TokenType::Equal => {
            self.pos += 1;
            let r = self.relational();
            let e1 = Expr::Equals(Box::new(e), Box::new(r));
            self.rest5(e1)
          }
          _ => e
        }
      }
      TokenType::Not => {
        self.pos += 1;
        let next_t = &self.tokens[self.pos];
        match next_t.ty {
          TokenType::Equal => {
            self.pos += 1;
            let r = self.relational();
            let e1 = Expr::NotEquals(Box::new(e), Box::new(r));
            self.rest5(e1)
          }
          _ => e
        }
       
      }
      _ => e
    }
  }
  fn relational(&mut self) -> Expr {
    let a = self.additive();
    self.rest6(a)
  }

  fn rest6(&mut self, e: Expr) -> Expr {
    let t = &self.tokens[self.pos];
    match t.ty {
      TokenType::Lt => {
        self.pos += 1;
        let next_t = &self.tokens[self.pos];
        match next_t.ty {
          TokenType::Equal => {
            self.pos += 1;
            let a = self.additive();
            let e1 = Expr::Let(Box::new(e), Box::new(a));
            self.rest6(e1)
          }
          _ => {
            let a = self.additive();
            let e1 = Expr::Lt(Box::new(e), Box::new(a));
            self.rest6(e1)
          }
        }
      }
      TokenType::Gt => {
        self.pos += 1;
        let next_t = &self.tokens[self.pos];
        match next_t.ty {
          TokenType::Equal => {
            self.pos += 1;
            let a = self.additive();
            let e1 = Expr::Get(Box::new(e), Box::new(a));
            self.rest6(e1)
          }
          _ => {
            let a = self.additive();
            let e1 = Expr::Gt(Box::new(e), Box::new(a));
            self.rest6(e1)
          }
        }
      }
      _ => e
    }
  }

  fn additive(&mut self) -> Expr {
    let m = self.multiplicative();//1
    self.rest(m)//-2-3
  }

  fn rest(&mut self, a: Expr) -> Expr{
    let t = &self.tokens[self.pos];
    match t.ty {
      TokenType::Add => {
        self.pos += 1;
        let m1 = self.multiplicative(); // 2
                                                     // 3
        let a1 = Expr::Add(Box::new(a), Box::new(m1));
        self.rest(a1)
      }
      TokenType::Neg => {
        self.pos += 1;
        let m1 = self.multiplicative(); // 2
                                                     // 3
        let a1 = Expr::Sub(Box::new(a), Box::new(m1));
        self.rest(a1)
      }
      _ => {
        a
      },
    }
  }

  fn multiplicative(&mut self) -> Expr {
    let u = self.unary();
    self.rest2(Expr::Unary(u))
  }

  fn rest2(&mut self, m: Expr) -> Expr {
    let t = &self.tokens[self.pos];
    match t.ty {
       TokenType::Div => {
        self.pos += 1;
        let u1 = self.unary();
        let m1 = Expr::Div(Box::new(m), Box::new(Expr::Unary(u1)));
        self.rest2(m1)
      }
      TokenType::Mod => {
        self.pos += 1;
        let u1 = self.unary();
        let m1 = Expr::Mod(Box::new(m), Box::new(Expr::Unary(u1)));
        self.rest2(m1)
      }
      TokenType::Mul => {
        self.pos += 1;
        let u1 = self.unary();
        let m1 = Expr::Mul(Box::new(m), Box::new(Expr::Unary(u1)));
        self.rest2(m1)
      }
      _ => {
        m
      },
    }
  }

  fn unary(&mut self) -> Unary {
    let t = &self.tokens[self.pos];
    self.pos += 1;
    match &t.ty {
      TokenType::Neg => Unary::Neg(Box::new(self.unary())),
      TokenType::Num(val) => Unary::Int(*val),
      TokenType::LeftParen => {
        let r = Unary::Primary(Box::new(self.expr()));
        self.pos += 1; //skip right paren
        r
      },
      TokenType::Ident(id) => {
        let t = &self.tokens[self.pos];
        let name = id.clone();
        match &t.ty {
            TokenType::LeftParen => {
              self.expect(TokenType::LeftParen);
              let params = self.expression_list();
              self.expect(TokenType::RightParen);
              let func = self.symbols.get_fn(&name);
              Unary::Call(Call{
                name: func.name.clone(),
                params,
              })
            }
            TokenType::LeftBrack => {
              // todo
              // support
              let scope = self.symbols.extis(&self.symbols.current_scope, &id).1;
              let mut e_list = VecDeque::new();
              loop {
                  let t = &self.tokens[self.pos];
                  match &t.ty {
                      TokenType::LeftBrack => {
                        self.expect(TokenType::LeftBrack);
                        let e = self.expr();
                        match e {
                            Expr::Unary(Unary::Int(num)) => {
                              e_list.push_back(num);
                            }
                            _ => self.bad_token("index only support number.")
                        }
                        
                        self.expect(TokenType::RightBrack);
                      }
                      _ => break
                  }
              }
              Unary::Index(Box::new(scope), IndexExpr{name: name,index: e_list})
            }
            _ => {
              let scope = self.symbols.extis(&self.symbols.current_scope, &id).1;
              Unary::Identifier(Box::new(scope),Box::new(name))
            }
        }
      }
      _ => self.bad_token(&format!("expected , actual {:?}", t.ty)),
    }
  }



  fn _type(&mut self) -> Type{
      self.expect(TokenType::Int);
      Type::Integer
  }
  fn identifier(&mut self) -> String {
    let token = &self.tokens[self.pos];
    let name;
    if let TokenType::Ident(id) = &token.ty {
      name = id
    } else {
       self.bad_token("ident expected");
    }
    self.pos += 1;
    name.to_string()
  }

  fn decl_expr(&mut self) -> Option<Expr> {
    let token = &self.tokens[self.pos];
    self.pos += 1;
    match token.ty {
        TokenType::Equal => {
          let e = self.expr(); 
          self.expect(TokenType::Semicolon);
          Some(e)
        },
        TokenType::Semicolon => None, // option expr
        _ => self.bad_token("; or expr expected from decl"),
    }
    
  }
  fn decl(&mut self) -> Decl { // decl is special assign then add assign ir.
    let t = self._type();
    let name = self.identifier();
    let indexes = self.index();
    let expr = self.decl_expr();
    let scope = self.symbols.current_scope.clone();
    self.symbols.put(name.clone(), Symbol::new(name.clone(), indexes.clone()));  
    Decl { t, name, indexes, expr, scope }
  }

  fn index(&mut self) -> VecDeque<i32> {
    let mut indexes = VecDeque::new();
    loop {
        let t = &self.tokens[self.pos];
        match t.ty {
          TokenType::LeftBrack => {
            self.expect(TokenType::LeftBrack);
            let t = &self.tokens[self.pos];
            if let TokenType::Num(n) = t.ty {
              indexes.push_back(n);//first high
              self.pos += 1; // eat index
            } else {
                self.bad_token("expected int")
            }
            self.expect(TokenType::RightBrack);
          }
          _ => break
        }
    }
    indexes
  }
  
  fn stmt(&mut self) -> Stmt {
    let t = &self.tokens[self.pos];
    match t.ty {
      TokenType::Return => {
        self.pos += 1; // eat return
        let e = Stmt::Ret(self.expr()); // return must has expr
        self.expect(TokenType::Semicolon);
        e
      }
      TokenType::Semicolon => {
        self.pos += 1; // eat ;
        Stmt::Expr(Some(Expr::Null))
      } // branch 0 stmt
      TokenType::If => {
        self.expect(TokenType::If);
        self.expect(TokenType::LeftParen);
        let condition = self.expr();
        self.expect(TokenType::RightParen);
        let then_stmt = self.stmt();
        let t1 = &self.tokens[self.pos];
        let mut else_stmt = None;
        if let TokenType::Else = t1.ty {
          self.pos += 1; // eat else
          else_stmt = Some(Box::new(self.stmt()));
        } 
        Stmt::If(condition, Box::new(then_stmt), else_stmt)
      }
      TokenType::For => {
        self.expect(TokenType::For);
        self.expect(TokenType::LeftParen);
        let mut init = None;
        let t = &self.tokens[self.pos];
        match t.ty {
            TokenType::Semicolon => {

            }
            _ => {
              init = Some(self.expr());
            }
        }
        self.expect(TokenType::Semicolon);
        let mut cond = None;
        let t = &self.tokens[self.pos];
        match t.ty {
            TokenType::Semicolon => {

            }
            _ => {
              cond = Some(self.expr());
            }
        }
        self.expect(TokenType::Semicolon);
        let mut update = None;
        let t = &self.tokens[self.pos];
        match t.ty {
            TokenType::Semicolon => {

            }
            _ => {
              update = Some(self.expr());
            }
        }
        self.expect(TokenType::RightParen);
        let stmt = self.stmt();
        Stmt::For(init, cond, update, Box::new(stmt))
      }
      TokenType::While => {
        self.expect(TokenType::While);
        self.expect(TokenType::LeftParen);
        let condition = self.expr();
        self.expect(TokenType::RightParen);
        let stmt = self.stmt();
        Stmt::While(condition, Box::new(stmt))
      }
      TokenType::Continue => {
        self.expect(TokenType::Continue);
        Stmt::Continue
      }
      TokenType::Break => {
        self.expect(TokenType::Break);
        Stmt::Break
      }
      TokenType::LeftBrace => {
        Stmt::Block(self.compound_statement())
      }
      _ => {
        let e = self.expr(); 
        self.expect(TokenType::Semicolon);
        Stmt::Expr(Some(e))
      }
    }
  }

  fn block_item(&mut self) -> Option<BlockItem> {
    let t = &self.tokens[self.pos];
    match t.ty {
      TokenType::Int => Some(BlockItem::Decl(self.decl())),
      TokenType::RightBrace => None, // when } finish.
      _ => Some(BlockItem::Stmt(self.stmt()))
    }
  }

  fn parameter_list(&mut self) -> Vec<Param> {
    let mut params = vec![];
    let t = &self.tokens[self.pos];
    match t.ty {
      TokenType::RightParen => {}
      _ => {
        let _t = self._type();
        let id = self.identifier();
        let indexes = self.index();
        let param = Param::new(self.symbols.current_scope.clone(),id);
        self.symbols.put(param.name.clone(), Symbol::new(param.name.clone(), indexes));
        params.push(param);
        
        loop {
            let t = &self.tokens[self.pos];
            match t.ty {
                TokenType::Comma => {
                  self.pos += 1; //eat ,
                  let _t = self._type();
                  let id = self.identifier();
                  let indexes = self.index();
                  let param = Param::new(self.symbols.current_scope.clone(),id);
                  self.symbols.put(param.name.clone(), Symbol::new(param.name.clone(), indexes));
                  params.push(param);
                }
                _ => break,
            }
        }
      }
    }
    params
  }

  fn expression_list(&mut self) -> Vec<Expr> {
    let mut exprs = vec![];
    let t = &self.tokens[self.pos];
    match t.ty {
      TokenType::RightParen => {}
      _ => {
        let expr = self.expr();
        exprs.push(expr);
        loop {
            let t = &self.tokens[self.pos];
            match t.ty {
                TokenType::Comma => {
                  self.pos += 1; //eat ,
                  exprs.push(self.expr());
                }
                _ => break,
            }
        }
      }
    }
    exprs
  }

  fn compound_statement(&mut self) -> Vec<BlockItem> {
    self.expect(TokenType::LeftBrace);
    
    let mut block_items = vec![];
    loop { // branch mutli stmt
        let stmt = self.block_item();
        match stmt {
            Some(s) => block_items.push(s),
            None => break
        }
    }
    self.expect(TokenType::RightBrace);
    
    block_items
  }

  fn func(&mut self) -> Func {
        self._type();
        // self.expect(TokenType::Int);
        let ident = self.identifier();
        self.symbols.put(ident.clone(), Symbol::new_fn(ident.clone()));
        self.symbols.enter_scope();
        // self.expect(TokenType::Ident("main".to_string()));
        self.expect(TokenType::LeftParen);
        let params = self.parameter_list();
        self.expect(TokenType::RightParen);
        let body = self.compound_statement();
        self.symbols.leave_scope();
        Func {
          name: ident.clone(),
          stmt: body,
          params,
        }
    }
  


  fn prog(&mut self) {
    let mut funcs: Vec<Func> = vec![];
    let mut global_vars: Vec<Decl> = vec![];
    loop {
          let t = &self.tokens[self.pos];
          match t.ty {
            TokenType::Eof => break,
            TokenType::Int => { //type
              self._type(); // eat type
              let t = &self.tokens[self.pos];
              match &t.ty {
                  // expect id token
                  TokenType::Ident(_id) => {
                    self.pos += 1; // eat id
                  }
                  _ => self.bad_token("ident expected"),
              }
              let t = &self.tokens[self.pos];
              match t.ty {
                TokenType::LeftParen => {
                  self.pos -= 2;
                  funcs.push(self.func());
                }
                _ => {
                  self.pos -= 2;
                  global_vars.push(self.decl());
                }
              }
            }
            _ => self.bad_token("no func or global var")
          }
    }
    self.prog = Some(Prog { funcs, global_vars });
  }
}
