use std::collections::HashMap;
use crate::ast::{Expr, Stmt};

pub struct Interpreter {
    variables: HashMap<String, f64>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    pub fn execute(&mut self, statements: Vec<Stmt>) {
        for stmt in statements {
            self.execute_stmt(stmt);
        }
    }

    fn execute_stmt(&mut self, stmt: Stmt) {
        match stmt {
            Stmt::Let(name, expr) => {
                let value = self.eval(expr);
                self.variables.insert(name, value);
            }
            Stmt::Print(expr) => {
                let value = self.eval(expr);
                println!("{}", value);
            }
        }
    }

    fn eval(&self, expr: Expr) -> f64 {
        match expr {
            Expr::Number(n) => n,

            Expr::Variable(name) => {
                *self.variables.get(&name).unwrap_or(&0.0)
            }

            Expr::Binary { left, op, right } => {
                let l = self.eval(*left);
                let r = self.eval(*right);

                match op {
                    '+' => l + r,
                    '-' => l - r,
                    '*' => l * r,
                    '/' => l / r,
                    _ => 0.0,
                }
            }

            Expr::String(_) => {
                0.0 // strings not evaluated yet
            }
        }
    }
}
