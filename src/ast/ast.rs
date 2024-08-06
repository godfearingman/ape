use crate::tokeniser::token_enum::Operations;

#[derive(Debug)]
pub enum Expr {
    Number(f64),
    Variable(String),
    BinaryOp(Box<Expr>, Operations, Box<Expr>),
    UnaryOp(Box<Expr>, Operations),
    Assignment(Box<Expr>, String),
    ScopeExp(Vec<Expr>),
}

pub fn factorial(n: f64) -> f64 {
    if n < 0.0 {
        f64::NAN // Factorial is not defined for negative numbers
    } else if n == 0.0 || n == 1.0 {
        1.0
    } else {
        (1..=(n as u64)).fold(1.0, |acc, x| acc * x as f64)
    }
}
