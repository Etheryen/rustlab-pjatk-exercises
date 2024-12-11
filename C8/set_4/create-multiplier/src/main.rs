fn create_multiplier(num: i32) -> impl Fn(i32) -> i32 {
    move |arg| arg * num
}

fn main() {
    let multiply_by_7 = create_multiplier(7);

    println!("{:?}", (1..10).map(multiply_by_7).collect::<Vec<_>>())
}
