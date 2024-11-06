use std::sync::{Arc, Mutex};

// lepiej by było użyć AtomicU32 :((
pub struct Counter(pub u32);

impl Counter {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn increment(counter: Arc<Mutex<Self>>) {
        counter.lock().unwrap().0 += 1;
    }
}
