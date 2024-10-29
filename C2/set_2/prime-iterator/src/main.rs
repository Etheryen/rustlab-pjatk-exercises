struct PrimeNumbers {
    n: u32,
    curr: u32,
}

impl PrimeNumbers {
    fn new(n: u32) -> Self {
        Self { n, curr: 1 }
    }

    fn is_prime(number: u32) -> bool {
        !(2..=(number as f64).sqrt() as u32).any(|curr| number % curr == 0)
    }
}

impl Iterator for PrimeNumbers {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.curr += 1;

        while !Self::is_prime(self.curr) {
            self.curr += 1;
        }

        if self.curr > self.n {
            return None;
        }
        Some(self.curr)
    }
}

fn main() {
    let mut primes = PrimeNumbers::new(20);
    while let Some(prime) = primes.next() {
        println!("{}", prime); // Wynik: 2, 3, 5, 7, 11, 13, 17, 19
    }
}
