// iterators4.rs

pub fn run_iterators4() {
    let n = 5;
    let result = factorial(n);
    println!("Factorial of {}! is {}", n, result);
}

// Using `fold()`
// (1..=num).fold(1, |acc, n| acc * n)
pub fn factorial(num: u64) -> u64 {
    (1..=num).product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(1, factorial(0));
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }

    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}
