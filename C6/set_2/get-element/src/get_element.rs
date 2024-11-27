pub fn get_element(vec: Vec<i32>, index: usize) -> i32 {
    match vec.get(index) {
        Some(&element) => element,
        None => panic!("index out of range"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_index() {
        assert_eq!(get_element(vec![2, 3, 9], 2), 9);
    }

    #[test]
    #[should_panic(expected = "index out of range")]
    fn invalid_index() {
        get_element(vec![2, 3, 9], 3);
    }
}
