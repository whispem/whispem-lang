use std::collections::HashMap;
use crate::ast::{Program, Statement, Expression};

pub struct Interpreter {
    variables: HashMap<String, f64>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    pub fn execute(&mut self, program: Program) {
        for statement in program.statements {
            self.execute_statement(statement);
        }
    }

    fn execute_statement(&mut self, statement: Statement) {
        match statement {
            Statement::Let(name, expr) => {
                let value = self.evaluate_expression(expr);
                self.variables.insert(name, value);
            }
            Statement::Print(expr) => {
                let value = self.evaluate_expression(expr);
                println!("{}", value);
            }
        }
    }

    fn evaluate_expression(&self, expr: Expression) -> f64 {
        match expr {
            Expression::Number(value) => value,
            Expression::Variable(name) => {
                *self.variables.get(&name).unwrap_or(&0.0)
            }
        }
    }
}
