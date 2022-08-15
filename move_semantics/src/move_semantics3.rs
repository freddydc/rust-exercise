// move_semantics3.rs

pub fn run_move_semantics3() {
    let coin = Vec::new();

    let mut coins = fill_vec(coin);

    println!("Coins has length {} content {:?}", coins.len(), coins);

    coins.push(48);

    println!("Coins has length {} content {:?}", coins.len(), coins);
}

fn fill_vec(mut vec: Vec<i32>) -> Vec<i32> {
    vec.push(12);
    vec.push(24);
    vec.push(36);

    vec
}
