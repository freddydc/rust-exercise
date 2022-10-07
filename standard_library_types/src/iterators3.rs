// iterators3.rs

pub fn run_iterators3() {
    let first_result = get_division_list();
    let second_result = get_division_result_list();

    println!("{:#?}", first_result);
    println!("{:#?}", second_result);
}

#[derive(Debug, PartialEq, Eq)]
pub enum DivisionError {
    NotDivisible(NotDivisibleError),
    DivideByZero,
}

#[derive(Debug, PartialEq, Eq)]
pub struct NotDivisibleError {
    dividend: i32,
    divisor: i32,
}

pub fn divide(x: i32, y: i32) -> Result<i32, DivisionError> {
    if y == 0 {
        Err(DivisionError::DivideByZero)
    } else if (x % y) != 0 {
        Err(DivisionError::NotDivisible(NotDivisibleError {
            dividend: x,
            divisor: y,
        }))
    } else {
        Ok(x / y)
    }
}

fn get_division_list() -> Result<Vec<i32>, DivisionError> {
    let numbers = vec![27, 297, 38502, 81];
    numbers.into_iter().map(|n| divide(n, 27)).collect()
}

fn get_division_result_list() -> Vec<Result<i32, DivisionError>> {
    let numbers = vec![27, 297, 38502, 81];
    numbers.into_iter().map(|n| divide(n, 27)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_division() {
        assert_eq!(divide(81, 9), Ok(9));
    }

    #[test]
    fn not_divisible() {
        assert_eq!(
            divide(81, 6),
            Err(DivisionError::NotDivisible(NotDivisibleError {
                dividend: 81,
                divisor: 6
            }))
        );
    }

    #[test]
    fn divide_by_zero() {
        assert_eq!(divide(81, 0), Err(DivisionError::DivideByZero));
    }

    #[test]
    fn divide_zero_by_something() {
        assert_eq!(divide(0, 81), Ok(0));
    }

    #[test]
    fn division_list() {
        assert_eq!(format!("{:?}", get_division_list()), "Ok([1, 11, 1426, 3])");
    }

    #[test]
    fn division_result_list() {
        assert_eq!(
            format!("{:?}", get_division_result_list()),
            "[Ok(1), Ok(11), Ok(1426), Ok(3)]"
        );
    }
}
