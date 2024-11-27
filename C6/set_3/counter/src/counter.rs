use std::sync::{Arc, Mutex};

pub struct Counter {
    pub count: u32,
}

impl Counter {
    pub fn new() -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(Self { count: 0 }))
    }

    pub fn increment(self_arcmutex: &Arc<Mutex<Self>>) {
        self_arcmutex.lock().unwrap().count += 1;
    }

    pub fn get_count(self_arcmutex: &Arc<Mutex<Self>>) -> u32 {
        self_arcmutex.lock().unwrap().count
    }
}

#[cfg(test)]
mod tests {
    use std::thread;

    use super::*;

    #[test]
    fn it_works() {
        const THREAD_COUNT: u32 = 999;

        let counter = Counter::new();
        let mut join_handles = Vec::new();

        for _ in 0..THREAD_COUNT {
            let thread_counter = Arc::clone(&counter);

            let handle = thread::spawn(move || {
                Counter::increment(&thread_counter);
            });
            join_handles.push(handle);
        }

        join_handles.into_iter().for_each(|h| h.join().unwrap());

        assert_eq!(Counter::get_count(&counter), THREAD_COUNT);
    }
}
