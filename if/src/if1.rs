// if1.rs

pub fn run_if1() {
    let answer = bigger(20, 500);
    println!("Bigger number is {}", answer);
}

pub fn bigger(a: i32, b: i32) -> i32 {
    // Complete this function to return the bigger number!
    // Do not use:
    // - another function call
    // - additional variables
    if b < a {
        a
    } else {
        b
    }
}

// Don't mind this for now :)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ten_is_bigger_than_eight() {
        assert_eq!(10, bigger(10, 8));
    }

    #[test]
    fn forty_is_bigger_than_thirty() {
        assert_eq!(40, bigger(30, 40));
    }
}
