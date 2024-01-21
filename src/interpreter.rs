use std::collections::HashMap;

use crate::ast::{BinOp, Expr};

#[derive(Default)]
pub struct Interpreter {
    values: HashMap<String, f64>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    MissingIdentifier,
}

impl Interpreter {
    #[must_use]
    pub const fn new(values: HashMap<String, f64>) -> Self {
        Self { values }
    }

    /// # Errors
    ///
    /// Will return `MissingIdentifier` if identifier can not be looked up
    pub fn eval(&self, ast: &Expr) -> Result<f64, Error> {
        match ast {
            Expr::Binary(lhs, BinOp::Add, rhs) => Ok(self.eval(lhs)? + self.eval(rhs)?),
            Expr::Binary(lhs, BinOp::Subtract, rhs) => Ok(self.eval(lhs)? - self.eval(rhs)?),
            Expr::Binary(lhs, BinOp::Multiply, rhs) => Ok(self.eval(lhs)? * self.eval(rhs)?),
            Expr::Binary(lhs, BinOp::Divide, rhs) => Ok(self.eval(lhs)? / self.eval(rhs)?),
            Expr::Number(val) => Ok(*val),
            Expr::Identifier(name) => self.fetch_identifiers_value(name),
        }
    }

    fn fetch_identifiers_value(&self, name: &String) -> Result<f64, Error> {
        self.values
            .get(name)
            .map_or(Err(Error::MissingIdentifier), |value| Ok(*value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_plus_one() {
        let x = Expr::num(1.) + Expr::num(1.);
        let interpreter = Interpreter::default();
        assert_eq!(interpreter.eval(&x), Ok(2.0));
    }

    #[test]
    fn a_plus_40_where_a_is_2() {
        let x = Expr::ident("a") + Expr::num(40.);

        let mut values = HashMap::new();
        values.insert("a".to_string(), 2.);

        assert_eq!(Interpreter::new(values).eval(&x), Ok(42.0));
    }
}
