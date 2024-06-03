use prusti_contracts::*;

#[ensures(result < n)]
#[requires(n > 0)]
fn log2rec(n: u32) -> u32 {
	if n == 1{
		return 0;
	} else {
		return 1 + log2rec(n / 2);
	}
}

fn main() {}