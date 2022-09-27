// traits4.rs

pub fn run_traits4() {
    let avocado = Avocado { quantity: 200 };
    let banana = Banana { quantity: 400 };

    if compare_license_types(&avocado, &banana) {
        let total_quantity = avocado.quantity + banana.quantity;
        println!("Avocado and Banana quantity: {}", total_quantity);
    }
}

pub trait Licensed {
    fn licensing_info(&self) -> String {
        "Some information".to_string()
    }
}

struct Avocado {
    quantity: u32,
}

struct Banana {
    quantity: u32,
}

impl Licensed for Avocado {}
impl Licensed for Banana {}

fn compare_license_types(fruit_one: &impl Licensed, fruit_two: &impl Licensed) -> bool {
    fruit_one.licensing_info() == fruit_two.licensing_info()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compare_license_information() {
        let avocado = Avocado { quantity: 200 };
        let banana = Banana { quantity: 400 };

        assert!(compare_license_types(&avocado, &banana));
    }

    #[test]
    fn compare_license_information_backwards() {
        let avocado = Avocado { quantity: 200 };
        let banana = Banana { quantity: 400 };

        assert!(compare_license_types(&banana, &avocado));
    }
}
