use prusti_contracts::*;

#[requires(value <= std::u32::MAX as u64)]
#[ensures(result == value as u32)]
fn convert_to_u32(value: u64) -> u32 {
    value as u32
}

fn main() {
    let value: u64 = 4294967296; // 2^32, melebihi kapasitas u32
    let result = convert_to_u32(value); // error karena tidak sesuai precondition
}