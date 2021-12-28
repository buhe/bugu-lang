pub mod lexer;
pub mod parser;
pub mod ast;
pub mod ir;
pub mod codegen;
pub mod regeister;
pub mod symbols;

use lexer::tokenize;
use parser::parsing;
use std::io::{Result, Write};

pub fn run(path: String, output: &mut impl Write) -> Result<()> {
  let t = tokenize(path);
  println!("Tokens: {:#?}", t);
   let mut p = parsing(&t);
     println!("Prog: {:#?}", &p.0);
     let mut p_ir = ir::ast2ir(&p.0, &mut p.1);
     println!("IR Prog: {:#?}", &p_ir.0);
     codegen::write_asm(&p_ir.0,&mut p_ir.1,&mut p.1, output)
  // Ok(())
}
