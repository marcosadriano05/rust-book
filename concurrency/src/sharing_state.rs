use std::{
    sync::{Arc, Mutex},
    thread,
};

pub fn exemple2() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for h in handles {
        h.join().unwrap();
    }

    println!("{}", *counter.lock().unwrap());
}

pub fn exemple1() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 10;
    }

    println!("{m:?}");
}
