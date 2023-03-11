use std::{sync::Mutex, thread};

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
                let id = thread::current().id();
                println!("This is my thread id: {id:?}");
                let mut v = seq.lock().unwrap();
                v.push(id);
            });
        }
    });
    assert_eq!(n.into_inner().unwrap(), 1000);
    println!("seq: {seq:?}");
}
