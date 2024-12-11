fn label_strings(strings: Vec<&str>) -> Vec<String> {
    strings
        .into_iter()
        .enumerate()
        .map(|(index, string)| format!("{index}: {string}"))
        .collect()
}

fn main() {
    println!("before: {:?}", vec!["ala", "ma", "kota"]);
    println!("after: {:?}", label_strings(vec!["ala", "ma", "kota"]));
}
