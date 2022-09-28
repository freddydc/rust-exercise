// traits5.rs

pub fn run_traits5() {
    let avocado = Avocado { reviews: 800 };
    let banana = Banana { reviews: 200 };

    if cart(&avocado) && cart(&banana) {
        println!("Avocado reviews: {}", avocado.reviews);
        println!("Banana reviews: {}", banana.reviews);
    }
}

fn cart(fruit: &(impl License + Summary)) -> bool {
    fruit.is_licensed() && fruit.is_summarized()
}

pub trait License {
    fn is_licensed(&self) -> bool {
        true
    }
}

pub trait Summary {
    fn is_summarized(&self) -> bool {
        true
    }
}

struct Avocado {
    reviews: u32,
}

struct Banana {
    reviews: u32,
}

impl License for Avocado {}
impl Summary for Avocado {}

impl License for Banana {}
impl Summary for Banana {}
