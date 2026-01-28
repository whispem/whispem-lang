use crate::ast::*;
use std::collections::HashMap;

pub struct Interpreter {
    env: HashMap<String, f64>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            env: HashMap::new(),
        }
    }

    pub fn run(&mut self, stmts: Vec<Stmt>) {
        for stmt in stmts {
            self.exec(stmt);
        }
    }

    fn exec(&mut self, stmt: Stmt) {
        match stmt {
            Stmt::Let { name, value } => {
                let v = self.eval(value);
                self.env.insert(name, v);
            }
            Stmt::Print(expr) => {
                let v = self.eval(expr);
                println!("{}", v);
            }
        }
    }

    fn eval(&mut self, expr: Expr) -> f64 {
        match expr {
            Expr::Number(n) => n,
            Expr::Variable(name) => *self.env.get(&name).expect("Undefined variable"),
            Expr::Binary { left, op, right } => {
                let l = self.eval(*left);
                let r = self.eval(*right);
                match op {
                    Operator::Plus => l + r,
                    Operator::Minus => l - r,
                    Operator::Star => l * r,
                    Operator::Slash => l / r,
                }
            }
        }
    }
}
