use accuyork_2024_02::*;

fn main() {
    let x = BinOp::Add;
    println!("{:?}", x);

    if let BinOp::Add = x {
        println!("Yes, it was Add");
    }

    if matches!(x, BinOp::Add) {
        println!("Yes, it was Add");
    }

    match x {
        BinOp::Add => println!("*** HERE ***"),
        BinOp::Subtract => println!("!!! not here !!!"),
        BinOp::Multiply => println!("!!! not here !!!"),
        BinOp::Divide => println!("!!! not here !!!"),
    }

    let x = Expr::Binary(
        Box::from(Expr::Number(1.)),
        BinOp::Add,
        Box::from(Expr::Number(1.)),
    );
    println!("{:?}", x);

    if let Expr::Binary(_, op, _) = &x {
        println!("Yes, it was a Binary {:?}", op);
    }

    if matches!(x, Expr::Binary(_,_,_)) {
        println!("Yes, it was some kind of a Binary expression");
    }

    match x {
        Expr::Binary(lhs, BinOp::Add, rhs) => println!("{:?} + {:?}", lhs, rhs),
        _ => panic!("not implemented"),
    }
}
