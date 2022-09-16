// errors4.rs

pub fn run_errors4() {
    let integer = PositiveNonzeroInteger::new(100);

    match integer {
        Ok(PositiveNonzeroInteger(x)) => println!("{}", x),
        Err(e) => println!("{:?}", e),
    }
}

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        match value {
            n if n < 0 => Err(CreationError::Negative),
            n if n == 0 => Err(CreationError::Zero),
            n => Ok(PositiveNonzeroInteger(n as u64)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn positive_nonzero_integer_creation() {
        assert!(PositiveNonzeroInteger::new(10).is_ok());
        assert_eq!(
            Err(CreationError::Negative),
            PositiveNonzeroInteger::new(-10)
        );
        assert_eq!(Err(CreationError::Zero), PositiveNonzeroInteger::new(0));
    }
}
