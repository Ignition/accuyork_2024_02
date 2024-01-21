use accuyork_2024_02::ast::Expr;
use accuyork_2024_02::interpreter::Interpreter;

fn main() {
    let x = Expr::num(1.) + Expr::num(1.);
    println!("{:?}", Interpreter::default().eval(&x));
}
