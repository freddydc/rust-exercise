// iterators2.rs

pub fn run_iterators2() {
    let cart = vec!["avocado", "banana"];
    let message = vec!["good", " ", "morning"];

    let cart_result = capitalize_words_vector(&cart);
    let message_result = capitalize_words_string(&message);

    println!("{:#?}", cart_result);
    println!("{}", message_result);
}

pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    match c.next() {
        None => String::new(),
        Some(first) => {
            let f = first.to_uppercase().collect::<String>();
            let value = c.as_str();
            format!("{}{}", f, value)
        }
    }
}

pub fn capitalize_words_vector(w: &[&str]) -> Vec<String> {
    w.iter().map(|value| capitalize_first(value)).collect()
}

pub fn capitalize_words_string(w: &[&str]) -> String {
    w.iter().map(|value| capitalize_first(value)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_input() {
        assert_eq!(capitalize_first("avocado"), "Avocado");
    }

    #[test]
    fn empty_input() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn iterate_string_vector() {
        let words = vec!["avocado", "banana"];
        assert_eq!(capitalize_words_vector(&words), ["Avocado", "Banana"]);
    }

    #[test]
    fn iterate_into_string() {
        let words = vec!["good", " ", "morning"];
        assert_eq!(capitalize_words_string(&words), "Good Morning");
    }
}
