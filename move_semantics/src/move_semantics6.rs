// move_semantics6.rs

pub fn run_move_semantics6() {
    let data = "Rust is great!".to_string();

    let answer = get_char(&data);
    println!("The char of {} is {:?}", data, answer);

    string_uppercase(data);
}

// Should not take ownership
fn get_char(data: &str) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();

    println!("{}", data);
}
