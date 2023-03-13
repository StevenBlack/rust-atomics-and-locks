// Mutex with a sleep while locked

use std::{sync::Mutex, thread};
use std::time::Duration; // New!

fn main() {
    let seq: Mutex<Vec<std::thread::ThreadId>> = Mutex::new(vec![]);
    let n = Mutex::new(0);
    thread::scope(|s| {
        for _ in 0..10 {
            s.spawn(|| {
                let mut guard = n.lock().unwrap();
                for _ in 0..100 {
                    *guard += 1;
                }
                thread::sleep(Duration::from_secs(1)); // New!

                let id = thread::current().id();
                println!("This is from thread id: {id:?}");
                let mut v = seq.lock().unwrap();
                v.push(id);
            });
        }
    });
    let tot = n.into_inner().unwrap();
    assert_eq!(tot, 1000);
    println!("total: {tot}");
    println!("seq: {seq:?}");
}
