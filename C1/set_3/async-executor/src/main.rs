use std::{
    future::poll_fn,
    task::Poll,
    time::{Duration, Instant},
};

use executor::Executor;

mod executor;

async fn async_sleep(duration: Duration) {
    let start = Instant::now();
    poll_fn(|cx| {
        if start.elapsed() >= duration {
            return Poll::Ready(());
        }

        cx.waker().wake_by_ref();
        Poll::Pending
    })
    .await;
}

async fn example_task() {
    println!("Hello from a task!");
}

async fn example_wait() {
    async_sleep(Duration::from_secs(1)).await;
    println!("Hello from a loooong task...");
}

fn main() {
    let executor = Executor::new();
    executor.spawn(example_task());
    executor.spawn(example_wait());
    executor.spawn(example_task());
    executor.spawn(example_wait());
    executor.spawn(async {
        println!("Hello from a closure!");
    });
    executor.run();

    // executor działa wiecznie, co możnaby zmienić gdyby tworzyć sender
    // i executor osobno oraz wywoływać drop(sender) przed executor.run(),
    // w ten sposób kanał task_sender byłby, zamknięty, co by sprawiło,
    // że executor.run() wychodzi z pętli, ale w tym przykładzie potrzebna
    // była jedna zmienna executor
}
