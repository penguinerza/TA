fn main() {
    let x = Box::new(42);
    
    let raw = Box::into_raw(x);

    let y = unsafe { Box::from_raw(raw) };
    let z = unsafe { Box::from_raw(raw) };

    drop(y);
    drop(z);
}