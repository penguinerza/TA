pub fn print_data(ptr: *const i32) {
    unsafe {
        println!("Data: {}", *ptr); // Ada potensi null pointer dereference
    }
}

fn main() {
    let ptr: *const i32 = std::ptr::null();
    print_data(ptr);
}