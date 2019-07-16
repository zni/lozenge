use std::collections::HashMap;
use crate::ast::{Block, Expr, Literal, Type};

pub struct Interp {
    pub env: HashMap<String, i32>
}

impl Interp {
    pub fn new() -> Interp {
        Interp {
            env: HashMap::new()
        }
    }

    pub fn eval(&mut self, program: Block) {
        match program {
            Block::Program(p) => self.eval(*p),
            Block::Block(c, v, procs, s) => {
                self.extend_env_consts(*c);
                self.extend_env_vars(*v);
                for p in procs {
                    println!("{:?}", p);
                }
                println!("{:#?}", s);
            },
            _ => (),
        }
    }

    fn extend_env_consts(&mut self, block: Block) {
        match block {
            Block::ConstDecs(cds) => {
                for cd in cds {
                    if let Block::Const(Expr::Var(s), Expr::Literal(l)) = *cd {
                        if let Literal::Number(n) = l {
                            self.env.insert(s, n);
                        }
                    }
                }
            },
            _ => (),
        }
    }

    fn extend_env_vars(&mut self, block: Block) {
        if let Block::VarDecs(vds) = block {
            for v in vds {
                if let Expr::Var(s) = *v {
                    self.env.insert(s, 0);
                }
            }
        }
    }
}
