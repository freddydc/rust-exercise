// iterators1.rs

pub fn run_iterators1() {
    let fruit_cart = vec!["banana", "cherry", "avocado", "peach", "grape"];
    let mut iterable_fruit_cart = fruit_cart.iter();

    let result = iterable_fruit_cart.next();
    println!("{:#?}", result);
}

#[cfg(test)]
mod tests {

    #[test]
    fn iterate_fruit_cart() {
        let fruit_cart = vec!["banana", "cherry", "avocado", "peach", "grape"];
        let mut iterable_fruit_cart = fruit_cart.iter();

        assert_eq!(iterable_fruit_cart.next(), Some(&"banana"));
        assert_eq!(iterable_fruit_cart.next(), Some(&"cherry"));
        assert_eq!(iterable_fruit_cart.next(), Some(&"avocado"));
        assert_eq!(iterable_fruit_cart.next(), Some(&"peach"));
        assert_eq!(iterable_fruit_cart.next(), Some(&"grape"));
        assert_eq!(iterable_fruit_cart.next(), None);
    }
}
