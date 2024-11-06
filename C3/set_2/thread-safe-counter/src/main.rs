use std::{
    sync::{Arc, Mutex},
    thread,
};

use counter::Counter;

mod counter;

fn main() {
    let counter = Arc::new(Mutex::new(Counter::new()));

    let mut handles = Vec::new();

    for _ in 0..1000 {
        let thread_counter = Arc::clone(&counter);

        let handle = thread::spawn(move || {
            Counter::increment(thread_counter);
        });

        handles.push(handle);
    }

    handles
        .into_iter()
        .for_each(|handle| handle.join().unwrap());

    println!("count: {}", counter.lock().unwrap().0);

    assert_eq!(counter.lock().unwrap().0, 1000);
}
