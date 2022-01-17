pub mod lexer;
pub mod parser;
pub mod ast;
pub mod ir;
pub mod codegen;
pub mod regeister;
pub mod symbols;
pub mod ir_handle;

use lexer::tokenize;
use parser::parsing;
use std::io::{Result, Write};

use crate::ir_handle::op;

pub fn run(path: String, output: &mut impl Write) -> Result<()> {
  let t = tokenize(path);
  println!("Tokens: {:#?}", t);
  let mut p = parsing(&t);
  println!("Prog: {:#?}", &p.0);
  let ir = ir::ast2ir(&p.0, &mut p.1);
  println!("IR before dataflow op Prog: {:#?}", &ir);
  let ir = op(&ir, &mut p.1);
  println!("IR Prog: {:#?}", &ir);
  codegen::write_asm(&ir,&mut p.1, output)
}
