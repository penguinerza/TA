use prusti_contracts::*;

#[requires(b != 0)]
fn unsigned_division(a: u32, b: u32) -> u32 {
    a / b
}

fn main() {
    unsigned_division(5, 0);
}
