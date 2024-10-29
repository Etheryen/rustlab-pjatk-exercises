use std::{
    collections::VecDeque,
    future::Future,
    pin::{pin, Pin},
    sync::{Arc, Mutex},
    task::{Context, Poll, Waker},
    thread,
};

struct Task {
    future: Pin<Box<dyn Future<Output = ()>>>,
    waker: Option<Waker>,
}

pub struct Executor {
    tasks: VecDeque<Task>,
}

impl Executor {
    pub fn new() -> Self {
        Self {
            tasks: VecDeque::new(),
        }
    }

    pub fn spawn<F>(&mut self, future: F)
    where
        F: Future<Output = ()> + 'static,
    {
        self.tasks.push_back(Task {
            future: Box::pin(future),
            waker: None,
        });
    }

    pub fn run(&mut self) {
        todo!()
    }
}
