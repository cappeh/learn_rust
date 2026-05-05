use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    // move will force the closure to take ownership of the values its using
    let handle = thread::spawn(move || {
        println!("Here is a vector: {v:?}");
    });

    handle.join().unwrap();
}
