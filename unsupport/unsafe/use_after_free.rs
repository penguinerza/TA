struct MyStruct {
    data: i32,
}

fn main() {
    let ptr = Box::into_raw(Box::new(MyStruct { data: 42 }));
    unsafe {
        // Mengakses data setelah di-free
        let data = (*ptr).data; 

        Box::from_raw(ptr); // mem::forget bisa digunakan untuk simulasi free
        let data_after_free = (*ptr).data; // Prusti seharusnya mendeteksi use-after-free di sini
    }
}
