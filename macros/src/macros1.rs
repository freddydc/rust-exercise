// macros1.rs

macro_rules! log_message {
    () => {
        println!("Good Morning!");
    };
}

pub fn run_macros1() {
    log_message!();
}
