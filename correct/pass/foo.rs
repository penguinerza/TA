use prusti_contracts::*;

#[ensures(result.2 == result.0 + result.1)]
fn foo() -> (i32, i32, i32) {
    let a = 5;
    let b = 7;
    let result = a + b;
    (a, b, result)
}

fn main() {}
