struct Container<T>
where
    T: PartialOrd + Copy,
{
    first: T,
    second: T,
}

impl<T> Container<T>
where
    T: PartialOrd + Copy,
{
    fn max(&self) -> T {
        if self.first >= self.second {
            self.first
        } else {
            self.second
        }
    }
}

fn main() {
    let nums = Container {
        first: 9,
        second: 12,
    };

    println!("{}", nums.max());
}
