use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use std::time::Duration;

/// Multiple producers, single consumer
pub fn exemple3() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let val = vec![
            String::from("Hi"),
            String::from("from"),
            String::from("other"),
            String::from("thread"),
        ];

        for v in val {
            tx1.send(v).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let val = vec![
            String::from("More"),
            String::from("messages"),
            String::from("from"),
            String::from("another thread"),
        ];

        for v in val {
            tx.send(v).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for message in rx {
        println!("{message}");
    }
}

/// Passing multiple messages between threads with channel
pub fn exemple2() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = vec![
            String::from("Hi"),
            String::from("from"),
            String::from("other"),
            String::from("thread"),
        ];

        for v in val {
            tx.send(v).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for message in rx {
        println!("{message}");
    }
}

/// Passing message between threads with channel
pub fn exemple1() {
    let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("Hi");
        tx.send(val).unwrap();
    });

    let message = rx.recv().unwrap();
    println!("{message}");
}
