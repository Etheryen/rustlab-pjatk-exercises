mod async_fetch;

#[tokio::main]
async fn main() {
    println!("{:?}", async_fetch::async_fetch("dwad").await);
}
