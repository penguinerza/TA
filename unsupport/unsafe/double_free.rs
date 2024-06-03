use std::alloc::{alloc, dealloc, Layout};

fn double_free() {
    unsafe {
        // Alokasi memori manual
        let layout = Layout::new::<i32>();
        let ptr = alloc(layout);

        // Memberikan nilai ke memori yang dialokasikan
        *(ptr as *mut i32) = 4;

        // Dealokasi pertama
        dealloc(ptr, layout);

        // Dealokasi kedua yang disengaja mengakibatkan double free
        dealloc(ptr, layout);
    }
}

fn main() {
    double_free();
}