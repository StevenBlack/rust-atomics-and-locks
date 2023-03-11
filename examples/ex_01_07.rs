use std::cell::Cell;

// Interior mutability is a design pattern in Rust for allowing you to mutate
// data even when there are immutable references to that data; normally
// this action is disallowed by the borrowing rules. To mutate data, the pattern
// uses unsafe code inside a data structure to bend Rust’s usual rules that
// govern mutation and borrowing.
fn main() {
    let c1 = Cell::new(100);
    let c2 = Cell::new(50);
    f(&c1, &c2);
    println!("Done!");

}

fn f(a: &Cell<i32>, b: &Cell<i32>) {
    let before = a.get();
    b.set(b.get() + 1);
    let after = a.get();
    if before != after {
        println!("It happened?"); // might happen
    }
}
