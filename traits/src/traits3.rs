// traits3.rs

pub fn run_traits3() {
    let avocado = Avocado { price: 100 };
    let banana = Banana {
        price: "200".to_string(),
    };

    if avocado.licensing_info() == banana.licensing_info() {
        println!("Avocado price: {}", avocado.price);
        println!("Banana price: {:?}", banana.price);
    }
}

pub trait Licensed {
    fn licensing_info(&self) -> String {
        "Some information".to_string()
    }
}

struct Avocado {
    price: i32,
}

struct Banana {
    price: String,
}

impl Licensed for Avocado {}
impl Licensed for Banana {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_licensing_info_the_same() {
        let licensing_info = String::from("Some information");
        let avocado = Avocado { price: 100 };
        let banana = Banana {
            price: "200".to_string(),
        };
        assert_eq!(avocado.licensing_info(), licensing_info);
        assert_eq!(banana.licensing_info(), licensing_info);
    }
}
