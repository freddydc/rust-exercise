// modules2.rs

use self::delicious_snacks::{fruit, veggie, BANANA, CARROT};

pub fn run_modules2() {
    println!("Favorite snacks: {} and {}", fruit, veggie);
    println!("Desired snacks: {} and {}", BANANA, CARROT);
}

mod delicious_snacks {
    pub use self::fruits::PEAR as fruit;
    pub use self::veggies::CUCUMBER as veggie;

    pub use self::fruits::BANANA;
    pub use self::veggies::CARROT;

    mod fruits {
        pub const PEAR: &str = "Pear";
        pub const BANANA: &str = "Banana";
    }

    mod veggies {
        pub const CUCUMBER: &str = "Cucumber";
        pub const CARROT: &str = "Carrot";
    }
}
