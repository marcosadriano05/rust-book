use std::thread::{self, JoinHandle};
use std::time::Duration;

/// Closure take ownership of the value that the spawned thread use from the main thread
pub fn exemple3() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Vector {v:?}");
    });

    handle.join().unwrap();
}

/// Waiting thread to finish before the main thread finish
pub fn exemple2() {
    let handle: JoinHandle<()> = thread::spawn(|| {
        for i in 1..10 {
            println!("Hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("Hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}

/// The main thread finish before other threads finish
pub fn exemple1() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("Hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("Hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }
}
