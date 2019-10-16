use std::thread;
use std::time::Duration;

fn main() {
    println!("Chapter 16 - Fearless Concurrency");
    // let handle = thread::spawn(|| {
    //     for i in 1..10 {
    //         println!("Hi number {} from spawned thread", i);
    //         thread::sleep(Duration::from_millis(1));
    //     }
    // });

    // handle.join().unwrap(); // Putting the join call here blocks the main thread until
    // the spawned thread finishes

    // for i in 1..5 {
    //     println!("Hi number {} from main thread", i);
    //     thread::sleep(Duration::from_millis(1));
    // }

    // handle.join().unwrap();

    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector = {:?}", v);
    });

    handle.join().unwrap();
}
