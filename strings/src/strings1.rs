// strings1.rs

pub fn run_strings1() {
    let answer = current_favorite_color();
    println!("My current favorite color is {}", answer);
}

fn current_favorite_color() -> String {
    "orange".to_string()
}
