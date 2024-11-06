use std::{
    cell::UnsafeCell,
    future::Future,
    pin::Pin,
    sync::{
        mpsc::{sync_channel, Receiver, SyncSender},
        Arc,
    },
    task::Context,
};

use futures::task::{waker_ref, ArcWake};

struct Task {
    future: UnsafeCell<Option<Pin<Box<dyn Future<Output = ()> + Send>>>>,
    task_sender: SyncSender<Arc<Task>>,
}

unsafe impl Sync for Task {}

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        let cloned = arc_self.clone();
        arc_self
            .task_sender
            .try_send(cloned)
            .expect("too many tasks queued");
    }
}

pub struct Executor {
    ready_queue: Receiver<Arc<Task>>,
    task_sender: SyncSender<Arc<Task>>,
}

impl Executor {
    pub fn new() -> Self {
        const MAX_QUEUED_TASKS: usize = 10_000;
        let (task_sender, ready_queue) = sync_channel(MAX_QUEUED_TASKS);
        Self {
            ready_queue,
            task_sender,
        }
    }

    pub fn spawn<F>(&self, future: F)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        let pinned = Box::pin(future);
        let task = Arc::new(Task {
            future: UnsafeCell::new(Some(pinned)),
            task_sender: self.task_sender.clone(),
        });
        self.task_sender
            .try_send(task)
            .expect("too many tasks queued");
    }

    pub fn run(&self) {
        while let Ok(task) = self.ready_queue.recv() {
            // SAFETY: only one thread accesses the future at a time

            let future_slot = task.future.get();

            let future_option = unsafe { (*future_slot).take() };

            if let Some(mut future) = future_option {
                let waker = waker_ref(&task);
                let context = &mut Context::from_waker(&waker);

                if future.as_mut().poll(context).is_pending() {
                    unsafe {
                        *future_slot = Some(future);
                    }
                }
            }
        }
    }
}
