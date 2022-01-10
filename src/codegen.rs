use crate::{ir::*, regeister::Regeister, symbols::{SymTab}};
use std::io::{Result, Write};

pub fn write_asm(p: &IrProg, bl: &mut BranchLabel ,table: &mut SymTab, w: &mut impl Write) -> Result<()> {
  let f = &p.func;
  let mut r = Regeister::init();
  writeln!(w, ".global {}", f.name)?;
  writeln!(w, "{}:", f.name)?;
  for s in &f.stmts {
    match s {
      IrStmt::Neg => {
        let t1 = r.near();
        let t2 = r.eat();
        writeln!(w, "  sub {}, x0, {}", t2,t1)?;
      }
      IrStmt::Ldc(x) => {
        let t = r.eat();
        writeln!(w, "  addi {}, x0, {}", t, x)?; //todo
      }
      IrStmt::Ret => {
        let t = r.near();
        writeln!(w, "  addi a0, {}, 0", t)?;
        writeln!(w, "  jalr x0, x1, 0")?;
      }
      IrStmt::Add => {
        let t1 = r.near();
        let t2 = r.near();
        let t = r.eat();
        writeln!(w, "  add {} ,{} ,{}", t, t2, t1)?;
      }
      IrStmt::Sub => {
        let t1 = r.near();
        let t2 = r.near();
        let t = r.eat();
        writeln!(w, "  sub {} ,{} ,{}", t, t2, t1)?;
      }
      IrStmt::Div => {
        let t1 = r.near();
        let t2 = r.near();
        let t = r.eat();
        writeln!(w, "  div {} ,{} ,{}", t, t2, t1)?; //todo
      }
      IrStmt::Mod => {
        let t1 = r.near();
        let t2 = r.near();
        let t = r.eat();
        writeln!(w, "  mod {} ,{} ,{}", t, t2, t1)?; // todo
      }
      IrStmt::Mul => {
        
        let t1 = r.near();
        let t2 = r.near();
        let t = r.eat();
        writeln!(w, "  mul {} ,{} ,{}", t, t2, t1)?; //todo
      }
        IrStmt::Or => {
          // or t3,t1,t2 ; snez t3,t3
          let t1 = r.near();
          let t2 = r.near();
          let t = r.eat();
          writeln!(w, "  or {} ,{} ,{}", t, t2, t1)?;
          writeln!(w, "  sltu {} ,x0 ,{}", t, t)?;
        }
        IrStmt::And => {
          // snez d, s1; sub d, zero, d; and d, d, s2; snez d, d;
          let t1 = r.near(); // s2
          let t2 = r.near(); // s1
          let t = r.eat(); // d
          writeln!(w, "  sltu {} ,x0 ,{}", t, t2)?;
          writeln!(w, "  sub {} ,zero ,{}", t, t)?;
          writeln!(w, "  and {} ,{} ,{}", t, t, t1)?;
          writeln!(w, "  sltu {} ,x0 ,{}", t, t)?;
        }
        IrStmt::Equal => {
          // seqz t1,t2	Set EQual to Zero : if t2 == 0 then set t1 to 1 else 0
          let t1 = r.near();
          let t2 = r.near();
          let t = r.eat();
          writeln!(w, "  sub {} ,{} ,{}", t, t2, t1)?;
          let t3 = r.eat();
          let t4 = r.eat();

          writeln!(w, "  addi {} ,x0 ,1", t4)?;
          writeln!(w, "  sltu {} ,{} ,{}", t3, t, t4)?;
        }
        IrStmt::NotEqual => {
          // snez t1,t2	Set Not Equal to Zero : if t2 != 0 then set t1 to 1 else 0
          let t1 = r.near();
          let t2 = r.near();
          let t = r.eat();
          writeln!(w, "  sub {} ,{} ,{}", t, t2, t1)?;
          let t3 = r.eat();
          writeln!(w, "  sltu {} ,x0 ,{}", t3, t)?;
        }
        IrStmt::Lt => {
          let t1 = r.near();
          let t2 = r.near();
          let t = r.eat();
          writeln!(w, "  slt {} ,{} ,{}", t, t2, t1)?;
        }
        IrStmt::Let => {
          // eq
          let t1 = r.near();
          let t2 = r.near();
          let t = r.eat();
          writeln!(w, "  sub {} ,{} ,{}", t, t2, t1)?;
          let t3 = r.eat();
          let t6 = r.eat();

          writeln!(w, "  addi {} ,x0 ,1", t6)?;
          writeln!(w, "  sltu {} ,{} ,{}", t3, t, t6)?;
          // lt
          let t4 = r.eat();
          writeln!(w, "  slt {} ,{} ,{}", t4, t2, t1)?;
          // or
          let t5 = r.eat();
          writeln!(w, "  or {} ,{} ,{}", t5, t3, t4)?;
          writeln!(w, "  sltu {} ,x0 ,{}", t5, t5)?;
        }
        IrStmt::Gt => {// todo slt
          let t1 = r.near();
          let t2 = r.near();
          let t = r.eat();
          writeln!(w, "  sgt {} ,{} ,{}", t, t2, t1)?;
        }
        IrStmt::Get => {// todo slt
          // eq
          let t1 = r.near();
          let t2 = r.near();
          let t = r.eat();
          writeln!(w, "  sub {} ,{} ,{}", t, t2, t1)?;
          let t3 = r.eat();
          writeln!(w, "  seqz {} ,{}", t3, t)?;
          // gt
          let t4 = r.eat();
          writeln!(w, "  sgt {} ,{} ,{}", t4, t2, t1)?;
          // or
          let t5 = r.eat();
          writeln!(w, "  or {} ,{} ,{}", t5, t3, t4)?;
          writeln!(w, "  snez {} ,{}", t5, t5)?;
        }
        IrStmt::Assign(id) => {
          let t2 = r.near();// todo, noy use near api
          
          // save to table
          let entry = table.entry(id);
          entry.and_modify(|s| {
            if s.reg.is_none() {
              let t = r.eat();
              s.reg = Some(t.to_string()) 
            } 
          });
          let s = table.get(id);
          writeln!(w, "  addi {} ,{}, 0", s.reg.as_ref().unwrap(), t2)?;
        },
        IrStmt::Ref(id) => {
          let reg = table.get(id).reg.as_ref().unwrap();
          r.put_near(reg.clone());
        },
        IrStmt::Beq => {
          let t = r.near();
          let l = bl.label();
          // assert_eq!()
          writeln!(w, "  beq {} ,x0 ,{}", t, l.name)?;
        },
        IrStmt::Jmp => {
          let l = bl.label();
          writeln!(w, "  jal x0, {}", l.name)?;
        },
        IrStmt::Label(label) => {
          writeln!(w, "{}:", label)?;
        },
    }
  }
  Ok(())
}
