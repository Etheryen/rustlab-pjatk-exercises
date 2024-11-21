trait Summable<T> {
    fn sum(&self) -> T;
}

impl Summable<i32> for Vec<i32> {
    fn sum(&self) -> i32 {
        self.iter().sum()
    }
}

impl Summable<f64> for Vec<f64> {
    fn sum(&self) -> f64 {
        self.iter().sum()
    }
}

fn main() {
    let ints = vec![23, 5, -12, 33];
    println!("{}", ints.sum());

    let floats = vec![0.34, 3.023, -1.234, -0.323];
    println!("{}", floats.sum());
}
