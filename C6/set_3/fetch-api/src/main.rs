mod fetch_api;

fn main() {
    println!(
        "{:?}",
        fetch_api::api_request("https://jsonplaceholder.typicode.com/posts/1")
    );
}
