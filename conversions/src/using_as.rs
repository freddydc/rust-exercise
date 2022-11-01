// using_as.rs

// Type casting in Rust is done via the usage of the `as` operator.
// Please note that the `as` operator is not only used when type casting.
// It also helps with renaming imports.

pub fn run_using_as() {
    let scores = [6.0, 4.0, 8.0, 2.0];
    println!("The average is: ({:?})", average(&scores));
}

fn average(scores: &[f64]) -> f64 {
    let total = scores.iter().sum::<f64>();
    total / scores.len() as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_proper_type_and_value() {
        assert_eq!(average(&[6.0, 4.0, 8.0, 2.0]), 5.0);
    }
}
