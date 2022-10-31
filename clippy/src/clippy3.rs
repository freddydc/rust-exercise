// clippy3.rs

use std::mem;

pub fn run_clippy3() {
    let fruit: Option<()> = None;

    if fruit.is_none() {
        println!("Oh no! Fruit not available");
    }

    let score = &[-10, -20, -30, -40, -50, -60];
    println!("The array is: {:#?}", score);

    let mut basket = vec![(1, 2), (3, 4)];
    basket.clear();
    println!("This vector is empty: {:?}", basket);

    let mut avocado = (10, 20);
    let mut banana = (30, 40);

    mem::swap(&mut banana, &mut avocado);

    println!("Avocado basket: {:#?}", avocado);
    println!("Banana basket: {:#?}", banana);
}
