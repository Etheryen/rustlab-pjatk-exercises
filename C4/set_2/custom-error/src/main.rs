use std::{error::Error, fmt::Display};

#[derive(Debug)]
enum MathError {
    DivideByZero,
    NegativeSquareRoot,
}

impl Display for MathError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DivideByZero => write!(f, "can't divide by zero"),
            Self::NegativeSquareRoot => write!(f, "can't take a negative square root"),
        }
    }
}

impl Error for MathError {}

fn main() {
    println!("{}", MathError::DivideByZero);
    println!("{}", MathError::NegativeSquareRoot);
}
