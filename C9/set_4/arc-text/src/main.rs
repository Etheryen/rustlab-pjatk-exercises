use std::{sync::Arc, thread};

fn main() {
    let text = Arc::new("Hello, Arc!".to_string());

    let mut handles = Vec::new();

    for i in 0..3 {
        let thread_text = Arc::clone(&text);

        let h = thread::spawn(move || {
            println!("thread {} - {}", i, thread_text);
        });

        handles.push(h);
    }

    for h in handles {
        h.join().unwrap();
    }
}
