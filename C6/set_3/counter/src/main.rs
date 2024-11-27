use std::{sync::Arc, thread};

use counter::Counter;

mod counter;

fn main() {
    let counter = Counter::new();
    let mut join_handles = Vec::new();

    for _ in 0..123 {
        let thread_counter = Arc::clone(&counter);

        let handle = thread::spawn(move || {
            Counter::increment(&thread_counter);
        });
        join_handles.push(handle);
    }

    join_handles.into_iter().for_each(|h| h.join().unwrap());

    println!("{}", Counter::get_count(&counter));
}
