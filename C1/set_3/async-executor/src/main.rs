use executor::Executor;

mod executor;

async fn example_task() {
    println!("Hello from a task!");
}

fn main() {
    let mut executor = Executor::new();
    executor.spawn(example_task());
    executor.run();
}
