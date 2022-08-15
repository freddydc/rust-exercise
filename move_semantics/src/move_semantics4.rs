// move_semantics4.rs

pub fn run_move_semantics4() {
    let mut cart = fill_vec();

    println!("Cart has length {} content {:?}", cart.len(), cart);

    cart.push("Jacket".to_string());

    println!("Cart has length {} content {:?}", cart.len(), cart);
}

// `fill_vec()` no longer takes `vec: Vec<String>` as argument
fn fill_vec() -> Vec<String> {
    let vec = vec!["Hat".to_string(), "Shoe".to_string()];
    vec
}
