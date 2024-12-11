use std::collections::HashSet;

fn unique_sorted_squares(nums: Vec<i32>) -> Vec<i32> {
    let mut result = nums
        .into_iter()
        .collect::<HashSet<_>>()
        .into_iter()
        .map(|num| num * num)
        .collect::<Vec<_>>();

    result.sort();

    result
}

fn main() {
    let nums = vec![5, -3, 0, 1, 2, -3, 2, 5, 2, -3, -3];

    println!("before: {:?}", nums);
    println!("after: {:?}", unique_sorted_squares(nums));
}
