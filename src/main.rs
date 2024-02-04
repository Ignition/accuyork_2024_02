#[derive(Clone)]
struct S {}

fn by_mut(_: &mut S) {}

fn by_ref(_: &S) {}

fn by_val(_: S) {}

fn main() {
    let mut s = S {};
    by_mut(&mut s);
    by_ref(&s);
    by_val(s);

    // You can have as many immutable references, but only one mutable reference

    // let s1_ref = &s;
    // let s2_ref = &s;
    // let s3_ref = &s;
    // by_mut(&mut s);
    // by_ref(&s1_ref);
    // by_ref(&s3_ref);
    // by_ref(&s2_ref);
}
