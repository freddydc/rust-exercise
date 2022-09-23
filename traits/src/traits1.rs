// traits1.rs

// The trait AppendHello has only one function,
// which appends "Hello" to any object
// implementing this trait.

pub fn run_traits1() {
    let s = String::from("Mary!");
    let s = s.append_hello();
    println!("{:?}", s);
}

trait AppendHello {
    fn append_hello(&self) -> Self;
}

impl AppendHello for String {
    fn append_hello(&self) -> Self {
        if !self.is_empty() {
            format!("Hello {}", self)
        } else {
            "Hello".to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_hello_mary() {
        assert_eq!(
            String::from("Mary!").append_hello(),
            String::from("Hello Mary!")
        );
    }

    #[test]
    fn is_hello_hello() {
        assert_eq!(
            String::from("").append_hello().append_hello(),
            String::from("Hello Hello")
        );
    }
}
