use crate::{ir::*, symbols::{SymTab}};
use std::io::{Result, Write};

pub fn write_asm(p: &IrProg ,table: &mut SymTab, w: &mut impl Write) -> Result<()> {
  for g in &p.global_vars {
    
    match g {
        IrStmt::DeclGlobal(vn, val) => {
          writeln!(w, ".data")?;
          writeln!(w, ".align	2")?;
          writeln!(w, ".global {}", vn)?;
          writeln!(w, "{}:", vn)?;
          writeln!(w, "  .word {}", val)?;
        }
        IrStmt::DeclGlobalArray(vn, indexes) => {
          let mut memory = 4;
          indexes.iter().for_each(|e| memory *= e);
          writeln!(w, ".text")?;
          writeln!(w, "  .comm {},{},4", vn,memory)?
          // writeln!(w, ".bss")?;
          // writeln!(w, ".global {}", vn)?;
          // writeln!(w, "{}:", vn)?;
          // writeln!(w, "  .word {}", memory)?;
        }
        _ => unreachable!()
    }
  }
  for f in &p.funcs {
    let mut alloc_size = 0;
    writeln!(w, ".text")?;
    writeln!(w, ".align	2")?;
    writeln!(w, ".global {}", f.name)?;
    writeln!(w, "{}:", f.name)?;
    writeln!(w, "  addi sp, sp, -56")?;
    writeln!(w, "  sw s0, 0(sp)")?;
    writeln!(w, "  sw s1, 4(sp)")?;
    writeln!(w, "  sw s2, 8(sp)")?;
    writeln!(w, "  sw s3, 12(sp)")?;
    writeln!(w, "  sw s4, 16(sp)")?;
    writeln!(w, "  sw s5, 20(sp)")?;
    writeln!(w, "  sw s6, 24(sp)")?;
    writeln!(w, "  sw s7, 28(sp)")?;
    writeln!(w, "  sw s8, 32(sp)")?;
    writeln!(w, "  sw s9, 36(sp)")?;
    writeln!(w, "  sw s10, 40(sp)")?;
    writeln!(w, "  sw s11, 44(sp)")?;
    for s in &f.stmts {
      match s {
        IrStmt::Neg(t1, t2) => {
          writeln!(w, "  neg {} , {}", t2,t1)?;
        }
        IrStmt::Ldc(x, t) => {
          writeln!(w, "  li {}, {}", t, x)?;//todo
        }
        IrStmt::Ret(t) => {
          writeln!(w, "  mv a0, {}", t)?; // a0 is return value

          writeln!(w, "  lw s0, 0(sp)")?;
          writeln!(w, "  lw s1, 4(sp)")?;
          writeln!(w, "  lw s2, 8(sp)")?;
          writeln!(w, "  lw s3, 12(sp)")?;
          writeln!(w, "  lw s4, 16(sp)")?;
          writeln!(w, "  lw s5, 20(sp)")?;
          writeln!(w, "  lw s6, 24(sp)")?;
          writeln!(w, "  lw s7, 28(sp)")?;
          writeln!(w, "  lw s8, 32(sp)")?;
          writeln!(w, "  lw s9, 36(sp)")?;
          writeln!(w, "  lw s10, 40(sp)")?;
          writeln!(w, "  lw s11, 44(sp)")?;
          writeln!(w, "  addi sp, sp, {}", alloc_size + 56)?;

          writeln!(w, "  ret")?;
        }
        IrStmt::Add(t1, t2, t) => {
          writeln!(w, "  add {} ,{} ,{}", t, t2, t1)?;
        }
        IrStmt::Sub(t1, t2, t) => {
          writeln!(w, "  sub {} ,{} ,{}", t, t2, t1)?;
        }
        IrStmt::Div(t1, t2, t) => {
          writeln!(w, "  div {} ,{} ,{}", t, t2, t1)?; //todo
        }
        IrStmt::Mod(t1, t2, t) => {
          writeln!(w, "  mod {} ,{} ,{}", t, t2, t1)?; // todo
        }
        IrStmt::Mul(t1, t2, t) => {
          writeln!(w, "  mul {} ,{} ,{}", t, t2, t1)?; //todo
        }
        IrStmt::Or(t1, t2, t)=> {
          // or t3,t1,t2 ; snez t3,t3
          writeln!(w, "  or {} ,{} ,{}", t, t2, t1)?;
          writeln!(w, "  snez {} ,{}", t, t)?;
        }
        IrStmt::And(t1, t2, t) => {
          // snez d, s1; sub d, zero, d; and d, d, s2; snez d, d;
          writeln!(w, "  snez {} ,{}", t, t2)?;
          writeln!(w, "  sub {} ,zero ,{}", t, t)?;
          writeln!(w, "  and {} ,{} ,{}", t, t, t1)?;
          writeln!(w, "  snez {} ,{}", t, t)?;
        }
        IrStmt::Equal(t1, t2, t, t3) => {
          // seqz t1,t2	Set EQual to Zero : if t2 == 0 then set t1 to 1 else 0
          writeln!(w, "  sub {} ,{} ,{}", t, t2, t1)?;
          writeln!(w, "  seqz {} ,{}", t3, t)?;
        }
        IrStmt::NotEqual(t1, t2, t, t3) => {
          // snez t1,t2	Set Not Equal to Zero : if t2 != 0 then set t1 to 1 else 0
          writeln!(w, "  sub {} ,{} ,{}", t, t2, t1)?;
          writeln!(w, "  snez {} ,{}", t3, t)?
        }
        IrStmt::Lt(t1, t2, t) => {
          writeln!(w, "  slt {} ,{} ,{}", t, t2, t1)?;
        }
        IrStmt::Let(t1, t2, t, t3, t4, t5) => {
          // eq
          writeln!(w, "  sub {} ,{} ,{}", t, t2, t1)?;
          writeln!(w, "  seqz {} ,{}", t3, t)?;
          // lt
          writeln!(w, "  slt {} ,{} ,{}", t4, t2, t1)?;
          // or
          writeln!(w, "  or {} ,{} ,{}", t5, t3, t4)?;
          writeln!(w, "  snez {} ,{}", t5, t5)?;
        }
        IrStmt::Gt(t1, t2, t) => {// todo slt
          writeln!(w, "  sgt {} ,{} ,{}", t, t2, t1)?;
        }
        IrStmt::Get(t1, t2, t, t3, t4, t5) => {// todo slt
          // eq
          writeln!(w, "  sub {} ,{} ,{}", t, t2, t1)?;
          writeln!(w, "  seqz {} ,{}", t3, t)?;
          // gt
          writeln!(w, "  sgt {} ,{} ,{}", t4, t2, t1)?;
          // or
          writeln!(w, "  or {} ,{} ,{}", t5, t3, t4)?;
          writeln!(w, "  snez {} ,{}", t5, t5)?;
        }
        IrStmt::Assign(scope, id,t2) => {
          let s = table.get(scope, id);
          writeln!(w, "  mv {} ,{}", s.reg.as_ref().unwrap(), t2)?;
        },
        IrStmt::Ref(_scope,_id) => {
          // use
        },
        IrStmt::Beq(t,l) => {
          writeln!(w, "  beqz {} ,{}", t, l)?;
        },
        IrStmt::Jmp(l) => {
          writeln!(w, "  j {}", l)?;
        },
        IrStmt::Label(label) => {
          writeln!(w, "{}:", label)?;
        },
        IrStmt::Param(_, _, _) => {
          // let sym = table.get(scope, var_name);
          // like a0
          // let reg = sym.reg.as_ref().unwrap();
        },// args is a0-a7
        IrStmt::Call(regs, label,r) => {
          // mv t a
          // jmp label
          
          writeln!(w, "  addi sp, sp, -60")?;
          writeln!(w, "  sw ra, 0(sp)")?;
          writeln!(w, "  sw t0, 4(sp)")?;
          writeln!(w, "  sw t1, 8(sp)")?;
          writeln!(w, "  sw t2, 12(sp)")?;
          writeln!(w, "  sw t3, 16(sp)")?;
          writeln!(w, "  sw t4, 20(sp)")?;
          writeln!(w, "  sw t5, 24(sp)")?;
          writeln!(w, "  sw t6, 28(sp)")?;
          writeln!(w, "  sw a0, 32(sp)")?;
          writeln!(w, "  sw a1, 36(sp)")?;
          writeln!(w, "  sw a2, 40(sp)")?;
          writeln!(w, "  sw a3, 44(sp)")?;
          writeln!(w, "  sw a4, 48(sp)")?;
          writeln!(w, "  sw a5, 52(sp)")?;
          writeln!(w, "  sw a6, 56(sp)")?;
          writeln!(w, "  sw a7, 60(sp)")?;
          // args
          for pair in regs {
            writeln!(w, "  mv {} ,{}", pair.1, pair.0)?;
          }
          // call f
          writeln!(w, "  call {}", label)?;
          
          writeln!(w, "  lw ra, 0(sp)")?;
          writeln!(w, "  lw t0, 4(sp)")?;
          writeln!(w, "  lw t1, 8(sp)")?;
          writeln!(w, "  lw t2, 12(sp)")?;
          writeln!(w, "  lw t3, 16(sp)")?;
          writeln!(w, "  lw t4, 20(sp)")?;
          writeln!(w, "  lw t5, 24(sp)")?;
          writeln!(w, "  lw t6, 28(sp)")?;
          
          writeln!(w, "  lw a1, 36(sp)")?;
          writeln!(w, "  lw a2, 40(sp)")?;
          writeln!(w, "  lw a3, 44(sp)")?;
          writeln!(w, "  lw a4, 48(sp)")?;
          writeln!(w, "  lw a5, 52(sp)")?;
          writeln!(w, "  lw a6, 56(sp)")?;
          writeln!(w, "  lw a7, 60(sp)")?;
          writeln!(w, "  addi sp, sp, 60")?;

          // return
          // tx = a0
          writeln!(w, "  mv {} ,a0", r)?;

          writeln!(w, "  lw a0, 32(sp)")?;
        },
        IrStmt::Load(_,_,reg, base, offset) => {
          writeln!(w, "  lw {}, {}({})", reg, offset, base)?;
        }
        IrStmt::LoadSymbol(reg,vn) => {
          writeln!(w, "  la {}, {}", reg, vn)?;
        }
        IrStmt::Alloc(_,_,reg, size) => {
          alloc_size += size;
          writeln!(w, "  addi sp, sp, -{}", size)?;
          writeln!(w, "  addi {}, sp, 0", reg)?;
        }
        _ => unreachable!()
      }
    }
  }
  Ok(())
}
