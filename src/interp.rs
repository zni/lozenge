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
            Block::Block(consts, vars, procs, stmts) => {
                self.extend_env_consts(*consts);
                self.extend_env_vars(*vars);
                self.eval(*stmts);
            },
            Block::Begin(stmts) => {
                for stmt in stmts {
                    println!("{:?}", stmt);
                    self.eval(*stmt);
                }
            },
            Block::If(expr, block) => {
                let val = self.eval_expr(expr);
                if val > 0 {
                    self.eval(*block);
                }
            },
            Block::Assign(var, expr) => {
                let val = self.eval_expr(expr);
                if let Expr::Var(s) = var {
                    self.env.insert(s, val);
                }
            },
            Block::WriteLn(expr) => {
                println!("{}", self.eval_expr(expr));
            },
            _ => (),
        }
    }

    fn eval_expr(&mut self, expr: Expr) -> i32 {
        match expr {
            Expr::Literal(l) => {
                if let Literal::Number(n) = l {
                    return n
                } else {
                    return 0
                }
            },
            Expr::Var(v) => {
                // TODO Check if variable exists.
                return *self.env.get(&v).unwrap();
            },
            Expr::PrefixExpr(prefix, expr) => {
                if prefix.is_some() {
                    let prefix = prefix.unwrap();
                    match prefix {
                        Type::Minus => -1 * self.eval_expr(*expr),
                        Type::Plus => 1 * self.eval_expr(*expr),
                        _ => self.eval_expr(*expr),
                    }
                } else {
                    self.eval_expr(*expr)
                }
            },
            Expr::Expr(left, sign, right) => {
                match sign {
                    Type::Plus => self.eval_expr(*left) + self.eval_expr(*right),
                    Type::Minus => self.eval_expr(*left) - self.eval_expr(*right),
                    Type::Star => self.eval_expr(*left) * self.eval_expr(*right),
                    Type::Slash => self.eval_expr(*left) / self.eval_expr(*right),
                    _ => 0
                }
            },
            Expr::OddExpr(expr) => {
                let val = self.eval_expr(*expr);
                if val % 2 == 1 {
                    return 1;
                } else {
                    return 0;
                }
            }
            _ => 0,
        }
    }

    fn extend_env_consts(&mut self, block: Block) {
        if let Block::ConstDecs(cds) = block {
            for cd in cds {
                if let Block::Const(Expr::Var(s),
                                    Expr::Literal(l)) = *cd {
                    if let Literal::Number(n) = l {
                        self.env.insert(s, n);
                    }
                }
            }
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
