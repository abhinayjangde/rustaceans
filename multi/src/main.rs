// use std::thread;
// use std::time::Duration;

fn apply<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 {
    f(x)
}
fn main() {
    


    println!("result is {} ", apply(|n| n * n, 6));

    // let handle = thread::spawn(|| {  
    //     for i in 1..10 {
    //         println!("hi {} from spawned thread!", i);
    //         thread::sleep(Duration::from_millis(1));
    //     }
    // });

    // handle.join().unwrap();
    
    // for i in 1..5 {
    //     println!("hi {} from main thread", i);
    //     thread::sleep(Duration::from_millis(1));
    // }
}
