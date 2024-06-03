use prusti_contracts::*;

#[requires(index < arr.len())]
#[ensures(result == arr[index])]
fn get_value(arr: &[i32], index: usize) -> i32 {
    arr[index]
}

fn main() {
    let arr = [1, 2, 3, 4, 5];
    let index = 10;
    let value = get_value(&arr, index); // error karena tidak sesuai precondition
}
