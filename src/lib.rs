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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(true, true)
    }
}
