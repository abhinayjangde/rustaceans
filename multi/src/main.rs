use std::thread;
// use std::time::Duration;

fn main() {
    let v = vec![1, 2, 3];

    let handler = thread::spawn(move || {
        println!("vector is {v:?}");
    });

    handler.join().unwrap();
}
