// generics2.rs

pub fn run_generics2() {
    let store = Wrapper::new("Banana");
    let Wrapper { value } = store;

    println!("{:#?}", store);
    println!("Our value: {}", value);
}

#[derive(Debug)]
struct Wrapper<T> {
    value: T,
}

impl<T> Wrapper<T> {
    pub fn new(value: T) -> Self {
        Wrapper { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        assert_eq!(Wrapper::new(42).value, 42);
    }

    #[test]
    fn store_str_in_wrapper() {
        assert_eq!(Wrapper::new("Banana").value, "Banana");
    }
}
