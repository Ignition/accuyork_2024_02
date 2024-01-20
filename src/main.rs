use accuyork_2024_02::*;
use std::ops::Add;

fn main() {
    let x = Expr::Binary(
        Box::from(Expr::Number(1.)),
        BinOp::Add,
        Box::from(Expr::Number(1.)),
    );
    println!("{:?}", x);

    let x = Expr::Binary(Expr::Number(1.).into(), BinOp::Add, Expr::Number(1.).into());
    println!("{:?}", x);

    let x = Expr::Binary(Expr::num(1.).into(), BinOp::Add, Expr::num(1.).into());
    println!("{:?}", x);

    let x = Expr::binop(Expr::num(1.), BinOp::Add, Expr::num(1.));
    println!("{:?}", x);

    let x = Expr::num(1.).add(Expr::num(1.));
    println!("{:?}", x);

    let x = Expr::num(1.) + Expr::num(1.);
    println!("{:?}", x);
}
