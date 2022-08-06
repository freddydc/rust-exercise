// functions3.rs

fn call_me(num: u16, emoji: char) {
    for i in 0..num {
        println!("{} Call number {}", emoji, i + 1);
    }
}

pub fn run_functions3() {
    call_me(3, '🐏');
}
