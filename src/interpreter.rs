use std::collections::HashMap;

use crate::ast::{BinOp, Expr};

#[derive(Default)]
pub struct Interpreter {
    values: HashMap<String, f64>,
}

impl Interpreter {
    pub fn new(values: HashMap<String, f64>) -> Self {
        Self { values }
    }

    pub fn eval(&self, ast: &Expr) -> f64 {
        match ast {
            Expr::Binary(lhs, BinOp::Add, rhs) => self.eval(lhs) + self.eval(rhs),
            Expr::Binary(lhs, BinOp::Subtract, rhs) => self.eval(lhs) - self.eval(rhs),
            Expr::Binary(lhs, BinOp::Multiply, rhs) => self.eval(lhs) * self.eval(rhs),
            Expr::Binary(lhs, BinOp::Divide, rhs) => self.eval(lhs) / self.eval(rhs),
            Expr::Number(val) => *val,
            Expr::Identifier(name) => *self.values.get(name).unwrap(), // What about errors?
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_plus_one() {
        let x = Expr::num(1.) + Expr::num(1.);
        let interpreter = Interpreter::default();
        assert_eq!(interpreter.eval(&x), 2.0);
    }

    #[test]
    fn a_plus_40_where_a_is_2() {
        let x = Expr::ident("a") + Expr::num(40.);

        let mut values = HashMap::new();
        values.insert("a".to_string(), 2.);

        assert_eq!(Interpreter::new(values).eval(&x), 42.0);
    }
}
