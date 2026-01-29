use std::collections::HashMap;
use crate::ast::{Statement, Expression};

pub struct Interpreter {
    variables: HashMap<String, f64>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    pub fn execute(&mut self, statements: Vec<Statement>) {
        for stmt in statements {
            self.execute_statement(stmt);
        }
    }

    fn execute_statement(&mut self, stmt: Statement) {
        match stmt {
            Statement::Let(name, expr) => {
                let value = self.eval(expr);
                self.variables.insert(name, value);
            }
            Statement::Print(expr) => {
                let value = self.eval(expr);
                println!("{}", value);
            }
        }
    }

    fn eval(&self, expr: Expression) -> f64 {
        match expr {
            Expression::Number(n) => n,
            Expression::Identifier(name) => {
                *self.variables.get(&name).unwrap_or(&0.0)
            }
        }
    }
}
