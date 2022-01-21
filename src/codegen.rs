use crate::{ir::*, symbols::{SymTab}};
use std::io::{Result, Write};

pub fn write_asm(p: &IrProg ,table: &mut SymTab, w: &mut impl Write) -> Result<()> {
  writeln!(w, ".attribute unaligned_access, 0")?;
  writeln!(w, ".attribute stack_align, 16")?;
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
    writeln!(w, ".align	1")?;
    writeln!(w, ".global {}", f.name)?;
    writeln!(w, "{}:", f.name)?;
    writeln!(w, "  addi sp, sp, -88")?;
    writeln!(w, "  sd s0, 0(sp)")?;
    writeln!(w, "  sd s1, 8(sp)")?;
    writeln!(w, "  sd s2, 16(sp)")?;
    writeln!(w, "  sd s3, 24(sp)")?;
    writeln!(w, "  sd s4, 32(sp)")?;
    writeln!(w, "  sd s5, 40(sp)")?;
    writeln!(w, "  sd s6, 48(sp)")?;
    writeln!(w, "  sd s7, 56(sp)")?;
    writeln!(w, "  sd s8, 64(sp)")?;
    writeln!(w, "  sd s9, 72(sp)")?;
    writeln!(w, "  sd s10, 80(sp)")?;
    writeln!(w, "  sd s11, 88(sp)")?;
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

          writeln!(w, "  ld s0, 0(sp)")?;
          writeln!(w, "  ld s1, 8(sp)")?;
          writeln!(w, "  ld s2, 16(sp)")?;
          writeln!(w, "  ld s3, 24(sp)")?;
          writeln!(w, "  ld s4, 32(sp)")?;
          writeln!(w, "  ld s5, 40(sp)")?;
          writeln!(w, "  ld s6, 48(sp)")?;
          writeln!(w, "  ld s7, 56(sp)")?;
          writeln!(w, "  ld s8, 64(sp)")?;
          writeln!(w, "  ld s9, 72(sp)")?;
          writeln!(w, "  ld s10, 80(sp)")?;
          writeln!(w, "  ld s11, 88(sp)")?;
          writeln!(w, "  addi sp, sp, {}", alloc_size + 88)?;

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
          
          writeln!(w, "  addi sp, sp, -120")?;
          writeln!(w, "  sd ra, 0(sp)")?;
          writeln!(w, "  sd t0, 8(sp)")?;
          writeln!(w, "  sd t1, 16(sp)")?;
          writeln!(w, "  sd t2, 24(sp)")?;
          writeln!(w, "  sd t3, 32(sp)")?;
          writeln!(w, "  sd t4, 40(sp)")?;
          writeln!(w, "  sd t5, 48(sp)")?;
          writeln!(w, "  sd t6, 56(sp)")?;
          writeln!(w, "  sd a0, 64(sp)")?;
          writeln!(w, "  sd a1, 72(sp)")?;
          writeln!(w, "  sd a2, 80(sp)")?;
          writeln!(w, "  sd a3, 88(sp)")?;
          writeln!(w, "  sd a4, 96(sp)")?;
          writeln!(w, "  sd a5, 104(sp)")?;
          writeln!(w, "  sd a6, 112(sp)")?;
          writeln!(w, "  sd a7, 120(sp)")?;
          // args
          for pair in regs {
            writeln!(w, "  mv {} ,{}", pair.1, pair.0)?;
          }
          // call f
          writeln!(w, "  call {}", label)?;
          
          writeln!(w, "  ld ra, 0(sp)")?;
          writeln!(w, "  ld t0, 8(sp)")?;
          writeln!(w, "  ld t1, 16(sp)")?;
          writeln!(w, "  ld t2, 24(sp)")?;
          writeln!(w, "  ld t3, 32(sp)")?;
          writeln!(w, "  ld t4, 40(sp)")?;
          writeln!(w, "  ld t5, 48(sp)")?;
          writeln!(w, "  ld t6, 56(sp)")?;
          
          writeln!(w, "  ld a1, 72(sp)")?;
          writeln!(w, "  ld a2, 80(sp)")?;
          writeln!(w, "  ld a3, 88(sp)")?;
          writeln!(w, "  ld a4, 96(sp)")?;
          writeln!(w, "  ld a5, 104(sp)")?;
          writeln!(w, "  ld a6, 112(sp)")?;
          writeln!(w, "  ld a7, 120(sp)")?;
          writeln!(w, "  addi sp, sp, 120")?;

          // return
          // tx = a0
          writeln!(w, "  mv {} ,a0", r)?;

          writeln!(w, "  ld a0, 64(sp)")?;
        },
        IrStmt::Load(_,_,reg, base, offset) => {
          writeln!(w, "  ld {}, {}({})", reg, offset, base)?;
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
