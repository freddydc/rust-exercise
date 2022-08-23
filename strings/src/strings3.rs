// strings3.rs

pub fn run_strings3() {
    let trim_me_answer = trim_me(" Good Day!    ");
    let compose_me_answer = compose_me("Good morning");
    let replace_me_answer = replace_me("I think cars are cool");

    println!("{:?}", trim_me_answer);
    println!("{}", compose_me_answer);
    println!("{}", replace_me_answer);
}

fn trim_me(input: &str) -> String {
    input.trim().to_string()
}

fn compose_me(input: &str) -> String {
    format!("{} friend!", input)
}

fn replace_me(input: &str) -> String {
    input.replace("cars", "bikes")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Good Day!  "), "Good Day!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Good morning"), "Good morning friend!");
        assert_eq!(compose_me("Goodbye"), "Goodbye friend!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(
            replace_me("I think cars are cool"),
            "I think bikes are cool"
        );
        assert_eq!(
            replace_me("I love to look at cars"),
            "I love to look at bikes"
        );
    }
}
