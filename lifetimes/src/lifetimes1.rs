// lifetimes1.rs

// The Rust compiler needs to know how to check whether supplied references are
// valid, so that it can let the programmer know if a reference is at risk
// of going out of scope before it is used. Remember, references are borrows
// and do not own their own data. What if their owner goes out of scope?

pub fn run_lifetimes1() {
    let avocado = String::from("Avocado");
    let banana = "Banana";

    let result = longest(avocado.as_str(), banana);
    println!("The longest fruit is {}", result);
}

fn longest<'x>(fruit_one: &'x str, fruit_two: &'x str) -> &'x str {
    if fruit_one.len() > fruit_two.len() {
        fruit_one
    } else {
        fruit_two
    }
}
