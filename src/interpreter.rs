use crate::ast::{Expr, Stmt};
use std::collections::HashMap;

pub struct Interpreter {
    variables: HashMap<String, Value>,
}

#[derive(Debug, Clone)]
enum Value {
    Number(f64),
    Bool(bool),
    String(String),
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
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
                let value = self.eval(expr);
                self.variables.insert(name, value);
            }
            Stmt::Print(expr) => {
                let value = self.eval(expr);
                println!("{}", self.format_value(value));
            }
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                let cond = self.eval(condition);
                if self.is_truthy(&cond) {
                    for stmt in then_branch {
                        self.execute_stmt(stmt);
                    }
                } else if let Some(branch) = else_branch {
                    for stmt in branch {
                        self.execute_stmt(stmt);
                    }
                }
            }
            Stmt::While { condition, body } => {
                while self.is_truthy(&self.eval(condition.clone())) {
                    for stmt in body.clone() {
                        self.execute_stmt(stmt);
                    }
                }
            }
        }
    }

    fn eval(&mut self, expr: Expr) -> Value {
        match expr {
            Expr::Number(n) => Value::Number(n),
            Expr::String(s) => Value::String(s),
            Expr::Bool(b) => Value::Bool(b),
            Expr::Variable(name) => self
                .variables
                .get(&name)
                .cloned()
                .unwrap_or_else(|| panic!("Undefined variable: {}", name)),
            Expr::Binary { left, op, right } => {
                let l = self.eval(*left);
                let r = self.eval(*right);
                self.eval_binary(l, op, r)
            }
            Expr::Logical { left, op, right } => {
                let l = self.eval(*left);
                
                // Short-circuit evaluation
                if op == "or" {
                    if self.is_truthy(&l) {
                        return l;
                    }
                    return self.eval(*right);
                } else if op == "and" {
                    if !self.is_truthy(&l) {
                        return l;
                    }
                    return self.eval(*right);
                }
                
                panic!("Unknown logical operator: {}", op);
            }
            Expr::Unary { op, operand } => {
                let value = self.eval(*operand);
                self.eval_unary(op, value)
            }
        }
    }

    fn eval_binary(&self, left: Value, op: String, right: Value) -> Value {
        match (left, op.as_str(), right) {
            // Arithmetic
            (Value::Number(a), "+", Value::Number(b)) => Value::Number(a + b),
            (Value::Number(a), "-", Value::Number(b)) => Value::Number(a - b),
            (Value::Number(a), "*", Value::Number(b)) => Value::Number(a * b),
            (Value::Number(a), "/", Value::Number(b)) => Value::Number(a / b),

            // Comparison (numbers)
            (Value::Number(a), "Less", Value::Number(b)) => Value::Bool(a < b),
            (Value::Number(a), "LessEqual", Value::Number(b)) => Value::Bool(a <= b),
            (Value::Number(a), "Greater", Value::Number(b)) => Value::Bool(a > b),
            (Value::Number(a), "GreaterEqual", Value::Number(b)) => Value::Bool(a >= b),
            (Value::Number(a), "EqualEqual", Value::Number(b)) => Value::Bool(a == b),
            (Value::Number(a), "BangEqual", Value::Number(b)) => Value::Bool(a != b),

            // Comparison (booleans)
            (Value::Bool(a), "EqualEqual", Value::Bool(b)) => Value::Bool(a == b),
            (Value::Bool(a), "BangEqual", Value::Bool(b)) => Value::Bool(a != b),

            // Comparison (strings)
            (Value::String(a), "EqualEqual", Value::String(b)) => Value::Bool(a == b),
            (Value::String(a), "BangEqual", Value::String(b)) => Value::Bool(a != b),

            _ => panic!("Unsupported binary operation: {:?} {} {:?}", left, op, right),
        }
    }

    fn eval_unary(&self, op: String, value: Value) -> Value {
        match op.as_str() {
            "not" | "!" => Value::Bool(!self.is_truthy(&value)),
            "-" => match value {
                Value::Number(n) => Value::Number(-n),
                _ => panic!("Cannot negate non-number"),
            },
            _ => panic!("Unknown unary operator: {}", op),
        }
    }

    fn is_truthy(&self, value: &Value) -> bool {
        match value {
            Value::Bool(b) => *b,
            Value::Number(n) => *n != 0.0,
            Value::String(s) => !s.is_empty(),
        }
    }

    fn format_value(&self, value: Value) -> String {
        match value {
            Value::Number(n) => {
                if n.fract() == 0.0 {
                    format!("{}", n as i64)
                } else {
                    n.to_string()
                }
            }
            Value::Bool(b) => b.to_string(),
            Value::String(s) => s,
        }
    }
}
