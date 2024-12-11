fn square_odd_numbers(nums: Vec<i32>) -> Vec<i32> {
    nums.into_iter()
        .filter(|num| num % 2 != 0)
        .map(|num| num * num)
        .collect()
}

fn main() {
    println!("before: {:?}", (-5..10).collect::<Vec<_>>());
    println!("after: {:?}", square_odd_numbers((-5..15).collect()));
}
