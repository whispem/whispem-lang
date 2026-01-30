use crate::ast::{Expr, Stmt};
use std::collections::HashMap;

pub struct Interpreter {
    env: HashMap<String, Value>,
}

#[derive(Debug, Clone)]
enum Value {
    Number(f64),
    String(String),
    Bool(bool),
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            env: HashMap::new(),
        }
    }

    pub fn execute(&mut self, program: Vec<Stmt>) {
        for stmt in program {
            self.execute_stmt(stmt);
        }
    }

    fn execute_stmt(&mut self, stmt: Stmt) {
        match stmt {
            Stmt::Let(name, expr) => {
                let value = self.eval_expr(expr);
                self.env.insert(name, value);
            }
            Stmt::Print(expr) => {
                let value = self.eval_expr(expr);
                match value {
                    Value::Number(n) => println!("{}", n),
                    Value::String(s) => println!("{}", s),
                    Value::Bool(b) => println!("{}", b),
                }
            }
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                let cond = self.eval_expr(condition);
                if matches!(cond, Value::Bool(true)) {
                    for stmt in then_branch {
                        self.execute_stmt(stmt);
                    }
                } else if let Some(branch) = else_branch {
                    for stmt in branch {
                        self.execute_stmt(stmt);
                    }
                }
            }
        }
    }

    fn eval_expr(&mut self, expr: Expr) -> Value {
        match expr {
            Expr::Number(n) => Value::Number(n),
            Expr::String(s) => Value::String(s),
            Expr::Bool(b) => Value::Bool(b),
            Expr::Variable(name) => self.env.get(&name).unwrap().clone(),
            Expr::Binary { left, op, right } => {
                let l = self.eval_expr(*left);
                let r = self.eval_expr(*right);

                match (l, r, op.as_str()) {
                    (Value::Number(a), Value::Number(b), "Less") => Value::Bool(a < b),
                    (Value::Number(a), Value::Number(b), "Greater") => Value::Bool(a > b),
                    (Value::Number(a), Value::Number(b), "EqualEqual") => Value::Bool(a == b),
                    _ => panic!("Unsupported binary operation"),
                }
            }
        }
    }
}
