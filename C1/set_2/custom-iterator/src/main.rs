struct EvenNumbers {
    n: u32,
    curr: u32,
}

impl EvenNumbers {
    fn new(n: u32) -> Self {
        Self { n, curr: 0 }
    }
}

impl Iterator for EvenNumbers {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr > self.n {
            return None;
        }

        let result = self.curr;
        self.curr += 2;

        Some(result)
    }
}

fn main() {
    let mut even_numbers1 = EvenNumbers::new(10);
    while let Some(num) = even_numbers1.next() {
        println!("{}", num);
    }

    let even_numbers2 = EvenNumbers::new(10);

    // Metoda sum automatycznie zaimplementowana z traita Iterator
    println!("Suma: {}", even_numbers2.sum::<u32>());
}
