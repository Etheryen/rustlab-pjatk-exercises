use std::{
    sync::{Arc, Mutex},
    thread,
};

fn main() {
    // Znowu zadanie z counter Arc<Mutex<i32>> zamiast AtomicI32 :(
    let counter = Arc::new(Mutex::new(0));

    let mut handles = Vec::new();

    for _ in 0..5 {
        let thread_counter = Arc::clone(&counter);

        let h = thread::spawn(move || *thread_counter.lock().unwrap() += 1);

        handles.push(h);
    }

    for h in handles {
        h.join().unwrap();
    }

    println!("counter: {}", counter.lock().unwrap());
}
