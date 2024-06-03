use prusti_contracts::*;

#[requires(b != 0)]
fn signed_division(a: i32, b: i32) -> i32 {
	a / b
}

fn main() {}