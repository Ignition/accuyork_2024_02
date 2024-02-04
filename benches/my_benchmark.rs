use std::collections::HashMap;

use criterion::{black_box, Criterion, criterion_group, criterion_main};
use inkwell::context::Context;
use inkwell::OptimizationLevel;

use accuyork_2024_02::ast::Expr;
use accuyork_2024_02::interpreter::Interpreter;
use accuyork_2024_02::jit::*;

pub fn criterion_benchmark(c: &mut Criterion) {

    // Example AST representing: (a + 2.0) * (b / c) + a + (a * 3.0) - c
    let ast = (Expr::ident("a") + Expr::num(2.0))
        * (Expr::ident("b") / Expr::ident("c"))
        + Expr::ident("a")
        + (Expr::ident("a") * Expr::num(3.0))
        - Expr::ident("c");

    let mut values = HashMap::new();
    values.insert("a".into(), 1.0);
    values.insert("b".into(), 2.0);
    values.insert("c".into(), 3.0);


    let mut group = c.benchmark_group("Evaluator");
    group.bench_function("Interpret",
                         |b| {
                             let evaluator = Interpreter::new(values.clone());
                             b.iter(|| evaluator.eval(black_box(&ast)))
                         });
    group.bench_function("JIT",
                         |b|
                             {
                                 let context = Context::create();
                                 let jitter = JitSystem::new(&context, OptimizationLevel::Aggressive);
                                 let function = jitter.build_function("demo_func", &ast);
                                 b.iter(|| function.call(black_box(&values)))
                             });
    group.bench_function("JITCold",
                         |b|
                             {
                                 b.iter(|| {
                                     let context = Context::create();
                                     let jitter = JitSystem::new(&context, OptimizationLevel::Aggressive);
                                     let function = jitter.build_function("demo_func", &ast);
                                     function.call(black_box(&values))
                                 })
                             });
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);