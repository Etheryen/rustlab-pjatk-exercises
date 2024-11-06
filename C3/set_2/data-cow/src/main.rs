use std::borrow::Cow;

fn process_data(data: &str) -> Cow<str> {
    let mut cow = Cow::from(data);

    if data.contains('!') {
        let cow_mut = cow.to_mut();
        cow_mut.make_ascii_uppercase();
    }

    cow
}

fn main() {
    let data1 = "hello world!";
    let data2 = "hi";

    let processed1 = process_data(data1);
    let processed2 = process_data(data2);

    println!("{}", processed1);
    println!("{}", processed2);

    if let Cow::Borrowed(..) = processed1 {
        unreachable!();
    }

    if let Cow::Owned(..) = processed2 {
        unreachable!();
    }

    assert_eq!(processed1, "HELLO WORLD!");
    assert_eq!(processed2, "hi");
}
