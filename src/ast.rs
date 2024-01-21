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
    pub fn num(val: f64) -> Expr {
        Expr::Number(val)
    }

    pub fn binop(lhs: Expr, op: BinOp, rhs: Expr) -> Expr {
        Expr::Binary(lhs.into(), op, rhs.into())
    }

    pub fn ident(val: &str) -> Expr {
        Expr::Identifier(val.into())
    }
}

impl Add for Expr {
    type Output = Expr;

    fn add(self, rhs: Self) -> Self::Output {
        Expr::binop(self, BinOp::Add, rhs)
    }
}

impl Sub for Expr {
    type Output = Expr;

    fn sub(self, rhs: Self) -> Self::Output {
        Expr::binop(self, BinOp::Subtract, rhs)
    }
}

impl Mul for Expr {
    type Output = Expr;

    fn mul(self, rhs: Self) -> Self::Output {
        Expr::binop(self, BinOp::Multiply, rhs)
    }
}

impl Div for Expr {
    type Output = Expr;

    fn div(self, rhs: Self) -> Self::Output {
        Expr::binop(self, BinOp::Divide, rhs)
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
