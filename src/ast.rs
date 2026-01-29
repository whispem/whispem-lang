#[derive(Debug)]
pub enum Expr {
    Number(f64),
    String(String),
    Variable(String),
    Binary {
        left: Box<Expr>,
        op: char,
        right: Box<Expr>,
    },
}

#[derive(Debug)]
pub enum Stmt {
    Let(String, Expr),
    Print(Expr),
}
