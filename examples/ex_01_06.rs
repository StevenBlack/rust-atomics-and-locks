// Reference counting with shadow naming clones
// The clone of the Arc lives in a different scope. We can use the same
// name in each thread.

// Arc stands for “atomically reference counted.”

use std::thread;
use std::sync::Arc;

fn main() {
    let a = Arc::new([1, 2, 3]);

    let t1 = thread::spawn({
        let a = a.clone();
        move || {
            dbg!(a);
        }
    });
    let t2 = thread::spawn({
        let a = a.clone();
        move || {
            dbg!(a);
        }
    });
    t1.join().unwrap();
    t2.join().unwrap();

    // this will echo last
    dbg!(a);
}
