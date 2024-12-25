use std::{
    sync::{Arc, Mutex},
    thread,
};

struct SharedCounter {
    count: u32,
}

impl SharedCounter {
    fn new() -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(Self { count: 0 }))
    }

    fn increment(this: Arc<Mutex<Self>>) {
        this.lock().unwrap().count += 1;
    }

    fn get_count(this: Arc<Mutex<Self>>) -> u32 {
        this.lock().unwrap().count
    }
}

fn main() {
    let counter = SharedCounter::new();

    let mut handles = Vec::new();

    for _ in 0..1000 {
        let thread_counter = Arc::clone(&counter);

        let h = thread::spawn(move || SharedCounter::increment(thread_counter));

        handles.push(h);
    }

    for h in handles {
        h.join().unwrap();
    }

    println!("count: {}", SharedCounter::get_count(counter));
}
