// traits2.rs

pub fn run_traits2() {
    let cart = vec!["Avocado".to_string()].append_banana();
    println!("{:#?}", cart);
}

trait AppendBanana {
    fn append_banana(&mut self) -> Self;
}

impl AppendBanana for Vec<String> {
    fn append_banana(&mut self) -> Self {
        self.push("Banana".into());
        self.to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vector_pop_eq_banana() {
        let mut cart = vec![String::from("Avocado")].append_banana();
        assert_eq!(cart.pop().unwrap(), String::from("Banana"));
        assert_eq!(cart.pop().unwrap(), String::from("Avocado"));
    }
}
