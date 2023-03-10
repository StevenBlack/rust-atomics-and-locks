use std::thread;

fn main() {
    let numbers = Vec::from_iter(0..=1000);
    let t1 = thread::spawn(f);
    let t2 = thread::spawn(f);

    println!("Hello from the main thread.");

    //  it’s far more common to pass it a closure. This allows us to capture
    //  values to move into the new thread:
    let t = thread::spawn(move || {
        let len = numbers.len();
        let sum = numbers.iter().sum::<usize>();
        sum / len
    });

    t1.join().unwrap();
    t2.join().unwrap();

    let average = t.join().unwrap();
    println!("average: {average}");

}

fn f() {
    println!("Hello from another thread!");

    let id = thread::current().id();
    println!("This is my thread id: {id:?}");
}




