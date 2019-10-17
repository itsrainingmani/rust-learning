// use std::rc::Rc;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    println!("Chapter 16 - Fearless Concurrency");
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Hi number {} from spawned thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap(); // Putting the join call here blocks the main thread until
                            // the spawned thread finishes

    // for i in 1..5 {
    //     println!("Hi number {} from main thread", i);
    //     thread::sleep(Duration::from_millis(1));
    // }

    // handle.join().unwrap();

    // Demonstrate move semantics for closure
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector = {:?}", v);
    });

    handle.join().unwrap();

    let (tx, rx) = mpsc::channel(); // mpsc - Multi producer, Single Consumer

    let tx1 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        let vals = vec![
            String::from("hello"),
            String::from("from"),
            String::from("the"),
            String::from("other"),
            String::from("side"),
        ];
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("I'm"),
            String::from("a monkey"),
            String::from("that"),
            String::from("likes"),
            String::from("bananas"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }

    // let m = Mutex::new(5); // Mutex<T> is a smart pointer

    // {
    //     let mut num = m.lock().unwrap(); // acquire lock and block current thread
    //     *num = 6;
    // } // <- lock is released here automatically since the inner
    //   // MutexGuard has gone out of scope

    // println!("m = {:?}", m);

    let counter = Arc::new(Mutex::new(0)); // Atomically reference counted
                                           // Safe to be shared across threads
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());

    // let data = Arc::new(Mutex::new(0));
    // let d1 = data.lock();
    // let d2 = data.try_lock();
}
