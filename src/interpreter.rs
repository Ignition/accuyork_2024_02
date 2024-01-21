use crate::ast::{Expr,BinOp};

// Note: takes `&Expr`
pub fn eval(ast: &Expr) -> f64 {
    match ast {
        // But we pass &Box<Expr>, tick automatic use of AsRef<Expr>
        Expr::Binary(lhs, BinOp::Add, rhs) => eval(lhs) + eval(rhs),
        Expr::Binary(lhs, BinOp::Subtract, rhs) => eval(lhs) - eval(rhs),
        Expr::Binary(lhs, BinOp::Multiply, rhs) => eval(lhs) * eval(rhs),
        Expr::Binary(lhs, BinOp::Divide, rhs) => eval(lhs) / eval(rhs),
        Expr::Number(val) => *val,
        _ => panic!("Not implemented"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let x = Expr::num(1.) + Expr::num(1.);
        assert_eq!(eval(&x), 2.0);
    }
}
