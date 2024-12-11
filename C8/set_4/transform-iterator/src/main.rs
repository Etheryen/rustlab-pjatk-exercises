use std::fmt::Display;

fn transform_iterator<I, D>(iter: I) -> impl Iterator<Item = String>
where
    I: Iterator<Item = D>,
    D: Display,
{
    iter.enumerate().map(|(i, el)| format!("{}: {}", i, el))
}

fn main() {
    let iter = -5..=5;

    println!("before:");
    iter.clone().for_each(|el| println!("{}", el));

    println!("\nafter:");
    transform_iterator(iter).for_each(|el| println!("{}", el));
}
