#[derive(Debug, Clone)]
pub enum Expr {
    Number(f64),
    String(String),
    Bool(bool),
    Variable(String),
    Binary {
        left: Box<Expr>,
        op: String,
        right: Box<Expr>,
    },
    Logical {
        left: Box<Expr>,
        op: String,
        right: Box<Expr>,
    },
    Unary {
        op: String,
        operand: Box<Expr>,
    },
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Let(String, Expr),
    Print(Expr),
    If {
        condition: Expr,
        then_branch: Vec<Stmt>,
        else_branch: Option<Vec<Stmt>>,
    },
    While {
        condition: Expr,
        body: Vec<Stmt>,
    },
}
