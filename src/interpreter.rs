use crate::ast::{Expr, Stmt};
use std::collections::HashMap;
use std::io::{self, Write};
use std::fs;

pub struct Interpreter {
    globals: HashMap<String, Value>,
    functions: HashMap<String, FunctionDef>,
    call_stack: Vec<CallFrame>,
}

#[derive(Debug, Clone)]
struct FunctionDef {
    params: Vec<String>,
    body: Vec<Stmt>,
}

#[derive(Debug, Clone)]
struct CallFrame {
    locals: HashMap<String, Value>,
}

#[derive(Debug, Clone)]
enum Value {
    Number(f64),
    Bool(bool),
    String(String),
    Array(Vec<Value>),
    None,
}

enum ControlFlow {
    None,
    Return(Value),
    Break,
    Continue,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            globals: HashMap::new(),
            functions: HashMap::new(),
            call_stack: Vec::new(),
        }
    }

    pub fn execute(&mut self, program: Vec<Stmt>) {
        // First pass: collect function definitions
        for stmt in &program {
            if let Stmt::Function { name, params, body } = stmt {
                self.functions.insert(
                    name.clone(),
                    FunctionDef {
                        params: params.clone(),
                        body: body.clone(),
                    },
                );
            }
        }

        // Second pass: execute statements
        for stmt in program {
            if !matches!(stmt, Stmt::Function { .. }) {
                let _ = self.execute_stmt(stmt);
            }
        }
    }

    fn execute_stmt(&mut self, stmt: Stmt) -> ControlFlow {
        match stmt {
            Stmt::Let(name, expr) => {
                let value = self.eval(expr);
                if let Some(frame) = self.call_stack.last_mut() {
                    frame.locals.insert(name, value);
                } else {
                    self.globals.insert(name, value);
                }
                ControlFlow::None
            }
            Stmt::Print(expr) => {
                let value = self.eval(expr);
                println!("{}", self.format_value(value));
                ControlFlow::None
            }
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                let cond = self.eval(condition);
                if self.is_truthy(&cond) {
                    for stmt in then_branch {
                        match self.execute_stmt(stmt) {
                            ControlFlow::Return(val) => return ControlFlow::Return(val),
                            ControlFlow::Break => return ControlFlow::Break,
                            ControlFlow::Continue => return ControlFlow::Continue,
                            ControlFlow::None => {}
                        }
                    }
                } else if let Some(branch) = else_branch {
                    for stmt in branch {
                        match self.execute_stmt(stmt) {
                            ControlFlow::Return(val) => return ControlFlow::Return(val),
                            ControlFlow::Break => return ControlFlow::Break,
                            ControlFlow::Continue => return ControlFlow::Continue,
                            ControlFlow::None => {}
                        }
                    }
                }
                ControlFlow::None
            }
            Stmt::While { condition, body } => {
                let cond = condition.clone();
                let body_clone = body.clone();
                loop {
                    let cond_value = self.eval(cond.clone());
                    if !self.is_truthy(&cond_value) {
                        break;
                    }

                    for stmt in body_clone.clone() {
                        match self.execute_stmt(stmt) {
                            ControlFlow::Return(val) => return ControlFlow::Return(val),
                            ControlFlow::Break => return ControlFlow::None,
                            ControlFlow::Continue => break,
                            ControlFlow::None => {}
                        }
                    }
                }
                ControlFlow::None
            }
            Stmt::For { variable, iterable, body } => {
                let iter_value = self.eval(iterable);
                
                let items = match iter_value {
                    Value::Array(arr) => arr,
                    _ => panic!("For loop requires an array"),
                };

                for item in items {
                    // Set loop variable
                    if let Some(frame) = self.call_stack.last_mut() {
                        frame.locals.insert(variable.clone(), item);
                    } else {
                        self.globals.insert(variable.clone(), item);
                    }

                    // Execute body
                    for stmt in body.clone() {
                        match self.execute_stmt(stmt) {
                            ControlFlow::Return(val) => return ControlFlow::Return(val),
                            ControlFlow::Break => return ControlFlow::None,
                            ControlFlow::Continue => break,
                            ControlFlow::None => {}
                        }
                    }
                }
                ControlFlow::None
            }
            Stmt::Function { .. } => {
                // Already collected in first pass
                ControlFlow::None
            }
            Stmt::Return(expr) => {
                let value = if let Some(e) = expr {
                    self.eval(e)
                } else {
                    Value::None
                };
                ControlFlow::Return(value)
            }
            Stmt::Break => ControlFlow::Break,
            Stmt::Continue => ControlFlow::Continue,
            Stmt::IndexAssign { array, index, value } => {
                let idx = self.eval(index);
                let val = self.eval(value);

                let idx_num = match idx {
                    Value::Number(n) => n as usize,
                    _ => panic!("Array index must be a number"),
                };

                // Get mutable reference to the array
                let array_value = if let Some(frame) = self.call_stack.last_mut() {
                    frame.locals.get_mut(&array)
                } else {
                    self.globals.get_mut(&array)
                };

                if let Some(Value::Array(arr)) = array_value {
                    if idx_num >= arr.len() {
                        panic!("Array index {} out of bounds (array length: {})", idx_num, arr.len());
                    }
                    arr[idx_num] = val;
                } else {
                    panic!("Variable '{}' is not an array", array);
                }

                ControlFlow::None
            }
            Stmt::Expression(expr) => {
                self.eval(expr);
                ControlFlow::None
            }
        }
    }

    fn eval(&mut self, expr: Expr) -> Value {
        match expr {
            Expr::Number(n) => Value::Number(n),
            Expr::String(s) => Value::String(s),
            Expr::Bool(b) => Value::Bool(b),
            Expr::Array(elements) => {
                let values: Vec<Value> = elements.into_iter().map(|e| self.eval(e)).collect();
                Value::Array(values)
            }
            Expr::Index { array, index } => {
                let arr = self.eval(*array);
                let idx = self.eval(*index);

                let idx_num = match idx {
                    Value::Number(n) => n as usize,
                    _ => panic!("Array index must be a number"),
                };

                match arr {
                    Value::Array(elements) => {
                        if idx_num >= elements.len() {
                            panic!("Array index {} out of bounds (array length: {})", idx_num, elements.len());
                        }
                        elements[idx_num].clone()
                    }
                    _ => panic!("Cannot index non-array value"),
                }
            }
            Expr::Variable(name) => {
                // Check local scope first
                if let Some(frame) = self.call_stack.last() {
                    if let Some(value) = frame.locals.get(&name) {
                        return value.clone();
                    }
                }
                // Then check global scope
                self.globals
                    .get(&name)
                    .cloned()
                    .unwrap_or_else(|| panic!("Undefined variable: {}", name))
            }
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
            Expr::Call { name, arguments } => {
                // Check for built-in functions
                match name.as_str() {
                    "length" => {
                        if arguments.len() != 1 {
                            panic!("length() expects exactly 1 argument, got {}", arguments.len());
                        }
                        let arg = self.eval(arguments[0].clone());
                        return match arg {
                            Value::Array(arr) => Value::Number(arr.len() as f64),
                            Value::String(s) => Value::Number(s.len() as f64),
                            _ => panic!("length() requires an array or string"),
                        };
                    }
                    "push" => {
                        if arguments.len() != 2 {
                            panic!("push() expects exactly 2 arguments, got {}", arguments.len());
                        }
                        let arr = self.eval(arguments[0].clone());
                        let item = self.eval(arguments[1].clone());
                        
                        return match arr {
                            Value::Array(mut elements) => {
                                elements.push(item);
                                Value::Array(elements)
                            }
                            _ => panic!("push() requires an array as first argument"),
                        };
                    }
                    "pop" => {
                        if arguments.len() != 1 {
                            panic!("pop() expects exactly 1 argument, got {}", arguments.len());
                        }
                        let arr = self.eval(arguments[0].clone());
                        
                        return match arr {
                            Value::Array(mut elements) => {
                                if elements.is_empty() {
                                    panic!("Cannot pop from empty array");
                                }
                                elements.pop().unwrap()
                            }
                            _ => panic!("pop() requires an array"),
                        };
                    }
                    "reverse" => {
                        if arguments.len() != 1 {
                            panic!("reverse() expects exactly 1 argument, got {}", arguments.len());
                        }
                        let arr = self.eval(arguments[0].clone());
                        
                        return match arr {
                            Value::Array(mut elements) => {
                                elements.reverse();
                                Value::Array(elements)
                            }
                            _ => panic!("reverse() requires an array"),
                        };
                    }
                    "slice" => {
                        if arguments.len() != 3 {
                            panic!("slice() expects exactly 3 arguments (array, start, end), got {}", arguments.len());
                        }
                        let arr = self.eval(arguments[0].clone());
                        let start = self.eval(arguments[1].clone());
                        let end = self.eval(arguments[2].clone());
                        
                        let start_idx = match start {
                            Value::Number(n) => n as usize,
                            _ => panic!("slice() start index must be a number"),
                        };
                        
                        let end_idx = match end {
                            Value::Number(n) => n as usize,
                            _ => panic!("slice() end index must be a number"),
                        };
                        
                        return match arr {
                            Value::Array(elements) => {
                                if start_idx > end_idx {
                                    panic!("slice() start index cannot be greater than end index");
                                }
                                if end_idx > elements.len() {
                                    panic!("slice() end index {} out of bounds (array length: {})", end_idx, elements.len());
                                }
                                Value::Array(elements[start_idx..end_idx].to_vec())
                            }
                            _ => panic!("slice() requires an array"),
                        };
                    }
                    "range" => {
                        if arguments.len() != 2 {
                            panic!("range() expects exactly 2 arguments (start, end), got {}", arguments.len());
                        }
                        let start = self.eval(arguments[0].clone());
                        let end = self.eval(arguments[1].clone());
                        
                        let start_num = match start {
                            Value::Number(n) => n as i64,
                            _ => panic!("range() start must be a number"),
                        };
                        
                        let end_num = match end {
                            Value::Number(n) => n as i64,
                            _ => panic!("range() end must be a number"),
                        };
                        
                        let mut result = Vec::new();
                        for i in start_num..end_num {
                            result.push(Value::Number(i as f64));
                        }
                        
                        return Value::Array(result);
                    }
                    "input" => {
                        let prompt = if arguments.is_empty() {
                            String::new()
                        } else if arguments.len() == 1 {
                            let arg = self.eval(arguments[0].clone());
                            match arg {
                                Value::String(s) => s,
                                _ => panic!("input() prompt must be a string"),
                            }
                        } else {
                            panic!("input() expects 0 or 1 argument, got {}", arguments.len());
                        };
                        
                        if !prompt.is_empty() {
                            print!("{}", prompt);
                            io::stdout().flush().unwrap();
                        }
                        
                        let mut input = String::new();
                        io::stdin().read_line(&mut input).unwrap();
                        return Value::String(input.trim().to_string());
                    }
                    "read_file" => {
                        if arguments.len() != 1 {
                            panic!("read_file() expects exactly 1 argument, got {}", arguments.len());
                        }
                        let filename = self.eval(arguments[0].clone());
                        
                        let path = match filename {
                            Value::String(s) => s,
                            _ => panic!("read_file() requires a string filename"),
                        };
                        
                        match fs::read_to_string(&path) {
                            Ok(content) => return Value::String(content),
                            Err(e) => panic!("Failed to read file '{}': {}", path, e),
                        }
                    }
                    "write_file" => {
                        if arguments.len() != 2 {
                            panic!("write_file() expects exactly 2 arguments (filename, content), got {}", arguments.len());
                        }
                        let filename = self.eval(arguments[0].clone());
                        let content = self.eval(arguments[1].clone());
                        
                        let path = match filename {
                            Value::String(s) => s,
                            _ => panic!("write_file() filename must be a string"),
                        };
                        
                        let text = match content {
                            Value::String(s) => s,
                            _ => self.format_value(content),
                        };
                        
                        match fs::write(&path, text) {
                            Ok(_) => return Value::None,
                            Err(e) => panic!("Failed to write file '{}': {}", path, e),
                        }
                    }
                    _ => {}
                }

                // Get the function definition
                let func = self
                    .functions
                    .get(&name)
                    .cloned()
                    .unwrap_or_else(|| panic!("Undefined function: {}", name));

                // Check argument count
                if arguments.len() != func.params.len() {
                    panic!(
                        "Function {} expected {} arguments, got {}",
                        name,
                        func.params.len(),
                        arguments.len()
                    );
                }

                // Evaluate arguments
                let arg_values: Vec<Value> = arguments.into_iter().map(|arg| self.eval(arg)).collect();

                // Create new call frame
                let mut frame = CallFrame {
                    locals: HashMap::new(),
                };

                // Bind parameters
                for (param, value) in func.params.iter().zip(arg_values.iter()) {
                    frame.locals.insert(param.clone(), value.clone());
                }

                // Push frame onto call stack
                self.call_stack.push(frame);

                // Execute function body
                let mut return_value = Value::None;
                for stmt in func.body {
                    match self.execute_stmt(stmt) {
                        ControlFlow::Return(val) => {
                            return_value = val;
                            break;
                        }
                        ControlFlow::Break => panic!("'break' outside of loop"),
                        ControlFlow::Continue => panic!("'continue' outside of loop"),
                        ControlFlow::None => {}
                    }
                }

                // Pop frame from call stack
                self.call_stack.pop();

                return_value
            }
        }
    }

    fn eval_binary(&self, left: Value, op: String, right: Value) -> Value {
        match (&left, op.as_str(), &right) {
            // Arithmetic
            (Value::Number(a), "+", Value::Number(b)) => Value::Number(a + b),
            (Value::Number(a), "-", Value::Number(b)) => Value::Number(a - b),
            (Value::Number(a), "*", Value::Number(b)) => Value::Number(a * b),
            (Value::Number(a), "/", Value::Number(b)) => {
                if *b == 0.0 {
                    panic!("Division by zero");
                }
                Value::Number(a / b)
            }

            // String concatenation
            (Value::String(a), "+", Value::String(b)) => {
                Value::String(format!("{}{}", a, b))
            }

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

            _ => panic!(
                "Unsupported binary operation: {:?} {} {:?}",
                left, op, right
            ),
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
            Value::Array(arr) => !arr.is_empty(),
            Value::None => false,
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
            Value::Array(elements) => {
                let formatted: Vec<String> = elements
                    .into_iter()
                    .map(|v| self.format_value(v))
                    .collect();
                format!("[{}]", formatted.join(", "))
            }
            Value::None => String::new(),
        }
    }
}
