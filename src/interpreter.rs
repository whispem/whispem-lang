use crate::ast::{Expr, Stmt};
use std::collections::HashMap;

pub struct Interpreter {
    variables: HashMap<String, Value>,
}

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    String(String),
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    pub fn execute(&mut self, statements: Vec<Stmt>) {
        for stmt in statements {
            match stmt {
                Stmt::Let(name, expr) => {
                    let value = self.eval(expr);
                    self.variables.insert(name, value);
                }
                Stmt::Print(expr) => {
                    let value = self.eval(expr);
                    match value {
                        Value::Number(n) => println!("{}", n),
                        Value::String(s) => println!("{}", s),
                    }
                }
            }
        }
    }

    fn eval(&self, expr: Expr) -> Value {
        match expr {
            Expr::Number(n) => Value::Number(n),
            Expr::String(s) => Value::String(s),
            Expr::Variable(name) => self.variables.get(&name).unwrap().clone(),
            _ => panic!("Unsupported expression"),
        }
    }
}
