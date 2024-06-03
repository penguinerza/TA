use prusti_contracts::*;

#[ensures(result < n)]
#[requires(0 < n)]
fn log2_loop(n: u32) -> u32 {
    let mut res = 0;
    let mut n = n;

    while n > 1 {
        n /= 2;
        // res tidak akan lebih dari 32 tetapi dianggap mungkin akan terjadi overflow
        res += 1;
    }
    res
}

fn main() {
    // let n = 10;
    // let result = log2(n);
    // println!("log2({}) = {}", n, result);
}
