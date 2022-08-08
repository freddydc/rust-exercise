// if2.rs

// Step 1: Make me compile!
// Step 2: Get the banana_for_fruit and default_to_dinner tests passing!

pub fn run_if2() {
    let answer = cookie_if_bag("bag");
    println!("You won a {}", answer);
}

pub fn cookie_if_bag(choose: &str) -> &str {
    if choose == "bag" {
        "cookie"
    } else if choose == "fruit" {
        "banana"
    } else {
        "dinner"
    }
}

// No test changes needed!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cookie_for_bag() {
        assert_eq!(cookie_if_bag("bag"), "cookie");
    }

    #[test]
    fn banana_for_fruit() {
        assert_eq!(cookie_if_bag("fruit"), "banana");
    }

    #[test]
    fn default_to_dinner() {
        assert_eq!(cookie_if_bag("literally anything"), "dinner");
    }
}
