use std::thread;
use std::sync::Arc;

// Reference counting is a way to share data between threads.
fn main() {

    let a = Arc::new([1, 2, 3]);
    let b = a.clone();

    let t1 = thread::spawn(move || dbg!(a));
    let t2 = thread::spawn(move || dbg!(b));

    t1.join().unwrap();
    t2.join().unwrap();
}
