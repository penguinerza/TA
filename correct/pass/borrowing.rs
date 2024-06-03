fn borrowing() -> i32 {
    let s1 = Box::new(4);
    let s2 = s1;
    *s2
}

fn main() {}
