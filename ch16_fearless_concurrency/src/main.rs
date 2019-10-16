use std::sync::mpsc;
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

    // Demonstrate move semantics for closure
    // let v = vec![1, 2, 3];

    // let handle = thread::spawn(move || {
    //     println!("Here's a vector = {:?}", v);
    // });

    // handle.join().unwrap();

    let (tx, rx) = mpsc::channel(); // mpsc - Multi producer, Single Consumer
    thread::spawn(move || {
        let vals = vec![
            String::from("hello"),
            String::from("from"),
            String::from("the"),
            String::from("other"),
            String::from("side"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}
