struct Matrix<T> {
    data: Vec<Vec<T>>,
    curr_col: usize,
}

impl<T> Matrix<T> {
    fn from_vec(data: Vec<Vec<T>>) -> Self {
        Self { data, curr_col: 0 }
    }
}

impl<T> Iterator for Matrix<T>
where
    // Nie udało mi się na samych referencjach, bez Clone
    T: Clone,
{
    type Item = std::vec::IntoIter<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let iter = self
            .data
            .clone()
            .into_iter()
            .map(|row| {
                if self.curr_col < row.len() {
                    Some(row[self.curr_col].clone())
                } else {
                    None
                }
            })
            .collect::<Option<Vec<_>>>()?
            .into_iter();

        self.curr_col += 1;
        Some(iter)
    }
}

fn main() {
    println!("before:");

    for row in [[1, 2, 3], [4, 5, 6], [7, 8, 9]] {
        for val in row {
            print!("{} ", val);
        }
        println!();
    }

    println!("\nafter:");

    let matrix = Matrix::from_vec(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]);

    for col in matrix {
        for val in col {
            print!("{} ", val);
        }
        println!();
    }
}
