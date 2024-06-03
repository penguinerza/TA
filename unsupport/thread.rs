use std::thread;

fn main() {
    let handles: Vec<_> = (0..5)
        .map(|i| {
            thread::spawn(move || {
                println!("Thread {} berjalan", i);
            })
        })
        .collect();

    for handle in handles { // prusti belum support iterator
        handle.join().unwrap();
    }
}