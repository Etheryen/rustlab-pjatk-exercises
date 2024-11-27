mod fetch;

fn main() {
    println!("Fetching...");
    println!("Response: {:?}", fetch::fetch_data(123));
}
