use prusti_contracts::*;

#[ensures(result < n)]
#[requires(n > 0)]
fn log2rec_match(n: u32) -> u32 {
	match n {
		1 => 0,
		_ => 1 + log2rec_match(n / 2),
	}
}

fn main() {}