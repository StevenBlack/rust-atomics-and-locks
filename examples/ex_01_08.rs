// Mutex stands for mutual exclusion. It is a synchronization primitive that
// can be used to protect shared data from being simultaneously accessed by
// multiple threads. Mutex is a smart pointer, and the data it protects is
// wrapped in a lock. The lock is acquired by calling the lock method on the
// mutex. This call blocks the current thread until it is able to acquire the
// lock. The lock is released when the lock goes out of scope. The type of the
// data that the mutex protects is specified in the angle brackets <T> after
// the mutex name. The mutex in this example is protecting an i32, so the type
// of the mutex is Mutex<i32>. The lock method returns a smart pointer called
// MutexGuard. The MutexGuard smart pointer implements Deref to point at our
// inner data, so we can treat the MutexGuard smart pointer like a regular
// reference to the inner data. The MutexGuard smart pointer also has a Drop
// implementation that releases the lock automatically when a MutexGuard goes
// out of scope.

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
