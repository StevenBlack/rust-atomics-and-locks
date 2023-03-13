# 🦀 Rust Atomics and Locks 📖

Working through [Rust Atomics and Locks](https://marabos.nl/atomics/) by Mara Bos.

![2023-03-10_14-39-28](https://user-images.githubusercontent.com/80144/224441677-987e5db3-bf28-4395-a230-18c9df3c8925.jpg)

At the moment this is just a bunch of disenbodied code snippets that live
in the `examples/` folder, which is implicitly supported by Cargo.

Full credit to
[Add examples to your Rust libraries](http://xion.io/post/code/rust-examples.html)
by [Karol Kuczmarski](https://github.com/Xion), and  to the
[Rust Cargo Team](https://github.com/rust-lang/cargo) for the structure of this
repository.

## Sample calls

```bash
$ cargo run --example ex_01_01
```

```bash
$ cargo run --example ex_01_02
```

etc...


## The examples

* [`ex_01_01`](./examples/ex_01_01.rs): Spawn two threads. The main thread waits for the other threads to finish before exiting.
* [`ex_01_02`](./examples/ex_01_02.rs): Spawn threads passing-in a closure which allows capture of values to move into the new thread.
* [`ex_01_03`](./examples/ex_01_03.rs): *Ibid*.
* [`ex_01_04`](./examples/ex_01_04.rs): A scoped thread.
* [`ex_01_05`](./examples/ex_01_05.rs): Using `Arc`.
* [`ex_01_06`](./examples/ex_01_06.rs): Reference counting with shadow naming clones.
* [`ex_01_07`](./examples/ex_01_07.rs): Interior mutability with `Cell`.
* [`ex_01_08`](./examples/ex_01_08.rs): using `Mutex`.
* [`ex_01_09`](./examples/ex_01_09.rs): using `Mutex` with a sleep while locked.
* [`ex_01_10`](./examples/ex_01_10.rs): using `Mutex` with a sleep while not locked.


