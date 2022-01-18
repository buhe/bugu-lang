
use crate::{ir::{IrStmt, IrProg, IrFunc}, regeister::Regeister, symbols::SymTab};

pub fn dataflow(p: &IrProg, table: &mut SymTab) -> IrProg {
    let mut r = Regeister::init();
    let mut funcs = vec![];
    for f in &p.funcs {
        let mut basic_blocks: Vec<BasicBlock> = vec![];
        let mut first = BasicBlock::new();
        for s in &f.stmts {
            match s {
                IrStmt::Label(label)=> {
                    if !first.stmts.is_empty() {
                        basic_blocks.push(first);
                    }
                    first = BasicBlock::new_with_label(label.clone());
                    first.stmts.push(s);
                }
                IrStmt::Jmp(_) | IrStmt::Beq(_,_) | IrStmt::Ret(_) => {
                    first.stmts.push(s);
                    basic_blocks.push(first);
                    first = BasicBlock::new();
                }
                _ => {
                    first.stmts.push(s);
                }
            }
            
        }
        // last
        if !first.stmts.is_empty() {
            basic_blocks.push(first);
        }
        println!("{:?} bb:\n{:?}", f.name, basic_blocks);

        // let mut next_bb = 0;
        // for mut bb in basic_blocks {
        //     next_bb += 1;
        //     let stmts = bb.stmts;
        //     for s in stmts {
        //         match s {
        //             IrStmt::Jmp(label)  => {
                        
        //             }
        //             IrStmt::Beq(_,label) => {
        //                 let edge = basic_blocks.get(next_bb);
        //                 bb.edges.push(edge.unwrap());
        //             }
        //             // IrStmt::Ret(_) => {

        //             // }
        //             _ => {
        //             }
        //         }
        //     }
        // }

        let mut stmts: Vec<IrStmt> = Vec::new();
        for s in &f.stmts {
         match s {
            IrStmt::Add(_, _, _) => {
                stmts.push(IrStmt::Add(r.near(), r.near(), r.eat()));
            }
            IrStmt::Sub(_, _, _) => {
                stmts.push(IrStmt::Sub(r.near(), r.near(), r.eat()));
            }
            IrStmt::Mul(_, _, _) => {
                stmts.push(IrStmt::Mul(r.near(), r.near(), r.eat()));
            }
            IrStmt::Div(_, _, _) => {
                stmts.push(IrStmt::Div(r.near(), r.near(), r.eat()));
            }
            IrStmt::Mod(_, _, _) => {
                stmts.push(IrStmt::Mod(r.near(), r.near(), r.eat()));
            }
            IrStmt::Neg(_, _) => {
                stmts.push(IrStmt::Neg(r.near(), r.eat()));
            }
            IrStmt::Or(_, _, _) => {
                stmts.push(IrStmt::Or(r.near(), r.near(), r.eat()));
            }
            IrStmt::And(_, _, _) => {
                stmts.push(IrStmt::And(r.near(), r.near(), r.eat()));
            }
            IrStmt::Equal(_, _, _, _) => {
                stmts.push(IrStmt::Equal(r.near(), r.near(), r.eat(), r.eat()));
            }
            IrStmt::NotEqual(_, _, _, _) => {
                stmts.push(IrStmt::NotEqual(r.near(), r.near(), r.eat(), r.eat()));
            }
            IrStmt::Lt(_, _, _) => {
                stmts.push(IrStmt::Lt(r.near(), r.near(), r.eat()));
            }
            IrStmt::Let(_, _, _, _, _, _) => {
                stmts.push(IrStmt::Let(r.near(), r.near(), r.eat(), r.eat(), r.eat(), r.eat()));
            }
            IrStmt::Gt(_, _, _) => {
                stmts.push(IrStmt::Gt(r.near(), r.near(), r.eat()));
            }
            IrStmt::Get(_, _, _, _, _, _) =>  {
                stmts.push(IrStmt::Get(r.near(), r.near(), r.eat(), r.eat(), r.eat(), r.eat()));
            }
            IrStmt::Ldc(n, _reg) => {
                stmts.push(IrStmt::Ldc(*n, r.eat()));
            },
            IrStmt::Ret(_) => {
                stmts.push(IrStmt::Ret(r.near()));
            },
            IrStmt::Assign(s, n, _) => {
                let near = r.near();
                let entry = table.entry(&s, &n);
                entry.and_modify(|s| {
                    if s.alloc_phy_reg == false {
                        let t = r.eat();
                        s.reg = Some(t.to_string());
                        s.alloc_phy_reg = true; 
                    } 
                });
                stmts.push(IrStmt::Assign(s.to_vec(), n.to_string(), near));
            }
            IrStmt::Beq(_,l) => {
                stmts.push(IrStmt::Beq(r.near(),l.to_string()));
            }
            IrStmt::Ref(s, n) => {
                // ref put near
                // println!("t-phy-reg:{:?}", table);
                let reg = table.get(&s, &n).reg.as_ref().unwrap();
                r.put_near(reg.clone());
                stmts.push(IrStmt::Ref(s.to_vec(), n.to_string()));
            }
            IrStmt::Call(regs,l,_) => {
                let mut args = vec![];
                for reg in regs {
                    args.push((r.near(),reg.1.to_string()));
                }
                stmts.push(IrStmt::Call(args, l.to_string(), r.eat()));
            }
            IrStmt::Load(scope,name,_, _, n) => {
                // let near = r.near();
                // get phy reg
                let reg = table.get(&scope, &name).reg.as_ref().unwrap();
                let eat = r.eat();
                stmts.push(IrStmt::Load(scope.clone(),name.clone(),eat, reg.clone(), *n))}
            ,
            IrStmt::LoadSymbol(_, v) => {
                // todo!! alloc phy reg
                let entry = table.entry(&vec![1], &v);
                let t = r.eat();
                entry.and_modify(|s| {
                    if s.alloc_phy_reg == false {
                        
                        s.reg = Some(t.to_string());
                        s.alloc_phy_reg = true; 
                    } 
                });
                stmts.push(IrStmt::LoadSymbol(t, v.clone()))
            }
            IrStmt::Jmp(_) | IrStmt::Label(_) => {stmts.push(s.clone());}
            IrStmt::Param(_,_,_) | IrStmt::DeclGlobal(_,_) | IrStmt::DeclGlobalArray(_,_) => unreachable!(),
            IrStmt::Alloc(scope,name,_, size) => {
                // alloc phy reg
                let entry = table.entry(&scope, &name);
                entry.and_modify(|s| {
                    if s.alloc_phy_reg == false {
                        let t = r.eat();
                        s.reg = Some(t.to_string());
                        s.alloc_phy_reg = true; 
                    } 
                });
                stmts.push(IrStmt::Alloc(scope.clone(),name.clone() ,r.eat(), *size));
            }
        }
        }
        for s in &f.params {
         match s {
             IrStmt::Param(_,_,_) => stmts.push(s.clone()),
             _ => unreachable!()
         }
        }
        funcs.push(IrFunc {
            name: f.name.clone(),
            stmts,
            params: f.params.clone(),
        });
    }
    
   
    IrProg {
        funcs, global_vars: p.global_vars.to_owned()
    }
}
/*
    1. 除出口语句外基本块中不含任何的 Branch、Beqz（条件为假时跳转）、Bnez（条件为真时跳转）或者 Return 等跳转语句（但可以包含 Call 语句）。
    2. 除入口语句外基本块中不含任何的 Label 标记，即不能跳转到基本块中间。
    3. 在满足前两条的前提下含有最多的连续语句，即基本块的头尾再纳入一条语句将会违反上面两条规则。
*/
/*
1）第一条指令。
2）跳转指令的目标指令。
3）紧跟跳转指令之后的指令。
*/
#[derive(Debug)]
pub struct BasicBlock<'a> {
   pub stmts: Vec<&'a IrStmt>,
   pub edges: Vec<&'a BasicBlock<'a>>,  // from none,branch,cond branch
   label: Option<String>,
}

impl<'a> BasicBlock<'a> {
    fn new() -> Self {
        Self {
            stmts: vec![],
            edges: vec![],
            label: None,
        }
    }

     fn new_with_label(label: String) -> Self {
        Self {
            stmts: vec![],
            edges: vec![],
            label: Some(label),
        }
    }
}
