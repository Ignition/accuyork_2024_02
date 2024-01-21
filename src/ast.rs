use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug)]
pub enum Expr {
    Binary(Box<Expr>, BinOp, Box<Expr>),
    Number(f64),
    Identifier(String),
}

#[derive(Debug)]
pub enum BinOp {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl Expr {
    #[must_use]
    pub const fn num(val: f64) -> Self {
        Self::Number(val)
    }
    #[must_use]
    pub fn binop(lhs: Self, op: BinOp, rhs: Self) -> Self {
        Self::Binary(lhs.into(), op, rhs.into())
    }
    #[must_use]
    pub fn ident(val: &str) -> Self {
        Self::Identifier(val.into())
    }
}

impl Add for Expr {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::binop(self, BinOp::Add, rhs)
    }
}

impl Sub for Expr {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::binop(self, BinOp::Subtract, rhs)
    }
}

impl Mul for Expr {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::binop(self, BinOp::Multiply, rhs)
    }
}

impl Div for Expr {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self::binop(self, BinOp::Divide, rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let x = Expr::num(1.) + Expr::num(1.);
        assert_eq!(format!("{:?}", x), "Binary(Number(1.0), Add, Number(1.0))")
    }
}
