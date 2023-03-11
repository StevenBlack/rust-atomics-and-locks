
// Interior mutability is a design pattern in Rust for allowing you to mutate
// data even when there are immutable references to that data; normally
// this action is disallowed by the borrowing rules. To mutate data, the pattern
// uses unsafe code inside a data structure to bend Rust’s usual rules that
// govern mutation and borrowing.

use std::cell::{Cell, RefCell};

fn main() {
    let c1 = Cell::new(100);
    let c2 = Cell::new(50);
    let rc = RefCell::new(vec![1, 2, 3]);
    f(&c1, &c2);
    f2(&rc);
    println!("Done!");

}

// A std::cell::Cell<T> simply wraps a T, but allows mutations through a shared
// reference. To avoid undefined behavior, it only allows you to copy the value
// out (if T is Copy), or replace it with another value as a whole. In addition,
// it can only be used within a single thread.
fn f(a: &Cell<i32>, b: &Cell<i32>) {
    let before = a.get();
    b.set(b.get() + 1);
    let after = a.get();
    if before != after {
        println!("It happened?"); // might happen
    }
}

// The RefCell<T> type is similar to Cell<T>, but instead of only providing a
// copy of the value, it provides a mutable reference. This means that the
// borrowing rules are enforced at runtime instead of compile time. This is
// useful when you know that the borrowing rules will be followed at runtime,
// but can’t express that to the compiler.
fn f2(v: &RefCell<Vec<i32>>) {
    v.borrow_mut().push(10); //
    println!("{:?}", v);
}

