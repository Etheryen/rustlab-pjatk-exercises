const SQRT_ERROR: &str = "can't take a sqrt of a negative number";

pub fn sqrt(number: f64) -> Result<f64, String> {
    if number < 0_f64 {
        Err(SQRT_ERROR.into())
    } else {
        Ok(number.sqrt())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn positives() {
        assert_eq!(sqrt(81_f64), Ok(9_f64));
    }

    #[test]
    fn negatives() {
        assert_eq!(sqrt(-23.3), Err(SQRT_ERROR.into()));
    }

    #[test]
    fn zero() {
        assert_eq!(sqrt(0_f64), Ok(0_f64));
    }
}
