// functions4.rs

// This store is having a sale where if the price is an even number, you get
// 10 Rust bucks off, but if it's an odd number, it's 3 Rust bucks off.
// (Don't worry about the function bodies themselves, we're only interested
// in the signatures for now. If anything, this is a good way to peek ahead
// to future exercises!)

pub fn run_functions4() {
    let original_price = 40;
    println!("Your sale price is {}", sale_price(original_price));
}

fn sale_price(price: u32) -> u32 {
    if is_even(price) {
        price - 10
    } else {
        price - 3
    }
}

fn is_even(num: u32) -> bool {
    num % 2 == 0
}
