use accuyork_2024_02::ast::*;
use accuyork_2024_02::interpreter::*;

fn main() {
    let x = Expr::num(1.) + Expr::num(1.);
    println!("{:?}", eval(&x));
}
