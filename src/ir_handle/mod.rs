use crate::{ir::IrProg, symbols::SymTab};

mod dataflow;

pub fn op(p: &IrProg, s: &mut SymTab) -> IrProg {
    dataflow::dataflow(p, s)
}